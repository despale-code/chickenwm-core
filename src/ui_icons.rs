//! Online SVG & Vector UI Icon Provider for ChickenWM applications (`chickenwm_core::ui_icons`).
//!
//! Fetches official Lucide / Iconify SVG icons from the online icon provider
//! (`https://api.iconify.design/lucide/{slug}.svg` / `https://unpkg.com/lucide-static@latest/icons/{slug}.svg`),
//! caches them in `~/.config/chickenwm/icon-cache/{slug}.svg`, rasterizes them via `resvg` at 64x64
//! white-mask RGBA, and renders them as anti-aliased `egui` textures tinted to any `Color32`.

#![cfg(feature = "egui")]

use egui::{Color32, CornerRadius, Pos2, Rect, Stroke, TextureHandle, TextureOptions, Vec2};
use parking_lot::Mutex;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::sync::{Arc, OnceLock};

const ICON_RASTER_SIZE: u32 = 64;

#[derive(Default)]
struct OnlineIconStore {
    rgba_cache: HashMap<String, Option<Arc<Vec<u8>>>>,
    in_flight: HashSet<String>,
}

static ICON_STORE: OnceLock<Mutex<OnlineIconStore>> = OnceLock::new();

fn icon_store() -> &'static Mutex<OnlineIconStore> {
    ICON_STORE.get_or_init(|| Mutex::new(OnlineIconStore::default()))
}

/// Directory on disk where online SVG icons are cached (`~/.config/chickenwm/icon-cache`).
pub fn icon_cache_dir() -> PathBuf {
    crate::config::config_dir().join("icon-cache")
}

fn rasterize_svg_white_mask(svg_bytes: &[u8]) -> Option<Arc<Vec<u8>>> {
    let mut rgba = crate::icon_theme::render_svg_to_rgba(svg_bytes, ICON_RASTER_SIZE, ICON_RASTER_SIZE)?;
    for px in rgba.chunks_exact_mut(4) {
        let a = px[3];
        if a > 0 {
            px[0] = 255;
            px[1] = 255;
            px[2] = 255;
        } else {
            px[0] = 0;
            px[1] = 0;
            px[2] = 0;
        }
    }
    Some(Arc::new(rgba))
}

fn fetch_online_svg_blocking(slug: &str) -> Option<Vec<u8>> {
    use std::os::windows::process::CommandExt;
    const CREATE_NO_WINDOW: u32 = 0x08000000;

    let urls = [
        format!("https://api.iconify.design/lucide/{slug}.svg"),
        format!("https://unpkg.com/lucide-static@latest/icons/{slug}.svg"),
    ];
    for url in &urls {
        if let Ok(out) = std::process::Command::new("curl.exe")
            .args(["-sL", "--fail", "--max-time", "8", url])
            .creation_flags(CREATE_NO_WINDOW)
            .output()
        {
            if out.status.success() && out.stdout.len() > 24 {
                let text = String::from_utf8_lossy(&out.stdout);
                if text.contains("<svg") {
                    return Some(out.stdout);
                }
            }
        }
    }
    None
}

fn get_or_fetch_icon_rgba(ctx: &egui::Context, slug: &str) -> Option<Arc<Vec<u8>>> {
    {
        let mut store = icon_store().lock();
        if let Some(cached) = store.rgba_cache.get(slug) {
            return cached.clone();
        }
        let disk_path = icon_cache_dir().join(format!("{slug}.svg"));
        if let Ok(bytes) = std::fs::read(&disk_path) {
            if let Some(rgba) = rasterize_svg_white_mask(&bytes) {
                store.rgba_cache.insert(slug.to_string(), Some(rgba.clone()));
                return Some(rgba);
            }
        }
        if store.in_flight.contains(slug) {
            return None;
        }
        store.in_flight.insert(slug.to_string());
    }

    let slug_owned = slug.to_string();
    let ctx_clone = ctx.clone();
    std::thread::spawn(move || {
        let fetched = fetch_online_svg_blocking(&slug_owned);
        let mut rasterized = None;
        if let Some(svg_bytes) = fetched {
            let dir = icon_cache_dir();
            let _ = std::fs::create_dir_all(&dir);
            let _ = std::fs::write(dir.join(format!("{slug_owned}.svg")), &svg_bytes);
            rasterized = rasterize_svg_white_mask(&svg_bytes);
        }
        let mut store = icon_store().lock();
        store.in_flight.remove(&slug_owned);
        store.rgba_cache.insert(slug_owned, rasterized);
        ctx_clone.request_repaint();
    });

    None
}

/// Paints an online Lucide/Iconify SVG icon into `rect` tinted with `color`.
/// Returns `true` if the online SVG texture was rendered.
pub fn paint_online_icon(painter: &egui::Painter, rect: Rect, slug: &str, color: Color32) -> bool {
    let ctx = painter.ctx();
    let tex_id = egui::Id::new(("chickenwm_online_icon_tex", slug));

    let handle: Option<TextureHandle> =
        ctx.data_mut(|d| d.get_temp::<TextureHandle>(tex_id));

    let handle = match handle {
        Some(h) => h,
        None => {
            let Some(rgba) = get_or_fetch_icon_rgba(ctx, slug) else {
                return false;
            };
            let img = egui::ColorImage::from_rgba_unmultiplied(
                [ICON_RASTER_SIZE as usize, ICON_RASTER_SIZE as usize],
                rgba.as_slice(),
            );
            let h = ctx.load_texture(
                format!("online_icon_{slug}"),
                img,
                TextureOptions::LINEAR,
            );
            ctx.data_mut(|d| d.insert_temp(tex_id, h.clone()));
            h
        }
    };

    let side = (rect.width().min(rect.height()) * 0.88).clamp(11.0, 26.0);
    let draw_rect = Rect::from_center_size(rect.center(), Vec2::splat(side));
    let uv = Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0));
    painter.image(handle.id(), draw_rect, uv, color);
    true
}

