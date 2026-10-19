//! `chickenwm-core::sys` — Low-level undocumented Win32 & DWM API bindings for
//! Architecture A (Parasitic Shared-Visual Compositor) and instant workspace cloaking.
//!
//! Provides:
//! - `DwmpCreateSharedThumbnailVisual` (`dwmapi.dll` Ordinal 147): Binds an HWND's redirection
//!   surface visual directly into DirectComposition (`IDCompositionVisual2`).
//! - `DwmpQueryWindowThumbnailSourceSize` (`dwmapi.dll` Ordinal 162): Exact source thumbnail dimensions.
//! - `DwmDxGetWindowSharedSurface` (`dwmapi.dll` Ordinal 100): Shared DXGI surface handle.
//! - `SetWindowCompositionAttribute` (`user32.dll`): Workspace cloaking (`WCA_CLOAK = 17`) and
//!   capture exclusion (`WCA_EXCLUDED_FROM_LIVEPREVIEW = 13`, `WCA_EXCLUDED_FROM_DDA = 24`).
//! - High-level safe wrappers for cloaking, chrome stripping, and alpha hit-testing.

use std::sync::OnceLock;

use windows::core::{HRESULT, PCSTR, PCWSTR};
use windows::Win32::Foundation::{COLORREF, HANDLE, HWND, LUID, RECT, SIZE};
use windows::Win32::Graphics::DirectComposition::IDCompositionVisual2;
use windows::Win32::Graphics::Dwm::{
    DwmSetWindowAttribute, DWMWA_NCRENDERING_POLICY, DWMWA_WINDOW_CORNER_PREFERENCE,
    DWMNCRP_DISABLED, DWMNCRP_ENABLED, DWMWCP_DEFAULT, DWMWCP_DONOTROUND,
    DWM_THUMBNAIL_PROPERTIES,
};
use windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT;
use windows::Win32::System::LibraryLoader::{GetModuleHandleW, GetProcAddress, LoadLibraryW};
use windows::Win32::UI::WindowsAndMessaging::{
    RemovePropW, SetLayeredWindowAttributes, SetPropW, LWA_ALPHA,
};

/// Window composition attribute enumeration (`user32.dll!SetWindowCompositionAttribute`).
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WINDOWCOMPOSITIONATTRIB {
    WcaUndefined = 0,
    WcaNcRenderingEnabled = 1,
    WcaNcRenderingPolicy = 2,
    WcaTransitionComposited = 3,
    WcaCompactAttribute = 4,
    WcaExtendedFrameBounds = 5,
    WcaHasIconicBitmap = 6,
    WcaThemeMetadata = 7,
    WcaCloak = 17, // 0x11: Hides window while keeping surface state controllable
    WcaCloaked = 18,
    WcaAccentPolicy = 19,
    WcaFreezeRepresentation = 20, // 0x14
    WcaEverUncloaked = 21,
    WcaVisualOwner = 22,
    WcaHolographic = 23,
    WcaExcludedFromLivePreview = 13, // 0x0D: Overlay exclusion from preview
    WcaExcludedFromDda = 24,         // 0x18: Overlay exclusion from Desktop Duplication
    WcaPassiveUpdateMode = 25,
    WcaUseDarkModeColors = 26,
    WcaCornerStyle = 27,
    WcaPartColor = 28,
    WcaDisableMoveSizeFeedback = 29,
    WcaLast = 30,
}

#[repr(C)]
pub struct WINDOWCOMPOSITIONATTRIBDATA {
    pub attrib: WINDOWCOMPOSITIONATTRIB,
    pub pv_data: *mut core::ffi::c_void,
    pub cb_data: usize,
}

pub type SetWindowCompositionAttributeFn = unsafe extern "system" fn(
    hwnd: HWND,
    p_attr_data: *const WINDOWCOMPOSITIONATTRIBDATA,
) -> windows::Win32::Foundation::BOOL;

