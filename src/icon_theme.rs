//! Icon themes.
//!
//! An icon theme is a folder of PNGs under `~/.config/chickenwm/icons/<theme>/`. Modules ask for
//! an icon by [`IconRequest`]; the theme answers with a PNG path, or `None` to fall back to the
//! Windows shell icon. Lookups go from most to least specific:
//!
//! | request                    | files tried                                        |
//! |----------------------------|----------------------------------------------------|
//! | folder                     | `folder.png`                                       |
//! | special folder (Downloads) | `folder-downloads.png`, `folder.png`               |
//! | drive                      | `drive.png`                                        |
//! | file `report.pdf`          | `ext-pdf.png`, `document.png` (category), `file.png` |
//! | app `code.exe` (launcher)  | `app-code.png` (apps keep their own icon otherwise) |
//!
//! Themes:
//! - `system`: no theme; Windows shell icons everywhere.
//! - `chickenwm`: built in. [`generate_builtin`] draws the whole set from the current palette, so
//!   icons follow the accent color and light/dark mode.
//! - anything else: a user-provided folder (any subset of the names above; missing ones fall
//!   back).

use crate::config::{config_dir, RgbaColor};
use crate::theme::{Palette, ThemeMode};
use anyhow::Result;
use std::path::{Path, PathBuf};

pub const SPECIAL_FOLDERS: [&str; 7] = ["home", "desktop", "downloads", "documents", "pictures", "music", "videos"];

/// File categories with their own built-in icon.
pub const CATEGORIES: [&str; 12] = [
    "image", "video", "audio", "archive", "code", "text", "document", "pdf", "executable", "font",
    "disk", "file",
];

pub fn icons_root() -> PathBuf {
    config_dir().join("icons")
}

/// Category for an extension (lower-case, no dot).
pub fn category_for_ext(ext: &str) -> &'static str {
    match ext {
        "png" | "jpg" | "jpeg" | "gif" | "webp" | "bmp" | "svg" | "ico" | "tif" | "tiff" | "heic" | "avif" | "raw" | "psd" => "image",
        "mp4" | "mkv" | "webm" | "mov" | "avi" | "wmv" | "flv" | "m4v" => "video",
        "mp3" | "flac" | "wav" | "ogg" | "m4a" | "aac" | "opus" | "wma" => "audio",
        "zip" | "7z" | "rar" | "gz" | "tgz" | "tar" | "xz" | "bz2" | "zst" | "cab" => "archive",
        "rs" | "py" | "js" | "ts" | "tsx" | "jsx" | "c" | "h" | "cpp" | "hpp" | "cs" | "go" | "java" | "kt" | "lua" | "rb"
        | "php" | "sh" | "ps1" | "bat" | "cmd" | "html" | "css" | "scss" | "json" | "toml" | "yaml" | "yml" | "xml"
        | "sql" | "wgsl" | "glsl" | "hlsl" | "conf" | "ini" => "code",
        "txt" | "md" | "log" | "csv" | "rtf" | "nfo" => "text",
        "doc" | "docx" | "odt" | "xls" | "xlsx" | "ods" | "ppt" | "pptx" | "odp" | "epub" => "document",
        "pdf" => "pdf",
        "exe" | "msi" | "com" | "appx" | "msix" | "dll" | "sys" => "executable",
        "ttf" | "otf" | "woff" | "woff2" => "font",
        "iso" | "img" | "vhd" | "vhdx" => "disk",
        _ => "file",
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IconRequest {
    Folder,
    /// One of [`SPECIAL_FOLDERS`].
    SpecialFolder(String),
    Drive,
    /// Lower-case extension without the dot ("" for none).
    File(String),
    /// Executable stem, lower-case ("code" for `Code.exe`).
    App(String),
}

/// A loaded icon theme. Cheap to clone.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IconTheme {
    pub name: String,
    pub dir: Option<PathBuf>,
    pub inherits: Vec<String>,
}

impl IconTheme {
    pub fn load(name: &str) -> Self {
        let name = name.trim();
        if name.is_empty() || name.eq_ignore_ascii_case("system") {
            return Self {
                name: "system".to_string(),
                dir: None,
                inherits: Vec::new(),
            };
        }
        let d = icons_root().join(name);
        let (dir, inherits) = if d.is_dir() {
            let mut inh = Vec::new();
            if let Ok(content) = std::fs::read_to_string(d.join("index.theme")) {
                for line in content.lines() {
                    let trimmed = line.trim();
                    if let Some(val) = trimmed.strip_prefix("Inherits=").or_else(|| trimmed.strip_prefix("inherits=")) {
                        for item in val.split([',', ';']) {
                            let item = item.trim();
                            if !item.is_empty() && !inh.iter().any(|x: &String| x.eq_ignore_ascii_case(item)) {
                                inh.push(item.to_string());
                            }
                        }
                    }
                }
            }
            (Some(d), inh)
        } else {
            (None, Vec::new())
        };
        Self {
            name: name.to_string(),
            dir,
            inherits,
        }
    }

    /// The theme selected in `chickenwm.conf`.
    pub fn current() -> Self {
        let name = crate::config::load_or_init_config()
            .map(|(_, cfg)| cfg.theme.icon_theme)
            .unwrap_or_else(|_| "chickenwm".into());
        Self::load(&name)
    }