pub fn paint_gear(painter: &egui::Painter, rect: Rect, color: Color32) {
    if paint_online_icon(painter, rect, "settings", color) {
        return;
    }
    let c = rect.center();
    let r = rect.width().min(rect.height()) * 0.22;
    let stroke = Stroke::new(1.5_f32, color);
    painter.circle_stroke(c, r, stroke);
    for i in 0..6 {
        let angle = i as f32 * (std::f32::consts::PI / 3.0);
        let p1 = c + egui::vec2(angle.cos() * (r + 0.5), angle.sin() * (r + 0.5));
        let p2 = c + egui::vec2(angle.cos() * (r + 3.2), angle.sin() * (r + 3.2));
        painter.line_segment([p1, p2], Stroke::new(2.2_f32, color));
    }
}

pub fn paint_folder(painter: &egui::Painter, rect: Rect, color: Color32) {
    if paint_online_icon(painter, rect, "folder", color) {
        return;
    }
    let stroke = Stroke::new(1.4_f32, color);
    let c = rect.center();
    let w = (rect.width() * 0.7).min(16.0);
    let h = (rect.height() * 0.55).min(13.0);
    let r = Rect::from_center_size(c, egui::vec2(w, h));
    let tab = Rect::from_min_max(r.min - egui::vec2(0.0, 2.5), r.min + egui::vec2(w * 0.42, 0.0));
    painter.rect_stroke(tab, CornerRadius::same(1), stroke, egui::StrokeKind::Inside);
    painter.rect_stroke(r, CornerRadius::same(2), stroke, egui::StrokeKind::Inside);
}

pub fn paint_palette(painter: &egui::Painter, rect: Rect, color: Color32) {
    if paint_online_icon(painter, rect, "palette", color) {
        return;
    }
    let c = rect.center();
    let stroke = Stroke::new(1.4_f32, color);
    let r = Rect::from_center_size(c, egui::vec2(15.0, 13.0));
    painter.rect_stroke(r, CornerRadius::same(4), stroke, egui::StrokeKind::Inside);
    for dx in [-3.5_f32, 0.0, 3.5] {
        painter.circle_filled(c + egui::vec2(dx, -2.0), 1.2, color);
    }
    painter.circle_stroke(c + egui::vec2(2.5, 2.5), 1.5, stroke);
}

pub fn paint_wallpaper(painter: &egui::Painter, rect: Rect, color: Color32) {
    if paint_online_icon(painter, rect, "image", color) {
        return;
    }
    let c = rect.center();
    let stroke = Stroke::new(1.4_f32, color);
    let r = Rect::from_center_size(c, egui::vec2(15.0, 12.0));
    painter.rect_stroke(r, CornerRadius::same(2), stroke, egui::StrokeKind::Inside);
    painter.circle_stroke(r.min + egui::vec2(4.0, 3.5), 1.3, stroke);
    let pts = [
        r.min + egui::vec2(2.0, 9.0),
        r.min + egui::vec2(6.5, 4.5),
        r.min + egui::vec2(10.0, 8.0),
        r.min + egui::vec2(12.0, 6.0),
        r.min + egui::vec2(14.0, 9.0),
    ];
    painter.add(egui::Shape::line(pts.to_vec(), stroke));
}

pub fn paint_fonts(painter: &egui::Painter, rect: Rect, color: Color32) {
    if paint_online_icon(painter, rect, "type", color) {
        return;
    }
    let c = rect.center();
    let stroke = Stroke::new(1.6_f32, color);
    let top = c + egui::vec2(0.0, -5.5);
    let left = c + egui::vec2(-4.5, 5.0);
    let right = c + egui::vec2(4.5, 5.0);
    painter.line_segment([left, top], stroke);
    painter.line_segment([top, right], stroke);
    painter.line_segment([c + egui::vec2(-2.5, 1.2), c + egui::vec2(2.5, 1.2)], stroke);
    let s_stroke = Stroke::new(1.2_f32, color);
    painter.line_segment([left - egui::vec2(1.5, 0.0), left + egui::vec2(1.5, 0.0)], s_stroke);
    painter.line_segment([right - egui::vec2(1.5, 0.0), right + egui::vec2(1.5, 0.0)], s_stroke);
}

pub fn paint_displays(painter: &egui::Painter, rect: Rect, color: Color32) {
    if paint_online_icon(painter, rect, "monitor", color) {
        return;
    }
    let c = rect.center();
    let stroke = Stroke::new(1.4_f32, color);
    let screen = Rect::from_center_size(c + egui::vec2(0.0, -2.0), egui::vec2(15.0, 10.0));
    painter.rect_stroke(screen, CornerRadius::same(2), stroke, egui::StrokeKind::Inside);
    painter.line_segment([c + egui::vec2(0.0, 3.0), c + egui::vec2(0.0, 5.5)], stroke);
    painter.line_segment([c + egui::vec2(-4.0, 5.5), c + egui::vec2(4.0, 5.5)], stroke);
}