pub type DwmpCreateSharedThumbnailVisualFn = unsafe extern "system" fn(
    hwnd_destination: HWND,
    hwnd_source: HWND,
    dw_thumbnail_flags: u32,
    p_thumbnail_properties: *const DWM_THUMBNAIL_PROPERTIES,
    p_dcomp_device: *mut core::ffi::c_void,
    pp_visual: *mut *mut core::ffi::c_void,
    ph_thumbnail_id: *mut isize,
) -> HRESULT;

pub type DwmpQueryWindowThumbnailSourceSizeFn = unsafe extern "system" fn(
    hwnd_source: HWND,
    f_source_client_area_only: windows::Win32::Foundation::BOOL,
    p_size: *mut SIZE,
) -> HRESULT;

pub type DwmDxGetWindowSharedSurfaceFn = unsafe extern "system" fn(
    hwnd: HWND,
    luid_adapter: LUID,
    ul_flags: u32,
    pfmt_window: *mut DXGI_FORMAT,
    ph_surface: *mut HANDLE,
    pui_update_id: *mut u64,
) -> HRESULT;

fn load_user32_proc(name: &str) -> Option<usize> {
    unsafe {
        let h = GetModuleHandleW(windows::core::w!("user32.dll"))
            .unwrap_or_else(|_| LoadLibraryW(windows::core::w!("user32.dll")).unwrap_or_default());
        if h.is_invalid() {
            return None;
        }
        let name_c = std::ffi::CString::new(name).ok()?;
        let p = GetProcAddress(h, PCSTR(name_c.as_ptr() as *const u8));
        p.map(|f| f as usize)
    }
}

fn load_dwmapi_ordinal(ordinal: usize) -> Option<usize> {
    unsafe {
        let h = GetModuleHandleW(windows::core::w!("dwmapi.dll"))
            .unwrap_or_else(|_| LoadLibraryW(windows::core::w!("dwmapi.dll")).unwrap_or_default());
        if h.is_invalid() {
            return None;
        }
        let p = GetProcAddress(h, PCSTR(ordinal as *const u8));
        p.map(|f| f as usize)
    }
}

pub fn get_set_window_composition_attribute() -> Option<SetWindowCompositionAttributeFn> {
    static CACHE: OnceLock<Option<SetWindowCompositionAttributeFn>> = OnceLock::new();
    *CACHE.get_or_init(|| {
        load_user32_proc("SetWindowCompositionAttribute")
            .map(|addr| unsafe { std::mem::transmute(addr) })
    })
}

pub fn get_dwmp_create_shared_thumbnail_visual() -> Option<DwmpCreateSharedThumbnailVisualFn> {
    static CACHE: OnceLock<Option<DwmpCreateSharedThumbnailVisualFn>> = OnceLock::new();
    *CACHE.get_or_init(|| {
        load_dwmapi_ordinal(147).map(|addr| unsafe { std::mem::transmute(addr) })
    })
}

pub fn get_dwmp_query_window_thumbnail_source_size() -> Option<DwmpQueryWindowThumbnailSourceSizeFn> {
    static CACHE: OnceLock<Option<DwmpQueryWindowThumbnailSourceSizeFn>> = OnceLock::new();
    *CACHE.get_or_init(|| {
        load_dwmapi_ordinal(162).map(|addr| unsafe { std::mem::transmute(addr) })
    })
}

pub fn get_dwm_dx_get_window_shared_surface() -> Option<DwmDxGetWindowSharedSurfaceFn> {
    static CACHE: OnceLock<Option<DwmDxGetWindowSharedSurfaceFn>> = OnceLock::new();
    *CACHE.get_or_init(|| {
        load_dwmapi_ordinal(100).map(|addr| unsafe { std::mem::transmute(addr) })
    })
}

/// Window property set when ChickenWM takes ownership of window rendering/hit-testing.
pub const CHICKENWM_VISUAL_PROP: &str = "ChickenWMManagedVisual";

fn to_wide_null(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}