    pub fn is_system(&self) -> bool {
        self.dir.is_none()
    }

    pub fn resolve(&self, req: &IconRequest) -> Option<PathBuf> {
        let dir = self.dir.as_ref()?;
        if let Some(p) = resolve_in_dir(dir, req) {
            return Some(p);
        }
        for inh in &self.inherits {
            if !inh.eq_ignore_ascii_case(&self.name) {
                if let Some(p) = IconTheme::load(inh).resolve(req) {
                    return Some(p);
                }
            }
        }
        if !self.name.eq_ignore_ascii_case("chickenwm") {
            let chicken = IconTheme::load("chickenwm");
            if let Some(d) = chicken.dir.as_ref() {
                if let Some(p) = resolve_in_dir(d, req) {
                    return Some(p);
                }
            }
        }
        None
    }
}

fn candidate_names_and_contexts(req: &IconRequest) -> Vec<(&'static str, String)> {
    match req {
        IconRequest::Folder => vec![
            ("places", "folder".into()),
            ("places", "default-folder".into()),
            ("places", "inode-directory".into()),
            ("", "folder".into()),
        ],
        IconRequest::SpecialFolder(s) => {
            let mut v = Vec::new();
            match s.as_str() {
                "home" => {
                    v.push(("places", "user-home".into()));
                    v.push(("places", "folder-home".into()));
                    v.push(("places", "folder_home".into()));
                }
                "desktop" => {
                    v.push(("places", "user-desktop".into()));
                    v.push(("places", "folder-desktop".into()));
                    v.push(("places", "desktop".into()));
                }
                "downloads" => {
                    v.push(("places", "folder-download".into()));
                    v.push(("places", "folder-downloads".into()));
                }
                "documents" => {
                    v.push(("places", "folder-documents".into()));
                    v.push(("places", "folder-document".into()));
                }
                "pictures" => {
                    v.push(("places", "folder-pictures".into()));
                    v.push(("places", "folder-images".into()));
                    v.push(("places", "folder-photos".into()));
                }
                "music" => {
                    v.push(("places", "folder-music".into()));
                    v.push(("places", "folder-sound".into()));
                    v.push(("places", "folder-audio".into()));
                }
                "videos" => {
                    v.push(("places", "folder-videos".into()));
                    v.push(("places", "folder-video".into()));
                    v.push(("places", "folder-movies".into()));
                }
                _ => {}
            }
            v.push(("", format!("folder-{s}")));
            v.push(("places", "folder".into()));
            v.push(("", "folder".into()));
            v
        }
        IconRequest::Drive => vec![
            ("devices", "drive-harddisk".into()),
            ("devices", "harddrive".into()),
            ("devices", "drive-removable-media".into()),
            ("devices", "disks".into()),
            ("", "drive".into()),
        ],
        IconRequest::File(ext) => {
            let mut v = Vec::new();
            let cat = category_for_ext(ext);
            if !ext.is_empty() {
                v.push(("", format!("ext-{ext}")));
                match ext.as_str() {
                    "pdf" => {
                        v.push(("mimetypes", "application-pdf".into()));
                        v.push(("mimetypes", "pdf".into()));
                    }
                    "png" | "jpg" | "jpeg" | "gif" | "webp" | "bmp" | "svg" => {
                        v.push(("mimetypes", format!("image-{ext}")));
                        v.push(("mimetypes", "image-x-generic".into()));
                    }
                    "mp4" | "mkv" | "webm" | "mov" | "avi" => {
                        v.push(("mimetypes", format!("video-{ext}")));
                        v.push(("mimetypes", "video-x-generic".into()));
                    }
                    "mp3" | "flac" | "wav" | "ogg" | "m4a" => {
                        v.push(("mimetypes", "audio-x-generic".into()));
                    }
                    "zip" | "7z" | "rar" | "tar" | "gz" | "bz2" | "xz" => {
                        v.push(("mimetypes", "package-x-generic".into()));
                        v.push(("mimetypes", "application-zip".into()));
                    }
                    "rs" | "py" | "js" | "ts" | "c" | "cpp" | "go" | "html" | "css" => {
                        v.push(("mimetypes", format!("text-x-{ext}")));
                        v.push(("mimetypes", "text-x-script".into()));
                        v.push(("mimetypes", "text-x-source".into()));
                    }
                    "exe" | "msi" | "bat" | "cmd" => {
                        v.push(("mimetypes", "application-x-executable".into()));
                        v.push(("mimetypes", "application-x-ms-dos-executable".into()));
                    }
                    "txt" | "md" | "log" => {
                        v.push(("mimetypes", "text-plain".into()));
                        v.push(("mimetypes", "text-x-generic".into()));
                    }
                    "doc" | "docx" | "odt" | "rtf" => {
                        v.push(("mimetypes", "x-office-document".into()));
                        v.push(("mimetypes", "application-msword".into()));
                    }
                    "ttf" | "otf" | "woff" => {
                        v.push(("mimetypes", "font-x-generic".into()));
                    }
                    "iso" | "img" => {
                        v.push(("mimetypes", "media-optical".into()));
                        v.push(("mimetypes", "application-x-iso9660-image".into()));
                    }
                    _ => {}
                }
            }
            v.push(("", cat.to_string()));
            v.push(("mimetypes", format!("{cat}-x-generic")));
            v.push(("mimetypes", "text-x-generic".into()));
            v.push(("", "file".into()));
            v.push(("mimetypes", "empty".into()));
            v
        }
        IconRequest::App(stem) => {
            vec![
                ("apps", stem.clone()),
                ("apps", stem.replace(' ', "-")),
                ("", format!("app-{stem}")),
                ("apps", format!("application-x-{stem}")),
            ]
        }
    }
}