pub fn paint_tiling(painter: &egui::Painter, rect: Rect, color: Color32) {
    if paint_online_icon(painter, rect, "layout-panel-left", color) {
        return;
    }
    let c = rect.center();
    let stroke = Stroke::new(1.4_f32, color);
    let r = Rect::from_center_size(c, egui::vec2(14.0, 12.0));
    painter.rect_stroke(r, CornerRadius::same(2), stroke, egui::StrokeKind::Inside);
    let mid_x = r.left() + r.width() * 0.55;
    painter.line_segment([Pos2::new(mid_x, r.top()), Pos2::new(mid_x, r.bottom())], stroke);
    painter.line_segment([Pos2::new(mid_x, c.y), Pos2::new(r.right(), c.y)], stroke);
}

pub fn paint_animations(painter: &egui::Painter, rect: Rect, color: Color32) {
    if paint_online_icon(painter, rect, "sparkles", color) {
        return;
    }
    let c = rect.center() + egui::vec2(-1.5, -0.5);
    let stroke = Stroke::new(1.4_f32, color);
    let s = 5.0_f32;
    painter.line_segment([c - egui::vec2(s, 0.0), c + egui::vec2(s, 0.0)], stroke);
    painter.line_segment([c - egui::vec2(0.0, s), c + egui::vec2(0.0, s)], stroke);
    let c2 = c + egui::vec2(5.5, 4.0);
    let s2 = 2.5_f32;
    painter.line_segment([c2 - egui::vec2(s2, 0.0), c2 + egui::vec2(s2, 0.0)], stroke);
    painter.line_segment([c2 - egui::vec2(0.0, s2), c2 + egui::vec2(0.0, s2)], stroke);
}

pub fn paint_keyboard(painter: &egui::Painter, rect: Rect, color: Color32) {
    if paint_online_icon(painter, rect, "keyboard", color) {
        return;
    }
    let c = rect.center();
    let stroke = Stroke::new(1.4_f32, color);
    let r = Rect::from_center_size(c, egui::vec2(15.0, 11.0));
    painter.rect_stroke(r, CornerRadius::same(2), stroke, egui::StrokeKind::Inside);
    for dx in [-4.0_f32, -1.3, 1.3, 4.0] {
        painter.line_segment([c + egui::vec2(dx, -2.5), c + egui::vec2(dx, -2.0)], stroke);
    }
    painter.line_segment([c + egui::vec2(-3.5, 2.2), c + egui::vec2(3.5, 2.2)], stroke);
}

pub fn paint_window_rules(painter: &egui::Painter, rect: Rect, color: Color32) {
    if paint_online_icon(painter, rect, "layers", color) {
        return;
    }
    let c = rect.center();
    let stroke = Stroke::new(1.3_f32, color);
    let r1 = Rect::from_center_size(c + egui::vec2(2.0, -2.0), egui::vec2(10.0, 8.5));
    painter.rect_stroke(r1, CornerRadius::same(1), stroke, egui::StrokeKind::Inside);
    let r2 = Rect::from_center_size(c + egui::vec2(-2.0, 2.0), egui::vec2(10.0, 8.5));
    painter.rect_filled(r2, CornerRadius::same(1), color.gamma_multiply(0.2));
    painter.rect_stroke(r2, CornerRadius::same(1), stroke, egui::StrokeKind::Inside);
    painter.line_segment([Pos2::new(r2.left(), r2.top() + 2.5), Pos2::new(r2.right(), r2.top() + 2.5)], stroke);
}

pub fn paint_logs(painter: &egui::Painter, rect: Rect, color: Color32) {
    if paint_online_icon(painter, rect, "terminal", color) {
        return;
    }
    let c = rect.center();
    let stroke = Stroke::new(1.4_f32, color);
    let prompt_pts = [c + egui::vec2(-5.5, -3.5), c + egui::vec2(-2.5, -1.5), c + egui::vec2(-5.5, 0.5)];
    painter.add(egui::Shape::line(prompt_pts.to_vec(), stroke));
    painter.line_segment([c + egui::vec2(-0.5, -1.5), c + egui::vec2(5.5, -1.5)], stroke);
    painter.line_segment([c + egui::vec2(-5.5, 3.5), c + egui::vec2(5.5, 3.5)], stroke);
}

pub fn paint_clock(painter: &egui::Painter, rect: Rect, color: Color32) {
    if paint_online_icon(painter, rect, "clock", color) {
        return;
    }
    let c = rect.center();
    let r = (rect.width().min(rect.height()) * 0.42).max(5.0);
    let stroke = Stroke::new(1.4_f32, color);
    painter.circle_stroke(c, r, stroke);
    painter.line_segment([c, c + egui::vec2(-r * 0.45, -r * 0.4)], stroke);
    painter.line_segment([c, c + egui::vec2(r * 0.55, -r * 0.35)], stroke);
}

pub fn paint_battery(painter: &egui::Painter, rect: Rect, color: Color32, pct: u32, charging: bool) {
    let slug = if charging { "battery-charging" } else if pct > 60 { "battery-full" } else if pct > 20 { "battery-medium" } else { "battery-low" };
    if paint_online_icon(painter, rect, slug, color) {
        return;
    }
    let c = rect.center();
    let stroke = Stroke::new(1.3_f32, color);
    let body = Rect::from_center_size(c - egui::vec2(1.0, 0.0), egui::vec2(13.0, 8.0));
    painter.rect_stroke(body, CornerRadius::same(1), stroke, egui::StrokeKind::Inside);
    let pip = Rect::from_min_max(Pos2::new(body.right(), c.y - 2.0), Pos2::new(body.right() + 1.8, c.y + 2.0));
    painter.rect_filled(pip, CornerRadius::same(1), color);

    if charging {
        let pts = [
            c + egui::vec2(0.5, -3.5),
            c + egui::vec2(-1.5, 0.0),
            c + egui::vec2(0.0, 0.0),
            c + egui::vec2(-0.5, 3.5),
            c + egui::vec2(1.5, 0.0),
            c + egui::vec2(0.0, 0.0),
        ];
        painter.add(egui::Shape::line(pts.to_vec(), Stroke::new(1.4_f32, color)));
    } else {
        let fill_w = ((body.width() - 3.0) * (pct.clamp(0, 100) as f32 / 100.0)).max(1.0);
        let fill_r = Rect::from_min_size(body.min + egui::vec2(1.5, 1.5), egui::vec2(fill_w, body.height() - 3.0));
        painter.rect_filled(fill_r, CornerRadius::ZERO, color);
    }
}

