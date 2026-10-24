//! Administrator ("elevated") support.
//!
//! Windows (UIPI) stops a normal process from moving or resizing windows of an elevated one, so
//! ChickenWM can only tile those when it runs elevated itself. This module provides:
//! - [`is_elevated`] / [`run_elevated`]: check the current token and relaunch a program through
//!   the UAC prompt.
//! - A scheduled task (`ChickenWM`, "highest privileges", on-demand) that lets the shell start
//!   elevated at sign-in *without* a prompt after a one-time approval.

use anyhow::{anyhow, bail, Context, Result};
use std::os::windows::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::Command;
use windows::core::PCWSTR;
use windows::Win32::Foundation::{CloseHandle, HANDLE};
use windows::Win32::Security::{GetTokenInformation, TokenElevation, TOKEN_ELEVATION, TOKEN_QUERY};
use windows::Win32::System::Threading::{
    GetCurrentProcess, GetExitCodeProcess, OpenProcessToken, WaitForSingleObject, INFINITE,
};
use windows::Win32::UI::Shell::{ShellExecuteExW, SEE_MASK_NOCLOSEPROCESS, SHELLEXECUTEINFOW};
use windows::Win32::UI::WindowsAndMessaging::SW_HIDE;

/// Name of the on-demand scheduled task that starts ChickenWM elevated.
pub const ADMIN_TASK_NAME: &str = "ChickenWM";

const CREATE_NO_WINDOW: u32 = 0x0800_0000;

fn wide(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}

/// Whether this process runs with an elevated (administrator) token.
pub fn is_elevated() -> bool {
    static ELEVATED: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *ELEVATED.get_or_init(|| unsafe {
        let mut token = HANDLE::default();
        if OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token).is_err() {
            return false;
        }
        let mut info = TOKEN_ELEVATION::default();
        let mut len = 0u32;
        let ok = GetTokenInformation(
            token,
            TokenElevation,
            Some(&mut info as *mut _ as *mut core::ffi::c_void),
            std::mem::size_of::<TOKEN_ELEVATION>() as u32,
            &mut len,
        )
        .is_ok();
        let _ = CloseHandle(token);
        ok && info.TokenIsElevated != 0
    })
}

fn quote_arg(arg: &str) -> String {
    if !arg.is_empty() && !arg.contains([' ', '\t', '"']) {
        arg.to_string()
    } else {
        format!("\"{}\"", arg.replace('"', "\\\""))
    }
}

/// Runs `exe` with `args` through the UAC prompt (`runas`). Returns the exit code when `wait`
/// is set (otherwise 0). Fails when the user declines the prompt.
pub fn run_elevated(exe: &Path, args: &[&str], wait: bool) -> Result<u32> {
    let file = wide(&exe.to_string_lossy());
    let params = wide(&args.iter().map(|a| quote_arg(a)).collect::<Vec<_>>().join(" "));
    let verb = wide("runas");
    let mut info = SHELLEXECUTEINFOW {
        cbSize: std::mem::size_of::<SHELLEXECUTEINFOW>() as u32,
        fMask: if wait { SEE_MASK_NOCLOSEPROCESS } else { 0 },
        lpVerb: PCWSTR(verb.as_ptr()),
        lpFile: PCWSTR(file.as_ptr()),
        lpParameters: PCWSTR(params.as_ptr()),
        nShow: SW_HIDE.0,
        ..Default::default()
    };
    unsafe {
        ShellExecuteExW(&mut info).map_err(|e| anyhow!("administrator request was not approved: {e}"))?;
        if !wait || info.hProcess.is_invalid() {
            return Ok(0);
        }
        WaitForSingleObject(info.hProcess, INFINITE);
        let mut code = 0u32;
        let _ = GetExitCodeProcess(info.hProcess, &mut code);
        let _ = CloseHandle(info.hProcess);
        Ok(code)
    }
}

fn schtasks(args: &[&str]) -> Result<std::process::Output> {
    Command::new("schtasks.exe")
        .args(args)
        .creation_flags(CREATE_NO_WINDOW)
        .output()
        .context("could not run schtasks.exe")
}

/// Whether the elevated-start scheduled task is registered.
pub fn admin_task_exists() -> bool {
    schtasks(&["/query", "/tn", ADMIN_TASK_NAME]).map(|o| o.status.success()).unwrap_or(false)
}