fn resolve_in_dir(dir: &Path, req: &IconRequest) -> Option<PathBuf> {
    let candidates = candidate_names_and_contexts(req);
    const SIZES: [&str; 7] = ["scalable", "64x64", "48x48", "32x32", "24x24", "16x16", "128x128"];
    const EXTS: [&str; 2] = ["svg", "png"];

    for (context, name) in candidates {
        for ext in EXTS {
            // 1. Direct root file: <theme>/<name>.<ext> (only if not an app context)
            if context != "apps" {
                let p1 = dir.join(format!("{name}.{ext}"));
                if p1.is_file() {
                    return Some(p1);
                }
            }
            // 2. Direct context folder: <theme>/<context>/<name>.<ext>
            if !context.is_empty() {
                let p2 = dir.join(context).join(format!("{name}.{ext}"));
                if p2.is_file() {
                    return Some(p2);
                }
            }
            // 3. Size folders: <theme>/<size>/<context>/<name>.<ext> or <theme>/<size>/<name>.<ext>
            for sz in SIZES {
                if !context.is_empty() {
                    let p3 = dir.join(sz).join(context).join(format!("{name}.{ext}"));
                    if p3.is_file() {
                        return Some(p3);
                    }
                }
                if context != "apps" {
                    let p4 = dir.join(sz).join(format!("{name}.{ext}"));
                    if p4.is_file() {
                        return Some(p4);
                    }
                }
            }
        }
    }
    None
}

/// Folder names under `icons/` (plus `system`), for theme pickers.
pub fn available_themes() -> Vec<String> {
    let mut v = vec![
        "system".to_string(),
        "chickenwm".to_string(),
        "lucide".to_string(),
        "font-awesome".to_string(),
        "papirus".to_string(),
    ];
    if let Ok(rd) = std::fs::read_dir(icons_root()) {
        for e in rd.flatten() {
            if e.path().is_dir() {
                let n = e.file_name().to_string_lossy().into_owned();
                if !v.iter().any(|x| x.eq_ignore_ascii_case(&n)) {
                    v.push(n);
                }
            }
        }
    }
    v
}

/// Recursively copies all files and subdirectories from `src` to `dst`.
fn copy_dir_all(src: &Path, dst: &Path) -> Result<()> {
    std::fs::create_dir_all(dst)?;
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let target = dst.join(entry.file_name());
        if file_type.is_dir() {
            copy_dir_all(&entry.path(), &target)?;
        } else if file_type.is_file() {
            std::fs::copy(entry.path(), target)?;
        }
    }
    Ok(())
}

fn sanitize_theme_name(name: &str) -> String {
    let clean: String = name
        .chars()
        .map(|c| if c.is_alphanumeric() || c == '-' || c == '_' { c } else { '-' })
        .collect();
    let s = clean.trim_matches('-');
    if s.is_empty() {
        "custom-theme".to_string()
    } else {
        s.to_lowercase()
    }
}

fn find_theme_root(base: &Path) -> Option<PathBuf> {
    if base.join("index.theme").exists() || base.join("places").is_dir() || base.join("folder.svg").exists() || base.join("folder.png").exists() {
        return Some(base.to_path_buf());
    }
    if let Ok(entries) = std::fs::read_dir(base) {
        let dirs: Vec<PathBuf> = entries.flatten().filter_map(|e| e.path().is_dir().then_some(e.path())).collect();
        for d in &dirs {
            if d.join("index.theme").exists() || d.join("places").is_dir() || d.join("folder.svg").exists() || d.join("folder.png").exists() {
                return Some(d.clone());
            }
        }
        if dirs.len() == 1 {
            return find_theme_root(&dirs[0]);
        }
    }
    None
}

fn extract_theme_name(index_content: &str) -> Option<String> {
    for line in index_content.lines() {
        let t = line.trim();
        if let Some(val) = t.strip_prefix("Name=").or_else(|| t.strip_prefix("name=")) {
            let n = val.trim();
            if !n.is_empty() {
                return Some(n.to_string());
            }
        }
    }
    None
}