pub fn paint_volume(painter: &egui::Painter, rect: Rect, color: Color32, muted: bool, pct: u32) {
    let slug = if muted || pct == 0 {
        "volume-x"
    } else if pct < 50 {
        "volume-1"
    } else {
        "volume-2"
    };
    if paint_online_icon(painter, rect, slug, color) {
        return;
    }
    let c = rect.center();
    let stroke = Stroke::new(1.4_f32, color);
    let left = c.x - 5.5;
    let horn = [
        Pos2::new(left, c.y - 2.0),
        Pos2::new(left + 2.5, c.y - 2.0),
        Pos2::new(left + 5.5, c.y - 5.0),
        Pos2::new(left + 5.5, c.y + 5.0),
        Pos2::new(left + 2.5, c.y + 2.0),
        Pos2::new(left, c.y + 2.0),
    ];
    painter.add(egui::Shape::closed_line(horn.to_vec(), stroke));
    if muted {
        let x = left + 7.5;
        painter.line_segment([Pos2::new(x, c.y - 3.5), Pos2::new(x + 5.0, c.y + 3.5)], stroke);
        painter.line_segment([Pos2::new(x, c.y + 3.5), Pos2::new(x + 5.0, c.y - 3.5)], stroke);
    }
}

pub fn paint_wifi(painter: &egui::Painter, rect: Rect, color: Color32, signal: u8) {
    let slug = match signal {
        0 => "wifi-off",
        1..=40 => "wifi-low",
        _ => "wifi",
    };
    if paint_online_icon(painter, rect, slug, color) {
        return;
    }
    let c = rect.center() + egui::vec2(0.0, 3.5);
    let stroke = Stroke::new(1.4_f32, color);
    painter.circle_filled(c, 1.3, color);
    let rings = match signal {
        0..=24 => 1,
        25..=60 => 2,
        _ => 3,
    };
    for i in 1..=rings {
        let r = i as f32 * 3.5;
        let n = 10;
        let pts = (0..=n).map(|step| {
            let a = -std::f32::consts::PI * 0.75 + (std::f32::consts::FRAC_PI_2) * (step as f32 / n as f32);
            c + egui::vec2(a.cos() * r, a.sin() * r)
        }).collect();
        painter.add(egui::Shape::line(pts, stroke));
    }
}

pub fn paint_ethernet(painter: &egui::Painter, rect: Rect, color: Color32) {
    if paint_online_icon(painter, rect, "network", color) {
        return;
    }
    let c = rect.center();
    let stroke = Stroke::new(1.3_f32, color);
    let r = Rect::from_center_size(c, egui::vec2(13.0, 10.0));
    painter.rect_stroke(r, CornerRadius::same(2), stroke, egui::StrokeKind::Inside);
    let notch = Rect::from_center_size(Pos2::new(c.x, r.bottom() - 1.5), egui::vec2(5.0, 3.0));
    painter.rect_stroke(notch, CornerRadius::same(1), stroke, egui::StrokeKind::Inside);
}

pub fn paint_bluetooth(painter: &egui::Painter, rect: Rect, color: Color32) {
    if paint_online_icon(painter, rect, "bluetooth", color) {
        return;
    }
    let c = rect.center();
    let stroke = Stroke::new(1.4_f32, color);
    let h = 6.0_f32;
    painter.line_segment([c - egui::vec2(0.0, h), c + egui::vec2(0.0, h)], stroke);
    let pts = [
        c + egui::vec2(-3.5, -3.5),
        c + egui::vec2(3.5, 3.5),
        c + egui::vec2(0.0, h),
        c + egui::vec2(0.0, -h),
        c + egui::vec2(3.5, -3.5),
        c + egui::vec2(-3.5, 3.5),
    ];
    painter.add(egui::Shape::line(pts.to_vec(), stroke));
}

pub fn paint_media_play(painter: &egui::Painter, rect: Rect, color: Color32) {
    if paint_online_icon(painter, rect, "play", color) {
        return;
    }
    let c = rect.center();
    let s = 4.5_f32;
    let pts = vec![c + egui::vec2(-s * 0.8, -s), c + egui::vec2(s * 1.2, 0.0), c + egui::vec2(-s * 0.8, s)];
    painter.add(egui::Shape::convex_polygon(pts, color, Stroke::NONE));
}

pub fn paint_media_pause(painter: &egui::Painter, rect: Rect, color: Color32) {
    if paint_online_icon(painter, rect, "pause", color) {
        return;
    }
    let c = rect.center();
    let stroke = Stroke::new(2.0_f32, color);
    painter.line_segment([c + egui::vec2(-2.5, -4.5), c + egui::vec2(-2.5, 4.5)], stroke);
    painter.line_segment([c + egui::vec2(2.5, -4.5), c + egui::vec2(2.5, 4.5)], stroke);
}