/// The program the elevated-start task runs, when the task is registered.
pub fn admin_task_command() -> Option<PathBuf> {
    let out = schtasks(&["/query", "/tn", ADMIN_TASK_NAME, "/xml"]).ok()?;
    if !out.status.success() {
        return None;
    }
    let xml = String::from_utf8_lossy(&out.stdout);
    let start = xml.find("<Command>")? + "<Command>".len();
    let end = start + xml[start..].find("</Command>")?;
    let cmd = xml[start..end].replace("&lt;", "<").replace("&gt;", ">").replace("&amp;", "&");
    Some(PathBuf::from(cmd.trim().trim_matches('"')))
}

/// Whether the task is registered *and* starts `exe`. A task left over from another install
/// location still "starts" through `schtasks /run`, but runs nothing.
pub fn admin_task_runs(exe: &Path) -> bool {
    let Some(cmd) = admin_task_command() else {
        return false;
    };
    match (std::fs::canonicalize(&cmd), std::fs::canonicalize(exe)) {
        (Ok(a), Ok(b)) => a.to_string_lossy().eq_ignore_ascii_case(&b.to_string_lossy()),
        _ => false,
    }
}

/// Starts the scheduled task (an elevated ChickenWM, no prompt). True when Windows accepted the
/// request — not proof the program started; see [`admin_task_runs`].
pub fn start_admin_task() -> bool {
    schtasks(&["/run", "/tn", ADMIN_TASK_NAME]).map(|o| o.status.success()).unwrap_or(false)
}

fn xml_escape(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;")
}

/// Registers the elevated-start task for the current user. Needs an elevated caller.
pub fn install_admin_task(exe: &Path) -> Result<()> {
    if !is_elevated() {
        bail!("administrator rights are required to register the task");
    }
    let user = format!(
        "{}\\{}",
        std::env::var("USERDOMAIN").unwrap_or_default(),
        std::env::var("USERNAME").unwrap_or_default()
    );
    // No trigger: the task only ever runs on demand, so a shell-started (unelevated) rooster can
    // hand over to it without ever creating a second instance at sign-in.
    let xml = format!(
        r#"<?xml version="1.0" encoding="UTF-16"?>
<Task version="1.2" xmlns="http://schemas.microsoft.com/windows/2004/02/mit/task">
  <RegistrationInfo><Description>Starts ChickenWM with administrator rights, without a UAC prompt.</Description></RegistrationInfo>
  <Principals>
    <Principal id="Author">
      <UserId>{user}</UserId>
      <LogonType>InteractiveToken</LogonType>
      <RunLevel>HighestAvailable</RunLevel>
    </Principal>
  </Principals>
  <Settings>
    <MultipleInstancesPolicy>IgnoreNew</MultipleInstancesPolicy>
    <DisallowStartIfOnBatteries>false</DisallowStartIfOnBatteries>
    <StopIfGoingOnBatteries>false</StopIfGoingOnBatteries>
    <AllowHardTerminate>true</AllowHardTerminate>
    <AllowStartOnDemand>true</AllowStartOnDemand>
    <Enabled>true</Enabled>
    <ExecutionTimeLimit>PT0S</ExecutionTimeLimit>
    <Priority>4</Priority>
  </Settings>
  <Actions Context="Author">
    <Exec><Command>{exe}</Command></Exec>
  </Actions>
</Task>
"#,
        user = xml_escape(&user),
        exe = xml_escape(&exe.to_string_lossy()),
    );
    let mut bytes = vec![0xFF, 0xFE];
    for unit in xml.encode_utf16() {
        bytes.extend_from_slice(&unit.to_le_bytes());
    }
    let path = std::env::temp_dir().join("chickenwm-admin-task.xml");
    std::fs::write(&path, bytes)?;
    let out = schtasks(&["/create", "/tn", ADMIN_TASK_NAME, "/xml", &path.to_string_lossy(), "/f"]);
    let _ = std::fs::remove_file(&path);
    let out = out?;
    if !out.status.success() {
        bail!("schtasks failed: {}", String::from_utf8_lossy(&out.stderr).trim());
    }
    Ok(())
}

/// Removes the elevated-start task (a missing task is fine). Needs an elevated caller.
pub fn remove_admin_task() -> Result<()> {
    if !admin_task_exists() {
        return Ok(());
    }
    if !is_elevated() {
        bail!("administrator rights are required to remove the task");
    }
    let out = schtasks(&["/delete", "/tn", ADMIN_TASK_NAME, "/f"])?;
    if !out.status.success() {
        bail!("schtasks failed: {}", String::from_utf8_lossy(&out.stderr).trim());
    }
    Ok(())
}