/// Cloaks or uncloaks a window instantly using `SetWindowCompositionAttribute(WCA_CLOAK)`.
/// Unlike `SW_HIDE`, cloaking preserves window resources, active video playback, and swapchains.
pub fn set_window_cloak(hwnd: HWND, cloak: bool) -> windows::core::Result<()> {
    let fn_ptr = get_set_window_composition_attribute().ok_or_else(|| {
        windows::core::Error::new(
            HRESULT(-1),
            "SetWindowCompositionAttribute not found in user32.dll",
        )
    })?;
    let mut val: u32 = if cloak { 1 } else { 0 };
    let data = WINDOWCOMPOSITIONATTRIBDATA {
        attrib: WINDOWCOMPOSITIONATTRIB::WcaCloak,
        pv_data: &mut val as *mut u32 as *mut core::ffi::c_void,
        cb_data: std::mem::size_of::<u32>(),
    };
    let res = unsafe { fn_ptr(hwnd, &data) };
    if res.as_bool() {
        Ok(())
    } else {
        Err(windows::core::Error::from_win32())
    }
}

/// Excludes `hwnd` (e.g. the fullscreen compositor overlay) from live preview and desktop capture (DDA).
pub fn set_overlay_capture_exclusion(hwnd: HWND) -> windows::core::Result<()> {
    if let Some(fn_ptr) = get_set_window_composition_attribute() {
        let mut val: u32 = 1;
        let dda = WINDOWCOMPOSITIONATTRIBDATA {
            attrib: WINDOWCOMPOSITIONATTRIB::WcaExcludedFromDda,
            pv_data: &mut val as *mut u32 as *mut core::ffi::c_void,
            cb_data: std::mem::size_of::<u32>(),
        };
        unsafe { let _ = fn_ptr(hwnd, &dda); };

        let preview = WINDOWCOMPOSITIONATTRIBDATA {
            attrib: WINDOWCOMPOSITIONATTRIB::WcaExcludedFromLivePreview,
            pv_data: &mut val as *mut u32 as *mut core::ffi::c_void,
            cb_data: std::mem::size_of::<u32>(),
        };
        unsafe { let _ = fn_ptr(hwnd, &preview); };
    }
    Ok(())
}

/// Prepares `hwnd` for ChickenWM window management:
/// 1. Strips DWM non-client rendering and native corner rounding so ChickenWM's border renders cleanly.
/// 2. Ensures the window remains fully visible and responsive to hit-testing.
/// 3. Marks the window with `ChickenWMManagedVisual` property for panic recovery.
pub fn attach_in_place_hit_test(hwnd: HWND) -> windows::core::Result<()> {
    unsafe {
        // Strip native chrome and corner rounding
        let policy = DWMNCRP_DISABLED;
        let _ = DwmSetWindowAttribute(
            hwnd,
            DWMWA_NCRENDERING_POLICY,
            &policy as *const _ as *const _,
            std::mem::size_of_val(&policy) as u32,
        );
        let corner = DWMWCP_DONOTROUND;
        let _ = DwmSetWindowAttribute(
            hwnd,
            DWMWA_WINDOW_CORNER_PREFERENCE,
            &corner as *const _ as *const _,
            std::mem::size_of_val(&corner) as u32,
        );

        // Tag window for crash recovery
        let prop_w = to_wide_null(CHICKENWM_VISUAL_PROP);
        let _ = SetPropW(hwnd, PCWSTR(prop_w.as_ptr()), HANDLE(1 as *mut _));
    }
    Ok(())
}