pub fn paint_media_prev(painter: &egui::Painter, rect: Rect, color: Color32) {
    if paint_online_icon(painter, rect, "skip-back", color) {
        return;
    }
    let c = rect.center();
    let stroke = Stroke::new(1.8_f32, color);
    painter.line_segment([c + egui::vec2(-4.0, -4.5), c + egui::vec2(-4.0, 4.5)], stroke);
    let pts = vec![c + egui::vec2(3.5, -4.0), c + egui::vec2(-2.5, 0.0), c + egui::vec2(3.5, 4.0)];
    painter.add(egui::Shape::convex_polygon(pts, color, Stroke::NONE));
}

pub fn paint_media_next(painter: &egui::Painter, rect: Rect, color: Color32) {
    if paint_online_icon(painter, rect, "skip-forward", color) {
        return;
    }
    let c = rect.center();
    let stroke = Stroke::new(1.8_f32, color);
    painter.line_segment([c + egui::vec2(4.0, -4.5), c + egui::vec2(4.0, 4.5)], stroke);
    let pts = vec![c + egui::vec2(-3.5, -4.0), c + egui::vec2(2.5, 0.0), c + egui::vec2(-3.5, 4.0)];
    painter.add(egui::Shape::convex_polygon(pts, color, Stroke::NONE));
}

pub fn paint_cpu(painter: &egui::Painter, rect: Rect, color: Color32) {
    if paint_online_icon(painter, rect, "cpu", color) {
        return;
    }
    let c = rect.center();
    let stroke = Stroke::new(1.3_f32, color);
    let r = Rect::from_center_size(c, egui::vec2(10.0, 10.0));
    painter.rect_stroke(r, CornerRadius::same(1), stroke, egui::StrokeKind::Inside);
}

pub fn paint_memory(painter: &egui::Painter, rect: Rect, color: Color32) {
    if paint_online_icon(painter, rect, "memory-stick", color) {
        return;
    }
    let c = rect.center();
    let stroke = Stroke::new(1.3_f32, color);
    let r = Rect::from_center_size(c, egui::vec2(14.0, 8.5));
    painter.rect_stroke(r, CornerRadius::same(1), stroke, egui::StrokeKind::Inside);
}

pub fn paint_bell(painter: &egui::Painter, rect: Rect, color: Color32, dnd: bool) {
    let slug = if dnd { "bell-off" } else { "bell" };
    if paint_online_icon(painter, rect, slug, color) {
        return;
    }
    let c = rect.center();
    let stroke = Stroke::new(1.3_f32, color);
    let pts = [
        c + egui::vec2(0.0, -5.0),
        c + egui::vec2(-3.5, -1.0),
        c + egui::vec2(-5.0, 3.0),
        c + egui::vec2(5.0, 3.0),
        c + egui::vec2(3.5, -1.0),
        c + egui::vec2(0.0, -5.0),
    ];
    painter.add(egui::Shape::closed_line(pts.to_vec(), stroke));
    painter.circle_filled(c + egui::vec2(0.0, 4.5), 1.2, color);
    if dnd {
        painter.line_segment([c + egui::vec2(-5.5, -4.5), c + egui::vec2(5.5, 4.5)], Stroke::new(1.6_f32, color));
    }
}

pub fn paint_download(painter: &egui::Painter, rect: Rect, color: Color32, active: bool) {
    let c = rect.center();
    if paint_online_icon(painter, rect, "download", color) {
        if active {
            painter.circle_filled(c + egui::vec2(4.5, -4.2), 1.6, color);
        }
        return;
    }
    let stroke = Stroke::new(1.4_f32, color);
    painter.line_segment([c + egui::vec2(0.0, -5.2), c + egui::vec2(0.0, 1.8)], stroke);
    painter.line_segment([c + egui::vec2(-3.2, -1.2), c + egui::vec2(0.0, 2.0)], stroke);
    painter.line_segment([c + egui::vec2(3.2, -1.2), c + egui::vec2(0.0, 2.0)], stroke);
    painter.line_segment([c + egui::vec2(-5.0, 2.2), c + egui::vec2(-5.0, 4.6)], stroke);
    painter.line_segment([c + egui::vec2(-5.0, 4.6), c + egui::vec2(5.0, 4.6)], stroke);
    painter.line_segment([c + egui::vec2(5.0, 4.6), c + egui::vec2(5.0, 2.2)], stroke);
    if active {
        painter.circle_filled(c + egui::vec2(4.5, -4.2), 1.6, color);
    }
}

pub fn paint_search(painter: &egui::Painter, rect: Rect, color: Color32) {
    if paint_online_icon(painter, rect, "search", color) {
        return;
    }
    let c = rect.center() + egui::vec2(-1.0, -1.0);
    let stroke = Stroke::new(1.5_f32, color);
    let r = 4.2;
    painter.circle_stroke(c, r, stroke);
    let handle_start = c + egui::vec2(3.0, 3.0);
    let handle_end = handle_start + egui::vec2(3.5, 3.5);
    painter.line_segment([handle_start, handle_end], Stroke::new(1.8_f32, color));
}

pub fn paint_launcher(painter: &egui::Painter, rect: Rect, color: Color32) {
    if paint_online_icon(painter, rect, "layout-grid", color) {
        return;
    }
    let c = rect.center();
    let s = 4.5_f32;
    let gap = 1.8_f32;
    for dy in [-s - gap * 0.5, gap * 0.5] {
        for dx in [-s - gap * 0.5, gap * 0.5] {
            let r = Rect::from_min_size(c + egui::vec2(dx, dy), egui::vec2(s, s));
            painter.rect_filled(r, CornerRadius::same(1), color);
        }
    }
}