/// Returns `true` if `pid` runs in the same interactive Windows session as the current process
/// and has an unelevated (standard user / medium integrity) token.
fn is_process_unelevated_in_current_session(pid: u32) -> bool {
    if pid == 0 || pid == std::process::id() {
        return false;
    }
    unsafe {
        use windows::Win32::System::Threading::{
            OpenProcess, OpenProcessToken, PROCESS_QUERY_LIMITED_INFORMATION,
        };
        let Ok(proc_handle) = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, pid) else {
            return false;
        };
        let mut token = HANDLE::default();
        let opened = OpenProcessToken(proc_handle, TOKEN_QUERY, &mut token);
        let _ = CloseHandle(proc_handle);
        if opened.is_err() {
            return false;
        }
        let mut elevation = TOKEN_ELEVATION::default();
        let mut size = 0u32;
        let ok = GetTokenInformation(
            token,
            TokenElevation,
            Some(&mut elevation as *mut _ as *mut _),
            std::mem::size_of::<TOKEN_ELEVATION>() as u32,
            &mut size,
        );
        let _ = CloseHandle(token);
        ok.is_ok() && elevation.TokenIsElevated == 0
    }
}

/// Finds the process ID of an interactive unelevated user shell/host process in the current session
/// (prefers `explorer.exe`, falling back to `sihost.exe` or `ctfmon.exe` when `explorer.exe` is suppressed).
/// Never returns `chickenwm-rooster.exe` or an elevated process.
pub fn find_shell_process_id() -> Option<u32> {
    unsafe {
        use windows::Win32::System::Diagnostics::ToolHelp::*;
        let snap = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0).ok()?;
        let mut entry = PROCESSENTRY32W {
            dwSize: std::mem::size_of::<PROCESSENTRY32W>() as u32,
            ..Default::default()
        };
        let mut explorer_pid = None;
        let mut fallback_pid = None;
        if Process32FirstW(snap, &mut entry).is_ok() {
            loop {
                let name = String::from_utf16_lossy(&entry.szExeFile)
                    .trim_matches('\0')
                    .to_ascii_lowercase();
                let pid = entry.th32ProcessID;
                if name == "explorer.exe" && is_process_unelevated_in_current_session(pid) {
                    explorer_pid = Some(pid);
                    break;
                } else if fallback_pid.is_none()
                    && (name == "sihost.exe" || name == "ctfmon.exe")
                    && is_process_unelevated_in_current_session(pid)
                {
                    fallback_pid = Some(pid);
                }
                if Process32NextW(snap, &mut entry).is_err() {
                    break;
                }
            }
        }
        let _ = CloseHandle(snap);
        explorer_pid.or(fallback_pid)
    }
}