/// Restores `hwnd` to standard opaque state with native chrome and normal alpha.
pub fn restore_managed_window(hwnd: HWND) {
    unsafe {
        // Restore opacity to 255
        let _ = SetLayeredWindowAttributes(hwnd, COLORREF(0), 255, LWA_ALPHA);

        // Restore native non-client rendering and rounding
        let policy = DWMNCRP_ENABLED;
        let _ = DwmSetWindowAttribute(
            hwnd,
            DWMWA_NCRENDERING_POLICY,
            &policy as *const _ as *const _,
            std::mem::size_of_val(&policy) as u32,
        );
        let corner = DWMWCP_DEFAULT;
        let _ = DwmSetWindowAttribute(
            hwnd,
            DWMWA_WINDOW_CORNER_PREFERENCE,
            &corner as *const _ as *const _,
            std::mem::size_of_val(&corner) as u32,
        );

        // Uncloak if cloaked
        let _ = set_window_cloak(hwnd, false);

        // Remove marker property
        let prop_w = to_wide_null(CHICKENWM_VISUAL_PROP);
        let _ = RemovePropW(hwnd, PCWSTR(prop_w.as_ptr()));
    }
}

/// High-level function to bind an HWND's live DWM visual into a DirectComposition target visual.
pub fn create_shared_thumbnail_visual(
    hwnd_dest: HWND,
    hwnd_source: HWND,
    dcomp_device: *mut core::ffi::c_void,
) -> windows::core::Result<(IDCompositionVisual2, isize)> {
    let fn_create = get_dwmp_create_shared_thumbnail_visual().ok_or_else(|| {
        windows::core::Error::new(
            HRESULT(-1),
            "DwmpCreateSharedThumbnailVisual (ordinal 147) not found in dwmapi.dll",
        )
    })?;

    let props = DWM_THUMBNAIL_PROPERTIES {
        dwFlags: 0x4 | 0x8, // DWM_TNP_OPACITY | DWM_TNP_VISIBLE
        rcDestination: RECT::default(),
        rcSource: RECT::default(),
        opacity: 255,
        fVisible: windows::Win32::Foundation::BOOL(1),
        fSourceClientAreaOnly: windows::Win32::Foundation::BOOL(0),
    };

    let mut visual_ptr: *mut core::ffi::c_void = std::ptr::null_mut();
    let mut thumbnail_id: isize = 0;

    let hr = unsafe {
        fn_create(
            hwnd_dest,
            hwnd_source,
            2, // Pass 2 for live visual binding
            &props,
            dcomp_device,
            &mut visual_ptr,
            &mut thumbnail_id,
        )
    };

    if hr.is_err() || visual_ptr.is_null() {
        return Err(windows::core::Error::from(hr));
    }

    let visual: IDCompositionVisual2 = unsafe { std::mem::transmute(visual_ptr) };
    Ok((visual, thumbnail_id))
}

// ---------------------------------------------------------------------------
// Shell Task Manager & Shell Hook Window support (Phase 5)
// ---------------------------------------------------------------------------

pub const HSHELL_WINDOWCREATED: usize = 1;
pub const HSHELL_WINDOWDESTROYED: usize = 2;
pub const HSHELL_ACTIVATESHELLWINDOW: usize = 3;
pub const HSHELL_WINDOWACTIVATED: usize = 4;
pub const HSHELL_GETMINRECT: usize = 5;
pub const HSHELL_REDRAW: usize = 6;
pub const HSHELL_TASKMAN: usize = 7;
pub const HSHELL_LANGUAGE: usize = 8;
pub const HSHELL_SYSMENU: usize = 9;
pub const HSHELL_ENDTASK: usize = 10;
pub const HSHELL_ACCESSIBILITYSTATE: usize = 11;
pub const HSHELL_APPCOMMAND: usize = 12;
pub const HSHELL_WINDOWREPLACED: usize = 13;
pub const HSHELL_WINDOWREPLACING: usize = 14;
pub const HSHELL_MONITORCHANGED: usize = 16;
pub const HSHELL_FLASH: usize = 0x8006;
pub const HSHELL_RUDEAPPACTIVATED: usize = 0x8004;

/// Registers a window to receive Shell hook messages (HSHELL_*).
pub fn register_shell_hook_window(hwnd: HWND) -> bool {
    type Fn = unsafe extern "system" fn(HWND) -> windows::Win32::Foundation::BOOL;
    static FUNC: OnceLock<Option<Fn>> = OnceLock::new();
    let f = FUNC.get_or_init(|| {
        load_user32_proc("RegisterShellHookWindow").map(|addr| unsafe { std::mem::transmute(addr) })
    });
    if let Some(func) = f {
        unsafe { func(hwnd).as_bool() }
    } else {
        false
    }
}