pub fn paint_layout_master(painter: &egui::Painter, rect: Rect, color: Color32) {
    if paint_online_icon(painter, rect, "panel-left", color) {
        return;
    }
    let c = rect.center();
    let stroke = Stroke::new(1.4_f32, color);
    let r = Rect::from_center_size(c, egui::vec2(14.0, 12.0));
    painter.rect_stroke(r, CornerRadius::same(2), stroke, egui::StrokeKind::Inside);
    let mid_x = r.left() + r.width() * 0.55;
    painter.line_segment([Pos2::new(mid_x, r.top()), Pos2::new(mid_x, r.bottom())], stroke);
    painter.line_segment([Pos2::new(mid_x, c.y), Pos2::new(r.right(), c.y)], stroke);
}

pub fn paint_layout_dwindle(painter: &egui::Painter, rect: Rect, color: Color32) {
    if paint_online_icon(painter, rect, "layout-dashboard", color) {
        return;
    }
    let c = rect.center();
    let stroke = Stroke::new(1.4_f32, color);
    let r = Rect::from_center_size(c, egui::vec2(14.0, 12.0));
    painter.rect_stroke(r, CornerRadius::same(2), stroke, egui::StrokeKind::Inside);
    let mid_x = r.left() + r.width() * 0.5;
    painter.line_segment([Pos2::new(mid_x, r.top()), Pos2::new(mid_x, r.bottom())], stroke);
    painter.line_segment([Pos2::new(mid_x, c.y), Pos2::new(r.right(), c.y)], stroke);
    let sub_x = mid_x + (r.right() - mid_x) * 0.5;
    painter.line_segment([Pos2::new(sub_x, c.y), Pos2::new(sub_x, r.bottom())], stroke);
}

pub fn paint_window(painter: &egui::Painter, rect: Rect, color: Color32) {
    if paint_online_icon(painter, rect, "app-window", color) {
        return;
    }
    let c = rect.center();
    let stroke = Stroke::new(1.4_f32, color);
    let r = Rect::from_center_size(c, egui::vec2(14.0, 11.0));
    painter.rect_stroke(r, CornerRadius::same(2), stroke, egui::StrokeKind::Inside);
    painter.line_segment([Pos2::new(r.left(), r.top() + 3.0), Pos2::new(r.right(), r.top() + 3.0)], stroke);
    painter.circle_filled(Pos2::new(r.left() + 2.5, r.top() + 1.5), 0.8, color);
}

pub fn paint_git_branch(painter: &egui::Painter, rect: Rect, color: Color32) {
    if paint_online_icon(painter, rect, "git-branch", color) {
        return;
    }
    let c = rect.center();
    let stroke = Stroke::new(1.3_f32, color);
    let r = rect.width().min(rect.height()) * 0.42;
    let p_bottom = Pos2::new(c.x - r * 0.45, c.y + r * 0.55);
    let p_top = Pos2::new(c.x - r * 0.45, c.y - r * 0.55);
    let p_branch = Pos2::new(c.x + r * 0.45, c.y - r * 0.20);
    let node_r = (r * 0.28).clamp(1.5, 2.5);
    painter.line_segment([p_bottom, p_top], stroke);
    let fork_start = Pos2::new(p_bottom.x, c.y + r * 0.15);
    let ctrl1 = Pos2::new(c.x - r * 0.15, c.y + r * 0.10);
    let ctrl2 = Pos2::new(c.x + r * 0.20, c.y - r * 0.05);
    let shape = egui::epaint::CubicBezierShape::from_points_stroke(
        [fork_start, ctrl1, ctrl2, p_branch],
        false,
        Color32::TRANSPARENT,
        stroke,
    );
    painter.add(shape);
    painter.circle_filled(p_bottom, node_r, color);
    painter.circle_filled(p_top, node_r, color);
    painter.circle_filled(p_branch, node_r, color);
}

pub fn paint_control_center(painter: &egui::Painter, rect: Rect, color: Color32) {
    if paint_online_icon(painter, rect, "sliders-horizontal", color) {
        return;
    }
    let c = rect.center();
    let stroke = Stroke::new(1.2_f32, color);
    let pill_w = 14.0_f32;
    let pill_h = 5.2_f32;
    let r = CornerRadius::same(2);
    let top_rect = Rect::from_center_size(Pos2::new(c.x, c.y - 3.4), egui::vec2(pill_w, pill_h));
    painter.rect_stroke(top_rect, r, stroke, egui::StrokeKind::Inside);
    painter.circle_filled(Pos2::new(top_rect.right() - 3.2, top_rect.center().y), 1.6, color);
    let bot_rect = Rect::from_center_size(Pos2::new(c.x, c.y + 3.4), egui::vec2(pill_w, pill_h));
    painter.rect_stroke(bot_rect, r, stroke, egui::StrokeKind::Inside);
    painter.circle_filled(Pos2::new(bot_rect.left() + 3.2, bot_rect.center().y), 1.6, color);
}