/// Spawns a process as a child of `parent_pid` (typically `explorer.exe` or `sihost.exe`), forcing it
/// to inherit the parent process's unelevated (standard user / medium integrity) security token
/// and a clean console state (`CREATE_NEW_CONSOLE` so GUI/CUI apps never fail with `0xC0000142`).
fn spawn_as_child_of_process(parent_pid: u32, target: &Path, args: Option<&str>) -> Result<()> {
    unsafe {
        use windows::Win32::System::Threading::*;

        let h_parent = OpenProcess(PROCESS_CREATE_PROCESS, false, parent_pid)
            .map_err(|e| anyhow!("OpenProcess(PROCESS_CREATE_PROCESS) failed: {e}"))?;

        let mut size = 0usize;
        let _ = InitializeProcThreadAttributeList(LPPROC_THREAD_ATTRIBUTE_LIST(std::ptr::null_mut()), 1, 0, &mut size);
        let mut attr_memory = vec![0u8; size];
        let attr_list = LPPROC_THREAD_ATTRIBUTE_LIST(attr_memory.as_mut_ptr() as *mut _);

        if InitializeProcThreadAttributeList(attr_list, 1, 0, &mut size).is_err() {
            let _ = CloseHandle(h_parent);
            bail!("InitializeProcThreadAttributeList failed");
        }

        const PROC_THREAD_ATTRIBUTE_PARENT_PROCESS: usize = 0x00020000;
        let mut parent_handle_val = h_parent;
        let update_ok = UpdateProcThreadAttribute(
            attr_list,
            0,
            PROC_THREAD_ATTRIBUTE_PARENT_PROCESS,
            Some(&mut parent_handle_val as *mut _ as *const _),
            std::mem::size_of::<HANDLE>(),
            None,
            None,
        );

        if update_ok.is_err() {
            DeleteProcThreadAttributeList(attr_list);
            let _ = CloseHandle(h_parent);
            bail!("UpdateProcThreadAttribute failed");
        }

        let mut siex = STARTUPINFOEXW::default();
        siex.StartupInfo.cb = std::mem::size_of::<STARTUPINFOEXW>() as u32;
        siex.lpAttributeList = attr_list;

        let mut pi = PROCESS_INFORMATION::default();

        let is_exe = target.extension().map_or(false, |ext| {
            ext.eq_ignore_ascii_case("exe") || ext.eq_ignore_ascii_case("cmd") || ext.eq_ignore_ascii_case("bat")
        });

        // CRITICAL: Include `CREATE_NEW_CONSOLE` when reparenting `.exe`/`.cmd`/`.bat` with
        // `PROC_THREAD_ATTRIBUTE_PARENT_PROCESS`. Without `CREATE_NEW_CONSOLE`, Windows copies the
        // caller's `ConsoleHandle` into the child's `ProcessParameters`, which does not belong to
        // `parent_pid` (or is stale), causing `kernelbase.dll` in the child to abort with
        // `0xC0000142` (`STATUS_DLL_INIT_FAILED`). For GUI subsystem executables, `CREATE_NEW_CONSOLE`
        // simply clears `ConsoleHandle` without creating a console window.
        let target_str = target.to_string_lossy();
        let (cmd_str, creation_flags) = if is_exe && args.is_none() {
            (
                format!("\"{}\"", target_str),
                EXTENDED_STARTUPINFO_PRESENT | CREATE_NEW_CONSOLE,
            )
        } else if is_exe && args.is_some() {
            (
                format!("\"{}\" {}", target_str, args.unwrap()),
                EXTENDED_STARTUPINFO_PRESENT | CREATE_NEW_CONSOLE,
            )
        } else {
            (
                format!("cmd.exe /c start \"\" \"{}\" {}", target_str, args.unwrap_or("")),
                EXTENDED_STARTUPINFO_PRESENT | CREATE_NO_WINDOW,
            )
        };

        let mut cmd_w: Vec<u16> = cmd_str.encode_utf16().chain(std::iter::once(0)).collect();
        let work_dir = target.parent().filter(|p| p.is_dir()).map(|p| wide(&p.to_string_lossy()));

        let created = CreateProcessW(
            None,
            windows::core::PWSTR(cmd_w.as_mut_ptr()),
            None,
            None,
            false,
            creation_flags,
            None,
            work_dir.as_ref().map_or(PCWSTR::null(), |d| PCWSTR(d.as_ptr())),
            &siex.StartupInfo,
            &mut pi,
        );

        DeleteProcThreadAttributeList(attr_list);
        let _ = CloseHandle(h_parent);

        if created.is_err() {
            bail!("CreateProcessW failed: {}", std::io::Error::last_os_error());
        }

        let _ = CloseHandle(pi.hProcess);
        let _ = CloseHandle(pi.hThread);
        Ok(())
    }
}

/// Spawns an `.exe` directly with `CREATE_NEW_CONSOLE` and clean null stdio handles so it never
/// inherits a broken console handle (`0xC0000142`).
fn spawn_clean_executable(target: &Path, args: Option<&str>, work_dir: Option<&Path>) -> Result<()> {
    use std::os::windows::process::CommandExt;
    use std::process::Stdio;
    use windows::Win32::System::Threading::CREATE_NEW_CONSOLE;

    let is_exe = target
        .extension()
        .is_some_and(|ext| ext.eq_ignore_ascii_case("exe"));
    if !is_exe {
        bail!("not an executable");
    }
    let mut cmd = std::process::Command::new(target);
    if let Some(a) = args.map(str::trim).filter(|s| !s.is_empty()) {
        for part in a.split_whitespace() {
            cmd.arg(part);
        }
    }
    if let Some(d) = work_dir.or_else(|| target.parent().filter(|p| p.is_dir())) {
        cmd.current_dir(d);
    }
    cmd.stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .creation_flags(CREATE_NEW_CONSOLE.0);
    cmd.spawn()?;
    Ok(())
}