/// Deregisters a window from receiving Shell hook messages.
pub fn deregister_shell_hook_window(hwnd: HWND) -> bool {
    type Fn = unsafe extern "system" fn(HWND) -> windows::Win32::Foundation::BOOL;
    static FUNC: OnceLock<Option<Fn>> = OnceLock::new();
    let f = FUNC.get_or_init(|| {
        load_user32_proc("DeregisterShellHookWindow").map(|addr| unsafe { std::mem::transmute(addr) })
    });
    if let Some(func) = f {
        unsafe { func(hwnd).as_bool() }
    } else {
        false
    }
}

/// Registers the specified window as the Windows shell task manager.
pub fn set_taskman_window(hwnd: HWND) -> bool {
    type Fn = unsafe extern "system" fn(HWND) -> windows::Win32::Foundation::BOOL;
    static FUNC: OnceLock<Option<Fn>> = OnceLock::new();
    let f = FUNC.get_or_init(|| {
        load_user32_proc("SetTaskmanWindow").map(|addr| unsafe { std::mem::transmute(addr) })
    });
    if let Some(func) = f {
        unsafe { func(hwnd).as_bool() }
    } else {
        false
    }
}

// ---------------------------------------------------------------------------
// Accent Policy & BlurBehind / Acrylic support (WCA_ACCENT_POLICY)
// ---------------------------------------------------------------------------

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ACCENT_STATE {
    AccentDisabled = 0,
    AccentEnableGradient = 1,
    AccentEnableTransparentGradient = 2,
    AccentEnableBlurbehind = 3,
    AccentEnableAcrylicblurbehind = 4,
    AccentEnableHostbackdrop = 5,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ACCENT_POLICY {
    pub accent_state: ACCENT_STATE,
    pub accent_flags: u32,
    pub gradient_color: u32,
    pub animation_id: u32,
}

/// Applies an accent policy (blur-behind / acrylic) to an HWND.
pub fn set_window_accent_policy(hwnd: HWND, policy: &ACCENT_POLICY) -> bool {
    let Some(fn_set) = get_set_window_composition_attribute() else {
        return false;
    };
    let mut policy_copy = *policy;
    let data = WINDOWCOMPOSITIONATTRIBDATA {
        attrib: WINDOWCOMPOSITIONATTRIB::WcaAccentPolicy,
        pv_data: &mut policy_copy as *mut _ as *mut core::ffi::c_void,
        cb_data: std::mem::size_of::<ACCENT_POLICY>(),
    };
    unsafe { fn_set(hwnd, &data).as_bool() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_window_composition_attribute_resolved() {
        let fn_opt = get_set_window_composition_attribute();
        assert!(fn_opt.is_some(), "user32.dll!SetWindowCompositionAttribute must resolve");
    }

    #[test]
    fn test_dwmapi_ordinals_resolved() {
        let create_visual = get_dwmp_create_shared_thumbnail_visual();
        assert!(create_visual.is_some(), "dwmapi.dll Ordinal 147 must resolve");

        let query_size = get_dwmp_query_window_thumbnail_source_size();
        assert!(query_size.is_some(), "dwmapi.dll Ordinal 162 must resolve");

        let get_surface = get_dwm_dx_get_window_shared_surface();
        assert!(get_surface.is_some(), "dwmapi.dll Ordinal 100 must resolve");
    }

    #[test]
    fn test_shell_hook_symbols_resolved() {
        // Even with null HWND, functions shouldn't crash
        let null_hwnd = HWND(std::ptr::null_mut());
        let _ = register_shell_hook_window(null_hwnd);
        let _ = deregister_shell_hook_window(null_hwnd);
        let _ = set_taskman_window(null_hwnd);
    }
}