/// Import a Linux icon theme from a folder, .zip, .tar.gz, .tgz, or .tar file.
/// Extracts/copies to `~/.config/chickenwm/icons/<theme-name>` and returns the theme name.
pub fn import_theme_from_path(source: &Path) -> Result<String> {
    if !source.exists() {
        anyhow::bail!("Source path does not exist: {}", source.display());
    }

    let temp_dir = std::env::temp_dir().join(format!("chickenwm_icon_import_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&temp_dir);
    std::fs::create_dir_all(&temp_dir)?;

    let is_dir = source.is_dir();
    let path_str = source.to_string_lossy().to_ascii_lowercase();

    if is_dir {
        copy_dir_all(source, &temp_dir)?;
    } else if path_str.ends_with(".zip") {
        let file = std::fs::File::open(source)?;
        let mut archive = zip::ZipArchive::new(file)?;
        archive.extract(&temp_dir)?;
    } else if path_str.ends_with(".tar.gz") || path_str.ends_with(".tgz") {
        let file = std::fs::File::open(source)?;
        let tar = flate2::read::GzDecoder::new(file);
        let mut archive = tar::Archive::new(tar);
        archive.unpack(&temp_dir)?;
    } else if path_str.ends_with(".tar") {
        let file = std::fs::File::open(source)?;
        let mut archive = tar::Archive::new(file);
        archive.unpack(&temp_dir)?;
    } else {
        anyhow::bail!("Unsupported icon theme archive format: {}. Expected folder, .zip, or .tar.gz", source.display());
    }

    let theme_root = find_theme_root(&temp_dir).unwrap_or(temp_dir.clone());
    let theme_name = if let Ok(content) = std::fs::read_to_string(theme_root.join("index.theme")) {
        extract_theme_name(&content).unwrap_or_else(|| {
            source.file_stem().map(|s| s.to_string_lossy().into_owned()).unwrap_or_else(|| "custom-icons".into())
        })
    } else {
        source.file_stem().map(|s| s.to_string_lossy().into_owned()).unwrap_or_else(|| "custom-icons".into())
    };

    let sanitized = sanitize_theme_name(&theme_name);
    let target = icons_root().join(&sanitized);
    if target.exists() {
        let _ = std::fs::remove_dir_all(&target);
    }
    std::fs::create_dir_all(&target)?;
    copy_dir_all(&theme_root, &target)?;
    let _ = std::fs::remove_dir_all(&temp_dir);

    Ok(sanitized)
}

/// Ensures all predetermined bundled icon packs (`lucide`, `font-awesome`, `papirus`, `chickenwm`)
/// exist in `~/.config/chickenwm/icons/`.
pub fn ensure_builtin_themes(p: &Palette) -> Result<()> {
    let _ = generate_builtin(p);

    let lucide_dir = icons_root().join("lucide");
    if !lucide_dir.join("folder.svg").exists() {
        std::fs::create_dir_all(&lucide_dir)?;
        for asset in crate::icon_theme_presets::LUCIDE_ICONS {
            let target = lucide_dir.join(asset.rel_path);
            if let Some(parent) = target.parent() {
                let _ = std::fs::create_dir_all(parent);
            }
            let _ = std::fs::write(&target, asset.svg);
        }
    }

    let fa_dir = icons_root().join("font-awesome");
    if !fa_dir.join("folder.svg").exists() {
        std::fs::create_dir_all(&fa_dir)?;
        for asset in crate::icon_theme_presets::FONT_AWESOME_ICONS {
            let target = fa_dir.join(asset.rel_path);
            if let Some(parent) = target.parent() {
                let _ = std::fs::create_dir_all(parent);
            }
            let _ = std::fs::write(&target, asset.svg);
        }
    }

    let papirus_dir = icons_root().join("papirus");
    if !papirus_dir.join("index.theme").exists() {
        std::fs::create_dir_all(papirus_dir.join("places"))?;
        std::fs::create_dir_all(papirus_dir.join("mimetypes"))?;
        std::fs::create_dir_all(papirus_dir.join("devices"))?;
        std::fs::create_dir_all(papirus_dir.join("apps"))?;
        let _ = std::fs::write(papirus_dir.join("index.theme"), crate::icon_theme_presets::PAPIRUS_THEME_INDEX);
        for asset in crate::icon_theme_presets::PAPIRUS_ICONS {
            let target = papirus_dir.join(asset.rel_path);
            if let Some(parent) = target.parent() {
                let _ = std::fs::create_dir_all(parent);
            }
            let _ = std::fs::write(&target, asset.svg);
        }
    }

    Ok(())
}

// ───────────────────────────── built-in icon set ─────────────────────────────

const SIZE: u32 = 64;
const SS: u32 = 4; // supersampling per axis

struct Canvas {
    px: Vec<[f32; 4]>, // premultiplied RGBA 0..1
}

type Shape<'a> = &'a dyn Fn(f32, f32) -> bool;

impl Canvas {
    fn new() -> Self {
        Self { px: vec![[0.0; 4]; (SIZE * SIZE) as usize] }
    }

    /// Fills `shape` (a predicate in 0..64 icon units) with anti-aliased coverage.
    fn fill(&mut self, shape: Shape, color: RgbaColor) {
        let a = color.a as f32 / 255.0;
        let rgb = [color.r as f32 / 255.0, color.g as f32 / 255.0, color.b as f32 / 255.0];
        for y in 0..SIZE {
            for x in 0..SIZE {
                let mut hits = 0;
                for sy in 0..SS {
                    for sx in 0..SS {
                        let fx = x as f32 + (sx as f32 + 0.5) / SS as f32;
                        let fy = y as f32 + (sy as f32 + 0.5) / SS as f32;
                        if shape(fx, fy) {
                            hits += 1;
                        }
                    }
                }
                if hits == 0 {
                    continue;
                }
                let cov = hits as f32 / (SS * SS) as f32 * a;
                let p = &mut self.px[(y * SIZE + x) as usize];
                for i in 0..3 {
                    p[i] = rgb[i] * cov + p[i] * (1.0 - cov);
                }
                p[3] = cov + p[3] * (1.0 - cov);
            }
        }
    }

    fn to_rgba8(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(self.px.len() * 4);
        for p in &self.px {
            let a = p[3];
            let un = |v: f32| if a > 0.0 { (v / a).clamp(0.0, 1.0) } else { 0.0 };
            out.extend_from_slice(&[
                (un(p[0]) * 255.0).round() as u8,
                (un(p[1]) * 255.0).round() as u8,
                (un(p[2]) * 255.0).round() as u8,
                (a * 255.0).round() as u8,
            ]);
        }
        out
    }
}

fn rrect(x0: f32, y0: f32, x1: f32, y1: f32, r: f32) -> impl Fn(f32, f32) -> bool {
    // A radius larger than half the shorter side would make the clamp ranges invalid.
    let r = r.min((x1 - x0) / 2.0).min((y1 - y0) / 2.0).max(0.0);
    move |x, y| {
        if x < x0 || x > x1 || y < y0 || y > y1 {
            return false;
        }
        let cx = x.clamp(x0 + r, x1 - r);
        let cy = y.clamp(y0 + r, y1 - r);
        (x - cx).powi(2) + (y - cy).powi(2) <= r * r
    }
}

fn circle(cx: f32, cy: f32, r: f32) -> impl Fn(f32, f32) -> bool {
    move |x, y| (x - cx).powi(2) + (y - cy).powi(2) <= r * r
}

fn ring(cx: f32, cy: f32, r: f32, w: f32) -> impl Fn(f32, f32) -> bool {
    move |x, y| {
        let d = ((x - cx).powi(2) + (y - cy).powi(2)).sqrt();
        (d - r).abs() <= w / 2.0
    }
}

fn poly(points: Vec<(f32, f32)>) -> impl Fn(f32, f32) -> bool {
    move |x, y| {
        let mut inside = false;
        let n = points.len();
        for i in 0..n {
            let (xi, yi) = points[i];
            let (xj, yj) = points[(i + n - 1) % n];
            if (yi > y) != (yj > y) && x < (xj - xi) * (y - yi) / (yj - yi) + xi {
                inside = !inside;
            }
        }
        inside
    }
}

/// A stroked polyline with round caps.
fn stroke(points: Vec<(f32, f32)>, w: f32) -> impl Fn(f32, f32) -> bool {
    move |x, y| {
        points.windows(2).any(|s| {
            let ((ax, ay), (bx, by)) = (s[0], s[1]);
            let (dx, dy) = (bx - ax, by - ay);
            let len2 = dx * dx + dy * dy;
            let t = if len2 > 0.0 { (((x - ax) * dx + (y - ay) * dy) / len2).clamp(0.0, 1.0) } else { 0.0 };
            let (px, py) = (ax + t * dx, ay + t * dy);
            (x - px).powi(2) + (y - py).powi(2) <= (w / 2.0).powi(2)
        })
    }
}

fn mix(a: RgbaColor, b: RgbaColor, t: f32) -> RgbaColor {
    let l = |x: u8, y: u8| (x as f32 + (y as f32 - x as f32) * t).round() as u8;
    RgbaColor { r: l(a.r, b.r), g: l(a.g, b.g), b: l(a.b, b.b), a: l(a.a, b.a) }
}

/// Draws a glyph centered at (cx, cy) with roughly `s` units of extent.
fn glyph(c: &mut Canvas, name: &str, cx: f32, cy: f32, s: f32, col: RgbaColor) {
    let w = (s * 0.16).max(2.2);
    match name {
        "home" => {
            c.fill(&poly(vec![(cx - s * 0.5, cy), (cx, cy - s * 0.45), (cx + s * 0.5, cy)]), col);
            c.fill(&rrect(cx - s * 0.34, cy - 1.0, cx + s * 0.34, cy + s * 0.42, 1.5), col);
        }
        "desktop" => {
            c.fill(&rrect(cx - s * 0.5, cy - s * 0.38, cx + s * 0.5, cy + s * 0.2, 2.0), col);
            c.fill(&rrect(cx - s * 0.08, cy + s * 0.2, cx + s * 0.08, cy + s * 0.36, 0.5), col);
            c.fill(&rrect(cx - s * 0.26, cy + s * 0.34, cx + s * 0.26, cy + s * 0.44, 1.0), col);
        }
        "downloads" => {
            c.fill(&stroke(vec![(cx, cy - s * 0.42), (cx, cy + s * 0.14)], w), col);
            c.fill(&stroke(vec![(cx - s * 0.28, cy - s * 0.06), (cx, cy + s * 0.2), (cx + s * 0.28, cy - s * 0.06)], w), col);
            c.fill(&stroke(vec![(cx - s * 0.4, cy + s * 0.42), (cx + s * 0.4, cy + s * 0.42)], w), col);
        }
        "documents" | "text" | "document" => {
            for (i, frac) in [1.0, 0.8, 1.0, 0.6].iter().enumerate() {
                let y = cy - s * 0.36 + i as f32 * s * 0.24;
                c.fill(&stroke(vec![(cx - s * 0.4, y), (cx - s * 0.4 + s * 0.8 * frac, y)], w * 0.85), col);
            }
        }
        "pictures" | "image" => {
            c.fill(&poly(vec![(cx - s * 0.5, cy + s * 0.38), (cx - s * 0.12, cy - s * 0.14), (cx + s * 0.14, cy + s * 0.14), (cx + s * 0.28, cy), (cx + s * 0.5, cy + s * 0.38)]), col);
            c.fill(&circle(cx + s * 0.24, cy - s * 0.28, s * 0.13), col);
        }
        "music" | "audio" => {
            c.fill(&stroke(vec![(cx + s * 0.18, cy + s * 0.28), (cx + s * 0.18, cy - s * 0.42), (cx + s * 0.44, cy - s * 0.3)], w), col);
            c.fill(&circle(cx - s * 0.02, cy + s * 0.3, s * 0.2), col);
        }
        "videos" | "video" => {
            c.fill(&poly(vec![(cx - s * 0.28, cy - s * 0.4), (cx + s * 0.4, cy), (cx - s * 0.28, cy + s * 0.4)]), col);
        }
        "archive" => {
            c.fill(&rrect(cx - s * 0.1, cy - s * 0.5, cx + s * 0.1, cy + s * 0.5, 1.0), col);
            for i in 0..4 {
                let y = cy - s * 0.42 + i as f32 * s * 0.22;
                let dx = if i % 2 == 0 { -s * 0.2 } else { s * 0.2 };
                c.fill(&rrect(cx + dx - s * 0.1, y, cx + dx + s * 0.1, y + s * 0.12, 0.8), col);
            }
        }
        "code" => {
            c.fill(&stroke(vec![(cx - s * 0.14, cy - s * 0.32), (cx - s * 0.46, cy), (cx - s * 0.14, cy + s * 0.32)], w), col);
            c.fill(&stroke(vec![(cx + s * 0.14, cy - s * 0.32), (cx + s * 0.46, cy), (cx + s * 0.14, cy + s * 0.32)], w), col);
        }
        "pdf" => {
            c.fill(&rrect(cx - s * 0.5, cy - s * 0.2, cx + s * 0.5, cy + s * 0.22, 2.0), col);
        }
        "executable" => {
            c.fill(&rrect(cx - s * 0.46, cy - s * 0.38, cx + s * 0.46, cy + s * 0.38, 2.5), col);
            let bg = RgbaColor { a: 255, ..mix(col, RgbaColor { r: 255, g: 255, b: 255, a: 255 }, 0.85) };
            c.fill(&rrect(cx - s * 0.38, cy - s * 0.18, cx + s * 0.38, cy + s * 0.3, 1.5), bg);
        }
        "font" => {
            c.fill(&stroke(vec![(cx - s * 0.4, cy + s * 0.42), (cx, cy - s * 0.42), (cx + s * 0.4, cy + s * 0.42)], w), col);
            c.fill(&stroke(vec![(cx - s * 0.22, cy + s * 0.12), (cx + s * 0.22, cy + s * 0.12)], w), col);
        }
        "disk" => {
            c.fill(&ring(cx, cy, s * 0.36, w), col);
            c.fill(&circle(cx, cy, s * 0.1), col);
        }
        _ => {}
    }
}

fn folder_icon(p: &Palette, special: Option<&str>) -> Canvas {
    let mut c = Canvas::new();
    let body = p.dir;
    let back = mix(p.dir, if p.mode == ThemeMode::Light { RgbaColor { r: 0, g: 0, b: 0, a: 255 } } else { p.base }, 0.25);
    let front = mix(p.dir, RgbaColor { r: 255, g: 255, b: 255, a: 255 }, if p.mode == ThemeMode::Light { 0.12 } else { 0.18 });
    // Back with tab
    c.fill(&rrect(6.0, 12.0, 28.0, 24.0, 3.5), back);
    c.fill(&rrect(6.0, 17.0, 58.0, 52.0, 4.0), back);
    // Front
    c.fill(&rrect(6.0, 22.0, 58.0, 54.0, 4.0), body);
    c.fill(&rrect(6.0, 22.0, 58.0, 26.0, 2.0), front);
    if let Some(s) = special {
        let ink = if p.mode == ThemeMode::Light { p.accent_fg } else { p.base };
        glyph(&mut c, s, 32.0, 39.0, 18.0, RgbaColor { a: 235, ..ink });
    }
    c
}

fn file_icon(p: &Palette, category: &str) -> Canvas {
    let mut c = Canvas::new();
    let paper = if p.mode == ThemeMode::Light { p.surface } else { mix(p.surface, p.fg, 0.10) };
    let edge = mix(paper, p.fg, 0.28);
    let fold = mix(paper, p.fg, 0.16);
    let page = |x: f32, y: f32| {
        // page with the top-right corner cut off
        x >= 12.0 && x <= 52.0 && y >= 5.0 && y <= 59.0 && (x - 40.0) <= (y - 5.0)
    };
    let inner = |x: f32, y: f32| x >= 14.0 && x <= 50.0 && y >= 7.0 && y <= 57.0 && (x - 39.0) <= (y - 7.0);
    c.fill(&page, edge);
    c.fill(&inner, paper);
    c.fill(&poly(vec![(40.0, 5.0), (52.0, 17.0), (40.0, 17.0)]), fold);
    let (tint, glyph_name) = match category {
        "image" => (p.media, "image"),
        "video" => (p.media, "video"),
        "audio" => (p.media, "audio"),
        "archive" => (p.archive, "archive"),
        "code" => (p.code, "code"),
        "text" => (p.document, "text"),
        "document" => (p.code, "document"),
        "pdf" => (p.danger, "pdf"),
        "font" => (p.document, "font"),
        "disk" => (p.subtext, "disk"),
        _ => (p.document, ""),
    };
    if category == "pdf" {
        glyph(&mut c, "pdf", 32.0, 44.0, 26.0, tint);
        let txt = RgbaColor { a: 255, ..paper };
        // three small bars on the band, a stand-in for the "PDF" label
        for i in 0..3 {
            let x = 22.0 + i as f32 * 8.0;
            c.fill(&rrect(x, 42.0, x + 5.0, 46.0, 1.0), txt);
        }
    } else if !glyph_name.is_empty() {
        glyph(&mut c, glyph_name, 32.0, 38.0, 20.0, tint);
    }
    c
}

fn executable_icon(p: &Palette) -> Canvas {
    let mut c = Canvas::new();
    let frame = p.exec;
    c.fill(&rrect(6.0, 10.0, 58.0, 54.0, 6.0), frame);
    let screen = if p.mode == ThemeMode::Light { p.surface } else { p.base };
    c.fill(&rrect(10.0, 20.0, 54.0, 50.0, 3.0), screen);
    for (i, x) in [12.0f32, 18.0, 24.0].iter().enumerate() {
        let dot = mix(screen, frame, 0.4 + i as f32 * 0.2);
        c.fill(&circle(*x, 15.0, 2.0), dot);
    }
    c.fill(&stroke(vec![(18.0, 28.0), (26.0, 35.0), (18.0, 42.0)], 3.5), frame);
    c.fill(&stroke(vec![(30.0, 43.0), (42.0, 43.0)], 3.5), frame);
    c
}

fn drive_icon(p: &Palette) -> Canvas {
    let mut c = Canvas::new();
    let body = mix(p.surface, p.fg, if p.mode == ThemeMode::Light { 0.22 } else { 0.18 });
    let face = mix(body, p.fg, 0.12);
    c.fill(&rrect(6.0, 18.0, 58.0, 46.0, 6.0), body);
    c.fill(&rrect(6.0, 34.0, 58.0, 46.0, 6.0), face);
    c.fill(&circle(48.0, 40.0, 3.0), p.accent);
    for i in 0..4 {
        let x = 14.0 + i as f32 * 6.0;
        c.fill(&rrect(x, 38.0, x + 3.0, 42.0, 1.0), mix(face, p.fg, 0.35));
    }
    c
}

/// Render SVG data to an RGBA pixel buffer at the specified size.
pub fn render_svg_to_rgba(svg_data: &[u8], width: u32, height: u32) -> Option<Vec<u8>> {
    use resvg::{tiny_skia, usvg};
    let opt = usvg::Options::default();
    let tree = usvg::Tree::from_data(svg_data, &opt).ok()?;
    let mut pixmap = tiny_skia::Pixmap::new(width, height)?;
    let sx = width as f32 / tree.size().width();
    let sy = height as f32 / tree.size().height();
    let transform = tiny_skia::Transform::from_scale(sx, sy);
    resvg::render(&tree, transform, &mut pixmap.as_mut());
    Some(pixmap.take())
}

/// Load an icon file (.svg or raster image like .png), rasterizing/resizing to `(side, side)`.
pub fn load_icon_image(path: &Path, side: u32) -> Option<(usize, usize, Vec<u8>)> {
    let is_svg = path.extension().and_then(|e| e.to_str()).map(|e| e.eq_ignore_ascii_case("svg")).unwrap_or(false);
    if is_svg {
        let bytes = std::fs::read(path).ok()?;
        let rgba = render_svg_to_rgba(&bytes, side, side)?;
        Some((side as usize, side as usize, rgba))
    } else {
        let img = image::open(path).ok()?;
        let img = img.resize(side, side, image::imageops::FilterType::Lanczos3).to_rgba8();
        Some((img.width() as usize, img.height() as usize, img.into_raw()))
    }
}

fn save_png(canvas: &Canvas, path: &Path) -> Result<()> {
    let img = image::RgbaImage::from_raw(SIZE, SIZE, canvas.to_rgba8())
        .ok_or_else(|| anyhow::anyhow!("icon buffer size"))?;
    img.save(path)?;
    Ok(())
}

/// (Re)draws the built-in `chickenwm` icon theme from the palette.
pub fn generate_builtin(p: &Palette) -> Result<PathBuf> {
    let dir = icons_root().join("chickenwm");
    std::fs::create_dir_all(&dir)?;
    save_png(&folder_icon(p, None), &dir.join("folder.png"))?;
    for s in SPECIAL_FOLDERS {
        save_png(&folder_icon(p, Some(s)), &dir.join(format!("folder-{s}.png")))?;
    }
    save_png(&drive_icon(p), &dir.join("drive.png"))?;
    for cat in CATEGORIES {
        let canvas = if cat == "executable" { executable_icon(p) } else { file_icon(p, cat) };
        save_png(&canvas, &dir.join(format!("{cat}.png")))?;
    }
    std::fs::write(
        dir.join("README.txt"),
        "Built-in ChickenWM icon theme, regenerated from the system palette whenever the theme \
         changes. Don't edit these files; copy the folder under a new name to make your own theme.\n",
    )?;
    Ok(dir)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::theme::DEFAULT_ACCENT;

    #[test]
    fn categories_and_resolution() {
        assert_eq!(category_for_ext("rs"), "code");
        assert_eq!(category_for_ext("PDF".to_ascii_lowercase().as_str()), "pdf");
        assert_eq!(category_for_ext("weird"), "file");

        let dir = std::env::temp_dir().join("chickenwm-icon-theme-test");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        for f in ["folder.png", "code.png", "file.png", "ext-rs.png"] {
            std::fs::write(dir.join(f), b"x").unwrap();
        }
        let theme = IconTheme { name: "t".into(), dir: Some(dir.clone()), inherits: Vec::new() };
        assert_eq!(theme.resolve(&IconRequest::File("rs".into())), Some(dir.join("ext-rs.png")));
        assert_eq!(theme.resolve(&IconRequest::File("py".into())), Some(dir.join("code.png")));
        assert_eq!(theme.resolve(&IconRequest::File("zip".into())), Some(dir.join("file.png")));
        assert_eq!(theme.resolve(&IconRequest::SpecialFolder("music".into())), Some(dir.join("folder.png")));
        assert_eq!(theme.resolve(&IconRequest::App("code".into())), None);
        assert!(IconTheme::load("system").is_system());
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_svg_rendering_and_loader() {
        let svg = br##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><circle cx="12" cy="12" r="10" fill="#ff0000"/></svg>"##;
        let rgba = render_svg_to_rgba(svg, 32, 32).expect("render svg");
        assert_eq!(rgba.len(), 32 * 32 * 4);

        let temp_svg = std::env::temp_dir().join("chickenwm_test_icon.svg");
        std::fs::write(&temp_svg, svg).unwrap();
        let loaded = load_icon_image(&temp_svg, 48).expect("load icon image");
        assert_eq!(loaded.0, 48);
        assert_eq!(loaded.1, 48);
        assert_eq!(loaded.2.len(), 48 * 48 * 4);
        let _ = std::fs::remove_file(temp_svg);
    }

    #[test]
    fn test_freedesktop_xdg_resolution() {
        let dir = std::env::temp_dir().join("chickenwm-xdg-test-theme");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("places")).unwrap();
        std::fs::create_dir_all(dir.join("mimetypes")).unwrap();
        std::fs::create_dir_all(dir.join("devices")).unwrap();

        std::fs::write(dir.join("places/folder.svg"), b"<svg/>").unwrap();
        std::fs::write(dir.join("places/user-home.svg"), b"<svg/>").unwrap();
        std::fs::write(dir.join("places/folder-download.svg"), b"<svg/>").unwrap();
        std::fs::write(dir.join("devices/drive-harddisk.svg"), b"<svg/>").unwrap();
        std::fs::write(dir.join("mimetypes/application-pdf.svg"), b"<svg/>").unwrap();

        let theme = IconTheme { name: "xdg-test".into(), dir: Some(dir.clone()), inherits: Vec::new() };
        assert_eq!(theme.resolve(&IconRequest::Folder), Some(dir.join("places/folder.svg")));
        assert_eq!(theme.resolve(&IconRequest::SpecialFolder("home".into())), Some(dir.join("places/user-home.svg")));
        assert_eq!(theme.resolve(&IconRequest::SpecialFolder("downloads".into())), Some(dir.join("places/folder-download.svg")));
        assert_eq!(theme.resolve(&IconRequest::Drive), Some(dir.join("devices/drive-harddisk.svg")));
        assert_eq!(theme.resolve(&IconRequest::File("pdf".into())), Some(dir.join("mimetypes/application-pdf.svg")));

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn builtin_icons_render() {
        for mode in [ThemeMode::Light, ThemeMode::Dark] {
            let p = Palette::new(DEFAULT_ACCENT, mode);
            let mut all = vec![folder_icon(&p, None), executable_icon(&p), drive_icon(&p)];
            all.extend(SPECIAL_FOLDERS.iter().map(|s| folder_icon(&p, Some(s))));
            all.extend(CATEGORIES.iter().filter(|c| **c != "executable").map(|c| file_icon(&p, c)));
            for canvas in all {
                let px = canvas.to_rgba8();
                let opaque = px.chunks(4).filter(|c| c[3] > 200).count();
                let clear = px.chunks(4).filter(|c| c[3] == 0).count();
                assert!(opaque > 600 && clear > 300, "icon should have a solid shape and transparent margins");
            }
        }
    }
}