/// Opens an app, document, folder, shortcut, or URI.
/// If `verb == Some("runas")`, launches elevated via UAC prompt.
/// If `verb != Some("runas")` and the current process is running elevated as Administrator,
/// this automatically de-elevates by spawning the child under an unelevated session process token,
/// ensuring the user's applications never unexpectedly inherit Administrator privileges!
pub fn shell_open_unelevated(target: &Path, args: Option<&str>, verb: Option<&str>) {
    use windows::Win32::UI::Shell::ShellExecuteW;
    use windows::Win32::UI::WindowsAndMessaging::SW_SHOWNORMAL;

    let is_admin = verb.map_or(false, |v| v.eq_ignore_ascii_case("runas"));
    if is_admin {
        let args_vec: Vec<&str> = args.into_iter().collect();
        let _ = run_elevated(target, &args_vec, false);
        return;
    }

    if !is_elevated() {
        if spawn_clean_executable(target, args, None).is_ok() {
            return;
        }
        let file = wide(&target.to_string_lossy());
        let params = args.map(wide);
        let verb_w = wide("open");
        let dir = target.parent().filter(|p| p.is_dir()).map(|p| wide(&p.to_string_lossy()));
        unsafe {
            let _ = ShellExecuteW(
                None,
                PCWSTR(verb_w.as_ptr()),
                PCWSTR(file.as_ptr()),
                params.as_ref().map_or(PCWSTR::null(), |p| PCWSTR(p.as_ptr())),
                dir.as_ref().map_or(PCWSTR::null(), |d| PCWSTR(d.as_ptr())),
                SW_SHOWNORMAL,
            );
        }
        return;
    }

    // Process is elevated. De-elevate through an unelevated shell/session process if available.
    if let Some(shell_pid) = find_shell_process_id() {
        if spawn_as_child_of_process(shell_pid, target, args).is_ok() {
            return;
        }
    }

    // Backstop fallback if no unelevated parent could be opened
    if spawn_clean_executable(target, args, None).is_ok() {
        return;
    }
    let file = wide(&target.to_string_lossy());
    let params = args.map(wide);
    let verb_w = wide("open");
    let dir = target.parent().filter(|p| p.is_dir()).map(|p| wide(&p.to_string_lossy()));
    unsafe {
        let _ = ShellExecuteW(
            None,
            PCWSTR(verb_w.as_ptr()),
            PCWSTR(file.as_ptr()),
            params.as_ref().map_or(PCWSTR::null(), |p| PCWSTR(p.as_ptr())),
            dir.as_ref().map_or(PCWSTR::null(), |d| PCWSTR(d.as_ptr())),
            SW_SHOWNORMAL,
        );
    }
}

/// Spawns a command string as a standard user process when running elevated.
pub fn spawn_unelevated(cmd_line: &str, work_dir: Option<&Path>) -> Result<()> {
    use std::os::windows::process::CommandExt;
    use std::process::Stdio;
    use windows::Win32::System::Threading::CREATE_NEW_CONSOLE;

    if !is_elevated() {
        let mut parts = cmd_line.split_whitespace();
        if let Some(prog) = parts.next() {
            let mut cmd = std::process::Command::new(prog);
            cmd.args(parts)
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .creation_flags(CREATE_NEW_CONSOLE.0);
            if let Some(d) = work_dir {
                cmd.current_dir(d);
            }
            cmd.spawn()?;
            return Ok(());
        }
        bail!("empty command");
    }

    if let Some(shell_pid) = find_shell_process_id() {
        let prog = cmd_line.split_whitespace().next().unwrap_or(cmd_line);
        let path = Path::new(prog);
        let args = cmd_line.strip_prefix(prog).map(|s| s.trim()).filter(|s| !s.is_empty());
        if spawn_as_child_of_process(shell_pid, path, args).is_ok() {
            return Ok(());
        }
    }

    let mut parts = cmd_line.split_whitespace();
    if let Some(prog) = parts.next() {
        let mut cmd = std::process::Command::new(prog);
        cmd.args(parts)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .creation_flags(CREATE_NEW_CONSOLE.0);
        if let Some(d) = work_dir {
            cmd.current_dir(d);
        }
        cmd.spawn()?;
        return Ok(());
    }
    bail!("empty command")
}