pub fn paint_moon(painter: &egui::Painter, rect: Rect, color: Color32) {
    if paint_online_icon(painter, rect, "moon", color) {
        return;
    }
    let c = rect.center();
    let r = 5.5_f32;
    let mut pts = Vec::new();
    let steps = 16;
    for i in 0..=steps {
        let a = -std::f32::consts::FRAC_PI_2 + (std::f32::consts::PI * 1.25) * (i as f32 / steps as f32);
        pts.push(Pos2::new(c.x + a.cos() * r, c.y + a.sin() * r));
    }
    for i in (0..=steps).rev() {
        let a = -std::f32::consts::FRAC_PI_2 + (std::f32::consts::PI * 1.25) * (i as f32 / steps as f32);
        pts.push(Pos2::new(c.x + 2.4 + a.cos() * (r * 0.75), c.y + a.sin() * (r * 0.75)));
    }
    painter.add(egui::Shape::convex_polygon(pts, color, Stroke::NONE));
}

pub fn paint_sun(painter: &egui::Painter, rect: Rect, color: Color32) {
    if paint_online_icon(painter, rect, "sun", color) {
        return;
    }
    let c = rect.center();
    let stroke = Stroke::new(1.3_f32, color);
    let r = 3.2_f32;
    painter.circle_stroke(c, r, stroke);
    for i in 0..8 {
        let angle = i as f32 * (std::f32::consts::TAU / 8.0);
        let p1 = c + egui::vec2(angle.cos() * (r + 1.2), angle.sin() * (r + 1.2));
        let p2 = c + egui::vec2(angle.cos() * (r + 3.2), angle.sin() * (r + 3.2));
        painter.line_segment([p1, p2], stroke);
    }
}

pub fn paint_screen_mirror(painter: &egui::Painter, rect: Rect, color: Color32) {
    if paint_online_icon(painter, rect, "cast", color) {
        return;
    }
    let c = rect.center();
    let stroke = Stroke::new(1.3_f32, color);
    let r = CornerRadius::same(2);
    let back = Rect::from_center_size(Pos2::new(c.x + 2.0, c.y - 2.0), egui::vec2(10.0, 7.5));
    painter.rect_stroke(back, r, stroke, egui::StrokeKind::Inside);
    let front = Rect::from_center_size(Pos2::new(c.x - 2.0, c.y + 2.0), egui::vec2(10.0, 7.5));
    painter.rect_filled(front, r, Color32::from_black_alpha(160));
    painter.rect_stroke(front, r, stroke, egui::StrokeKind::Inside);
}

pub fn paint_airdrop(painter: &egui::Painter, rect: Rect, color: Color32) {
    if paint_online_icon(painter, rect, "radio", color) {
        return;
    }
    let c = rect.center() + egui::vec2(0.0, 1.0);
    let stroke = Stroke::new(1.2_f32, color);
    painter.circle_filled(c, 1.4, color);
}

pub fn paint_music(painter: &egui::Painter, rect: Rect, color: Color32) {
    if paint_online_icon(painter, rect, "music", color) {
        return;
    }
    let c = rect.center();
    let stroke = Stroke::new(1.4_f32, color);
    let n1 = Pos2::new(c.x - 3.5, c.y + 3.0);
    let n2 = Pos2::new(c.x + 3.5, c.y + 1.5);
    painter.circle_filled(n1, 2.0, color);
    painter.circle_filled(n2, 2.0, color);
    let top1 = Pos2::new(n1.x + 1.2, c.y - 4.5);
    let top2 = Pos2::new(n2.x + 1.2, c.y - 6.0);
    painter.line_segment([Pos2::new(n1.x + 1.2, n1.y), top1], stroke);
    painter.line_segment([Pos2::new(n2.x + 1.2, n2.y), top2], stroke);
    painter.line_segment([top1, top2], Stroke::new(2.2_f32, color));
}

pub fn paint_next(painter: &egui::Painter, rect: Rect, color: Color32) {
    if paint_online_icon(painter, rect, "skip-forward", color) {
        return;
    }
    let c = rect.center();
    let h = 5.0_f32;
    let t = vec![
        Pos2::new(c.x - 3.5, c.y - h),
        Pos2::new(c.x + 1.5, c.y),
        Pos2::new(c.x - 3.5, c.y + h),
    ];
    painter.add(egui::Shape::convex_polygon(t, color, Stroke::NONE));
    painter.line_segment([Pos2::new(c.x + 3.0, c.y - h), Pos2::new(c.x + 3.0, c.y + h)], Stroke::new(1.6_f32, color));
}

pub fn paint_magnet(painter: &egui::Painter, rect: Rect, color: Color32) {
    if paint_online_icon(painter, rect, "magnet", color) {
        return;
    }
    let c = rect.center() + egui::vec2(0.0, -1.0);
    let stroke = Stroke::new(1.6_f32, color);
    let r = 4.5_f32;
    let p_left = Pos2::new(c.x - r, c.y - 2.5);
    let p_right = Pos2::new(c.x + r, c.y - 2.5);
    let arc_l = Pos2::new(c.x - r, c.y + 1.0);
    let arc_r = Pos2::new(c.x + r, c.y + 1.0);
    painter.line_segment([p_left, arc_l], stroke);
    painter.line_segment([p_right, arc_r], stroke);
}

pub fn paint_close(painter: &egui::Painter, rect: Rect, color: Color32) {
    if paint_online_icon(painter, rect, "x", color) {
        return;
    }
    let c = rect.center();
    let s = (rect.width().min(rect.height()) * 0.28).max(3.5_f32);
    let stroke = Stroke::new(1.4_f32, color);
    painter.line_segment([c + egui::vec2(-s, -s), c + egui::vec2(s, s)], stroke);
    painter.line_segment([c + egui::vec2(s, -s), c + egui::vec2(-s, s)], stroke);
}

pub fn paint_trash(painter: &egui::Painter, rect: Rect, color: Color32) {
    if paint_online_icon(painter, rect, "trash-2", color) {
        return;
    }
    let c = rect.center();
    let stroke = Stroke::new(1.3_f32, color);
    painter.line_segment([c + egui::vec2(-5.5, -4.5), c + egui::vec2(5.5, -4.5)], stroke);
    painter.line_segment([c + egui::vec2(-2.0, -6.0), c + egui::vec2(2.0, -6.0)], stroke);
    let body = [
        c + egui::vec2(-4.2, -3.5),
        c + egui::vec2(-3.4, 5.5),
        c + egui::vec2(3.4, 5.5),
        c + egui::vec2(4.2, -3.5),
    ];
    painter.add(egui::Shape::closed_line(body.to_vec(), stroke));
}

pub fn paint_status_dot(painter: &egui::Painter, rect: Rect, color: Color32) {
    let c = rect.center();
    let r = (rect.width().min(rect.height()) * 0.25).clamp(2.5_f32, 4.0_f32);
    painter.circle_filled(c, r, color);
}

pub fn paint_circle_check(painter: &egui::Painter, rect: Rect, color: Color32) {
    if paint_online_icon(painter, rect, "circle-check", color) {
        return;
    }
    let c = rect.center();
    let r = (rect.width().min(rect.height()) * 0.42).max(4.5_f32);
    let stroke = Stroke::new(1.3_f32, color);
    painter.circle_stroke(c, r, stroke);
}

pub fn paint_warning(painter: &egui::Painter, rect: Rect, color: Color32) {
    if paint_online_icon(painter, rect, "triangle-alert", color) {
        return;
    }
    let c = rect.center();
    let stroke = Stroke::new(1.3_f32, color);
    let top = c + egui::vec2(0.0, -5.5);
    let bl = c + egui::vec2(-5.5, 4.5);
    let br = c + egui::vec2(5.5, 4.5);
    painter.add(egui::Shape::closed_line(vec![top, br, bl], stroke));
}

pub fn paint_file(painter: &egui::Painter, rect: Rect, color: Color32) {
    if paint_online_icon(painter, rect, "file", color) {
        return;
    }
    let c = rect.center();
    let stroke = Stroke::new(1.3_f32, color);
    let r = Rect::from_center_size(c, egui::vec2(9.0_f32, 12.0_f32));
    painter.rect_stroke(r, CornerRadius::same(1), stroke, egui::StrokeKind::Inside);
}

pub fn paint_chevron_down(painter: &egui::Painter, rect: Rect, color: Color32) {
    if paint_online_icon(painter, rect, "chevron-down", color) {
        return;
    }
    let c = rect.center();
    let stroke = Stroke::new(1.4_f32, color);
    let span = 3.5_f32;
    let y = c.y - 1.0_f32;
    painter.line_segment([Pos2::new(c.x - span, y), Pos2::new(c.x, y + span)], stroke);
    painter.line_segment([Pos2::new(c.x, y + span), Pos2::new(c.x + span, y)], stroke);
}

pub fn paint_minimize(painter: &egui::Painter, rect: Rect, color: Color32) {
    if paint_online_icon(painter, rect, "minus", color) {
        return;
    }
    let c = rect.center();
    let stroke = Stroke::new(1.3_f32, color);
    painter.line_segment([Pos2::new(c.x - 4.5_f32, c.y), Pos2::new(c.x + 4.5_f32, c.y)], stroke);
}

pub fn paint_maximize(painter: &egui::Painter, rect: Rect, color: Color32) {
    if paint_online_icon(painter, rect, "maximize-2", color) {
        return;
    }
    let c = rect.center();
    let stroke = Stroke::new(1.2_f32, color);
    let r = Rect::from_center_size(c, egui::vec2(9.0_f32, 9.0_f32));
    painter.rect_stroke(r, CornerRadius::same(1), stroke, egui::StrokeKind::Inside);
}

pub fn paint_link(painter: &egui::Painter, rect: Rect, color: Color32) {
    if paint_online_icon(painter, rect, "link", color) {
        return;
    }
    let c = rect.center();
    let stroke = Stroke::new(1.3_f32, color);
    let r = Rect::from_center_size(Pos2::new(c.x - 2.5_f32, c.y - 2.5_f32), egui::vec2(7.0_f32, 5.0_f32));
    painter.rect_stroke(r, CornerRadius::same(2), stroke, egui::StrokeKind::Inside);
}

pub fn paint_arrow_down(painter: &egui::Painter, rect: Rect, color: Color32) {
    if paint_online_icon(painter, rect, "arrow-down", color) {
        return;
    }
    let c = rect.center();
    let stroke = Stroke::new(1.3_f32, color);
    painter.line_segment([Pos2::new(c.x, c.y - 4.5_f32), Pos2::new(c.x, c.y + 4.5_f32)], stroke);
}

pub fn paint_arrow_up(painter: &egui::Painter, rect: Rect, color: Color32) {
    if paint_online_icon(painter, rect, "arrow-up", color) {
        return;
    }
    let c = rect.center();
    let stroke = Stroke::new(1.3_f32, color);
    painter.line_segment([Pos2::new(c.x, c.y + 4.5_f32), Pos2::new(c.x, c.y - 4.5_f32)], stroke);
}

pub fn paint_peer_swarm(painter: &egui::Painter, rect: Rect, color: Color32) {
    if paint_online_icon(painter, rect, "users", color) {
        return;
    }
    let c = rect.center();
    let stroke = Stroke::new(1.1_f32, color);
    painter.circle_stroke(c, 2.0_f32, stroke);
}




