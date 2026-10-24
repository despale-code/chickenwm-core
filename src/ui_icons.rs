//! Vector-drawn monotone UI icons for ChickenWM applications.
//!
//! Provides clean, resolution-independent 2D vector primitives matching
//! theme foreground/accent colors without any icon font or emoji dependencies.

#![cfg(feature = "egui")]

use egui::{Color32, CornerRadius, Pos2, Rect, Stroke};

pub fn paint_gear(painter: &egui::Painter, rect: Rect, color: Color32) {
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
    let c = rect.center();
    let stroke = Stroke::new(1.4_f32, color);
    let r = Rect::from_center_size(c, egui::vec2(15.0, 13.0));
    painter.rect_stroke(r, CornerRadius::same(4), stroke, egui::StrokeKind::Inside);
    // 3 paint spots
    for dx in [-3.5_f32, 0.0, 3.5] {
        painter.circle_filled(c + egui::vec2(dx, -2.0), 1.2, color);
    }
    // Thumb well
    painter.circle_stroke(c + egui::vec2(2.5, 2.5), 1.5, stroke);
}

pub fn paint_wallpaper(painter: &egui::Painter, rect: Rect, color: Color32) {
    let c = rect.center();
    let stroke = Stroke::new(1.4_f32, color);
    let r = Rect::from_center_size(c, egui::vec2(15.0, 12.0));
    painter.rect_stroke(r, CornerRadius::same(2), stroke, egui::StrokeKind::Inside);
    // Sun
    painter.circle_stroke(r.min + egui::vec2(4.0, 3.5), 1.3, stroke);
    // Mountain peaks
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
    let c = rect.center();
    let stroke = Stroke::new(1.6_f32, color);
    // Capital A
    let top = c + egui::vec2(0.0, -5.5);
    let left = c + egui::vec2(-4.5, 5.0);
    let right = c + egui::vec2(4.5, 5.0);
    painter.line_segment([left, top], stroke);
    painter.line_segment([top, right], stroke);
    // Crossbar
    painter.line_segment([c + egui::vec2(-2.5, 1.2), c + egui::vec2(2.5, 1.2)], stroke);
    // Base serifs
    let s_stroke = Stroke::new(1.2_f32, color);
    painter.line_segment([left - egui::vec2(1.5, 0.0), left + egui::vec2(1.5, 0.0)], s_stroke);
    painter.line_segment([right - egui::vec2(1.5, 0.0), right + egui::vec2(1.5, 0.0)], s_stroke);
}

pub fn paint_displays(painter: &egui::Painter, rect: Rect, color: Color32) {
    let c = rect.center();
    let stroke = Stroke::new(1.4_f32, color);
    let screen = Rect::from_center_size(c + egui::vec2(0.0, -2.0), egui::vec2(15.0, 10.0));
    painter.rect_stroke(screen, CornerRadius::same(2), stroke, egui::StrokeKind::Inside);
    // Neck & stand
    painter.line_segment([c + egui::vec2(0.0, 3.0), c + egui::vec2(0.0, 5.5)], stroke);
    painter.line_segment([c + egui::vec2(-4.0, 5.5), c + egui::vec2(4.0, 5.5)], stroke);
}

pub fn paint_tiling(painter: &egui::Painter, rect: Rect, color: Color32) {
    let c = rect.center();
    let stroke = Stroke::new(1.4_f32, color);
    let r = Rect::from_center_size(c, egui::vec2(14.0, 12.0));
    painter.rect_stroke(r, CornerRadius::same(2), stroke, egui::StrokeKind::Inside);
    // Vertical split
    let mid_x = r.left() + r.width() * 0.55;
    painter.line_segment([Pos2::new(mid_x, r.top()), Pos2::new(mid_x, r.bottom())], stroke);
    // Right horizontal split
    painter.line_segment([Pos2::new(mid_x, c.y), Pos2::new(r.right(), c.y)], stroke);
}

pub fn paint_animations(painter: &egui::Painter, rect: Rect, color: Color32) {
    let c = rect.center() + egui::vec2(-1.5, -0.5);
    let stroke = Stroke::new(1.4_f32, color);
    // Main 4-point star
    let s = 5.0_f32;
    painter.line_segment([c - egui::vec2(s, 0.0), c + egui::vec2(s, 0.0)], stroke);
    painter.line_segment([c - egui::vec2(0.0, s), c + egui::vec2(0.0, s)], stroke);
    // Mini star
    let c2 = c + egui::vec2(5.5, 4.0);
    let s2 = 2.5_f32;
    painter.line_segment([c2 - egui::vec2(s2, 0.0), c2 + egui::vec2(s2, 0.0)], stroke);
    painter.line_segment([c2 - egui::vec2(0.0, s2), c2 + egui::vec2(0.0, s2)], stroke);
}

pub fn paint_keyboard(painter: &egui::Painter, rect: Rect, color: Color32) {
    let c = rect.center();
    let stroke = Stroke::new(1.4_f32, color);
    let r = Rect::from_center_size(c, egui::vec2(15.0, 11.0));
    painter.rect_stroke(r, CornerRadius::same(2), stroke, egui::StrokeKind::Inside);
    // Row 1 key dots
    for dx in [-4.0_f32, -1.3, 1.3, 4.0] {
        painter.line_segment([c + egui::vec2(dx, -2.5), c + egui::vec2(dx, -2.0)], stroke);
    }
    // Spacebar
    painter.line_segment([c + egui::vec2(-3.5, 2.2), c + egui::vec2(3.5, 2.2)], stroke);
}

pub fn paint_window_rules(painter: &egui::Painter, rect: Rect, color: Color32) {
    let c = rect.center();
    let stroke = Stroke::new(1.3_f32, color);
    // Back window
    let r1 = Rect::from_center_size(c + egui::vec2(2.0, -2.0), egui::vec2(10.0, 8.5));
    painter.rect_stroke(r1, CornerRadius::same(1), stroke, egui::StrokeKind::Inside);
    // Front window
    let r2 = Rect::from_center_size(c + egui::vec2(-2.0, 2.0), egui::vec2(10.0, 8.5));
    painter.rect_filled(r2, CornerRadius::same(1), color.gamma_multiply(0.2));
    painter.rect_stroke(r2, CornerRadius::same(1), stroke, egui::StrokeKind::Inside);
    // Front window titlebar
    painter.line_segment([Pos2::new(r2.left(), r2.top() + 2.5), Pos2::new(r2.right(), r2.top() + 2.5)], stroke);
}

pub fn paint_logs(painter: &egui::Painter, rect: Rect, color: Color32) {
    let c = rect.center();
    let stroke = Stroke::new(1.4_f32, color);
    // Terminal prompt
    let prompt_pts = [c + egui::vec2(-5.5, -3.5), c + egui::vec2(-2.5, -1.5), c + egui::vec2(-5.5, 0.5)];
    painter.add(egui::Shape::line(prompt_pts.to_vec(), stroke));
    // Log lines
    painter.line_segment([c + egui::vec2(-0.5, -1.5), c + egui::vec2(5.5, -1.5)], stroke);
    painter.line_segment([c + egui::vec2(-5.5, 3.5), c + egui::vec2(5.5, 3.5)], stroke);
}

pub fn paint_clock(painter: &egui::Painter, rect: Rect, color: Color32) {
    let c = rect.center();
    let r = (rect.width().min(rect.height()) * 0.42).max(5.0);
    let stroke = Stroke::new(1.4_f32, color);
    painter.circle_stroke(c, r, stroke);
    // Hands: hour to ~10 o'clock, minute to ~2 o'clock
    painter.line_segment([c, c + egui::vec2(-r * 0.45, -r * 0.4)], stroke);
    painter.line_segment([c, c + egui::vec2(r * 0.55, -r * 0.35)], stroke);
}

pub fn paint_battery(painter: &egui::Painter, rect: Rect, color: Color32, pct: u32, charging: bool) {
    let c = rect.center();
    let stroke = Stroke::new(1.3_f32, color);
    let body = Rect::from_center_size(c - egui::vec2(1.0, 0.0), egui::vec2(13.0, 8.0));
    painter.rect_stroke(body, CornerRadius::same(1), stroke, egui::StrokeKind::Inside);
    // Terminal pip
    let pip = Rect::from_min_max(Pos2::new(body.right(), c.y - 2.0), Pos2::new(body.right() + 1.8, c.y + 2.0));
    painter.rect_filled(pip, CornerRadius::same(1), color);

    if charging {
        // Lightning bolt
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
        // Charge level fill
        let fill_w = ((body.width() - 3.0) * (pct.clamp(0, 100) as f32 / 100.0)).max(1.0);
        let fill_r = Rect::from_min_size(body.min + egui::vec2(1.5, 1.5), egui::vec2(fill_w, body.height() - 3.0));
        painter.rect_filled(fill_r, CornerRadius::ZERO, color);
    }
}

pub fn paint_volume(painter: &egui::Painter, rect: Rect, color: Color32, muted: bool, pct: u32) {
    let c = rect.center();
    let stroke = Stroke::new(1.4_f32, color);
    // Speaker horn box + cone
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
        // Diagonal mute slash
        let x = left + 7.5;
        painter.line_segment([Pos2::new(x, c.y - 3.5), Pos2::new(x + 5.0, c.y + 3.5)], stroke);
        painter.line_segment([Pos2::new(x, c.y + 3.5), Pos2::new(x + 5.0, c.y - 3.5)], stroke);
    } else {
        // Sound waves
        let waves = if pct > 60 { 2 } else if pct > 0 { 1 } else { 0 };
        for i in 1..=waves {
            let r = i as f32 * 3.0;
            let center_arc = Pos2::new(left + 3.0, c.y);
            let n = 8;
            let arc_pts = (0..=n).map(|step| {
                let a = -std::f32::consts::FRAC_PI_4 + (std::f32::consts::FRAC_PI_2) * (step as f32 / n as f32);
                center_arc + egui::vec2(a.cos() * r, a.sin() * r)
            }).collect();
            painter.add(egui::Shape::line(arc_pts, stroke));
        }
    }
}

pub fn paint_wifi(painter: &egui::Painter, rect: Rect, color: Color32, signal: u8) {
    let c = rect.center() + egui::vec2(0.0, 3.5);
    let stroke = Stroke::new(1.4_f32, color);
    // Base dot
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
    let c = rect.center();
    let stroke = Stroke::new(1.3_f32, color);
    let r = Rect::from_center_size(c, egui::vec2(13.0, 10.0));
    painter.rect_stroke(r, CornerRadius::same(2), stroke, egui::StrokeKind::Inside);
    // Bottom notch
    let notch = Rect::from_center_size(Pos2::new(c.x, r.bottom() - 1.5), egui::vec2(5.0, 3.0));
    painter.rect_stroke(notch, CornerRadius::same(1), stroke, egui::StrokeKind::Inside);
}

pub fn paint_bluetooth(painter: &egui::Painter, rect: Rect, color: Color32) {
    let c = rect.center();
    let stroke = Stroke::new(1.4_f32, color);
    let h = 6.0_f32;
    // Central vertical
    painter.line_segment([c - egui::vec2(0.0, h), c + egui::vec2(0.0, h)], stroke);
    // Wings
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
    let c = rect.center();
    let s = 4.5_f32;
    let pts = vec![c + egui::vec2(-s * 0.8, -s), c + egui::vec2(s * 1.2, 0.0), c + egui::vec2(-s * 0.8, s)];
    painter.add(egui::Shape::convex_polygon(pts, color, Stroke::NONE));
}

pub fn paint_media_pause(painter: &egui::Painter, rect: Rect, color: Color32) {
    let c = rect.center();
    let stroke = Stroke::new(2.0_f32, color);
    painter.line_segment([c + egui::vec2(-2.5, -4.5), c + egui::vec2(-2.5, 4.5)], stroke);
    painter.line_segment([c + egui::vec2(2.5, -4.5), c + egui::vec2(2.5, 4.5)], stroke);
}

pub fn paint_media_prev(painter: &egui::Painter, rect: Rect, color: Color32) {
    let c = rect.center();
    let stroke = Stroke::new(1.8_f32, color);
    painter.line_segment([c + egui::vec2(-4.0, -4.5), c + egui::vec2(-4.0, 4.5)], stroke);
    let pts = vec![c + egui::vec2(3.5, -4.0), c + egui::vec2(-2.5, 0.0), c + egui::vec2(3.5, 4.0)];
    painter.add(egui::Shape::convex_polygon(pts, color, Stroke::NONE));
}

pub fn paint_media_next(painter: &egui::Painter, rect: Rect, color: Color32) {
    let c = rect.center();
    let stroke = Stroke::new(1.8_f32, color);
    painter.line_segment([c + egui::vec2(4.0, -4.5), c + egui::vec2(4.0, 4.5)], stroke);
    let pts = vec![c + egui::vec2(-3.5, -4.0), c + egui::vec2(2.5, 0.0), c + egui::vec2(-3.5, 4.0)];
    painter.add(egui::Shape::convex_polygon(pts, color, Stroke::NONE));
}

pub fn paint_cpu(painter: &egui::Painter, rect: Rect, color: Color32) {
    let c = rect.center();
    let stroke = Stroke::new(1.3_f32, color);
    let r = Rect::from_center_size(c, egui::vec2(10.0, 10.0));
    painter.rect_stroke(r, CornerRadius::same(1), stroke, egui::StrokeKind::Inside);
    // Inner die
    let die = Rect::from_center_size(c, egui::vec2(4.5, 4.5));
    painter.rect_filled(die, CornerRadius::ZERO, color.gamma_multiply(0.4));
    // Pins
    for d in [-2.5_f32, 2.5] {
        painter.line_segment([Pos2::new(c.x + d, r.top() - 2.0), Pos2::new(c.x + d, r.top())], stroke);
        painter.line_segment([Pos2::new(c.x + d, r.bottom()), Pos2::new(c.x + d, r.bottom() + 2.0)], stroke);
        painter.line_segment([Pos2::new(r.left() - 2.0, c.y + d), Pos2::new(r.left(), c.y + d)], stroke);
        painter.line_segment([Pos2::new(r.right(), c.y + d), Pos2::new(r.right() + 2.0, c.y + d)], stroke);
    }
}

pub fn paint_memory(painter: &egui::Painter, rect: Rect, color: Color32) {
    let c = rect.center();
    let stroke = Stroke::new(1.3_f32, color);
    let r = Rect::from_center_size(c, egui::vec2(14.0, 8.5));
    painter.rect_stroke(r, CornerRadius::same(1), stroke, egui::StrokeKind::Inside);
    // Chip pads
    for dx in [-3.5_f32, 0.0, 3.5] {
        let chip = Rect::from_center_size(Pos2::new(c.x + dx, c.y - 0.5), egui::vec2(2.2, 3.5));
        painter.rect_filled(chip, CornerRadius::ZERO, color.gamma_multiply(0.5));
    }
    // Bottom notch pin
    painter.line_segment([Pos2::new(c.x - 1.0, r.bottom()), Pos2::new(c.x - 1.0, r.bottom() - 1.5)], stroke);
}

pub fn paint_bell(painter: &egui::Painter, rect: Rect, color: Color32, dnd: bool) {
    let c = rect.center();
    let stroke = Stroke::new(1.3_f32, color);
    // Bell body
    let pts = [
        c + egui::vec2(0.0, -5.0),
        c + egui::vec2(-3.5, -1.0),
        c + egui::vec2(-5.0, 3.0),
        c + egui::vec2(5.0, 3.0),
        c + egui::vec2(3.5, -1.0),
        c + egui::vec2(0.0, -5.0),
    ];
    painter.add(egui::Shape::closed_line(pts.to_vec(), stroke));
    // Clapper dot
    painter.circle_filled(c + egui::vec2(0.0, 4.5), 1.2, color);

    if dnd {
        // Slash through bell
        painter.line_segment([c + egui::vec2(-5.5, -4.5), c + egui::vec2(5.5, 4.5)], Stroke::new(1.6_f32, color));
    }
}

pub fn paint_download(painter: &egui::Painter, rect: Rect, color: Color32, active: bool) {
    let c = rect.center();
    let stroke = Stroke::new(1.4_f32, color);
    // Vertical shaft
    painter.line_segment([c + egui::vec2(0.0, -5.2), c + egui::vec2(0.0, 1.8)], stroke);
    // Arrowhead
    painter.line_segment([c + egui::vec2(-3.2, -1.2), c + egui::vec2(0.0, 2.0)], stroke);
    painter.line_segment([c + egui::vec2(3.2, -1.2), c + egui::vec2(0.0, 2.0)], stroke);
    // Bottom tray bracket
    painter.line_segment([c + egui::vec2(-5.0, 2.2), c + egui::vec2(-5.0, 4.6)], stroke);
    painter.line_segment([c + egui::vec2(-5.0, 4.6), c + egui::vec2(5.0, 4.6)], stroke);
    painter.line_segment([c + egui::vec2(5.0, 4.6), c + egui::vec2(5.0, 2.2)], stroke);
    if active {
        painter.circle_filled(c + egui::vec2(4.5, -4.2), 1.6, color);
    }
}

pub fn paint_search(painter: &egui::Painter, rect: Rect, color: Color32) {
    let c = rect.center() + egui::vec2(-1.0, -1.0);
    let stroke = Stroke::new(1.5_f32, color);
    let r = 4.2;
    painter.circle_stroke(c, r, stroke);
    let handle_start = c + egui::vec2(3.0, 3.0);
    let handle_end = handle_start + egui::vec2(3.5, 3.5);
    painter.line_segment([handle_start, handle_end], Stroke::new(1.8_f32, color));
}

pub fn paint_launcher(painter: &egui::Painter, rect: Rect, color: Color32) {
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
    let c = rect.center();
    let stroke = Stroke::new(1.4_f32, color);
    let r = Rect::from_center_size(c, egui::vec2(14.0, 12.0));
    painter.rect_stroke(r, CornerRadius::same(2), stroke, egui::StrokeKind::Inside);
    let mid_x = r.left() + r.width() * 0.55;
    painter.line_segment([Pos2::new(mid_x, r.top()), Pos2::new(mid_x, r.bottom())], stroke);
    painter.line_segment([Pos2::new(mid_x, c.y), Pos2::new(r.right(), c.y)], stroke);
}

pub fn paint_layout_dwindle(painter: &egui::Painter, rect: Rect, color: Color32) {
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
    let c = rect.center();
    let stroke = Stroke::new(1.4_f32, color);
    let r = Rect::from_center_size(c, egui::vec2(14.0, 11.0));
    painter.rect_stroke(r, CornerRadius::same(2), stroke, egui::StrokeKind::Inside);
    painter.line_segment([Pos2::new(r.left(), r.top() + 3.0), Pos2::new(r.right(), r.top() + 3.0)], stroke);
    // Window control dot
    painter.circle_filled(Pos2::new(r.left() + 2.5, r.top() + 1.5), 0.8, color);
}

pub fn paint_git_branch(painter: &egui::Painter, rect: Rect, color: Color32) {
    let c = rect.center();
    let stroke = Stroke::new(1.3_f32, color);
    let r = rect.width().min(rect.height()) * 0.42;

    let p_bottom = Pos2::new(c.x - r * 0.45, c.y + r * 0.55);
    let p_top = Pos2::new(c.x - r * 0.45, c.y - r * 0.55);
    let p_branch = Pos2::new(c.x + r * 0.45, c.y - r * 0.20);

    let node_r = (r * 0.28).clamp(1.5, 2.5);

    // Main vertical trunk
    painter.line_segment([p_bottom, p_top], stroke);

    // Curved fork from trunk to branch node
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

    // Three branch nodes
    painter.circle_filled(p_bottom, node_r, color);
    painter.circle_filled(p_top, node_r, color);
    painter.circle_filled(p_branch, node_r, color);
}

/// macOS Control Center switch capsules icon (two stacked pill sliders with offset knobs).
pub fn paint_control_center(painter: &egui::Painter, rect: Rect, color: Color32) {
    let c = rect.center();
    let stroke = Stroke::new(1.2_f32, color);
    let pill_w = 14.0_f32;
    let pill_h = 5.2_f32;
    let r = CornerRadius::same(2);

    // Top slider (knob to the right)
    let top_rect = Rect::from_center_size(Pos2::new(c.x, c.y - 3.4), egui::vec2(pill_w, pill_h));
    painter.rect_stroke(top_rect, r, stroke, egui::StrokeKind::Inside);
    painter.circle_filled(Pos2::new(top_rect.right() - 3.2, top_rect.center().y), 1.6, color);

    // Bottom slider (knob to the left)
    let bot_rect = Rect::from_center_size(Pos2::new(c.x, c.y + 3.4), egui::vec2(pill_w, pill_h));
    painter.rect_stroke(bot_rect, r, stroke, egui::StrokeKind::Inside);
    painter.circle_filled(Pos2::new(bot_rect.left() + 3.2, bot_rect.center().y), 1.6, color);
}

/// Crescent moon icon for Do Not Disturb / Night mode.
pub fn paint_moon(painter: &egui::Painter, rect: Rect, color: Color32) {
    let c = rect.center();
    let r = 5.5_f32;
    // Outer arc and inner concave curve
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

/// Sun / display brightness icon.
pub fn paint_sun(painter: &egui::Painter, rect: Rect, color: Color32) {
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

/// Screen mirroring overlapping display monitors icon.
pub fn paint_screen_mirror(painter: &egui::Painter, rect: Rect, color: Color32) {
    let c = rect.center();
    let stroke = Stroke::new(1.3_f32, color);
    let r = CornerRadius::same(2);
    // Back display
    let back = Rect::from_center_size(Pos2::new(c.x + 2.0, c.y - 2.0), egui::vec2(10.0, 7.5));
    painter.rect_stroke(back, r, stroke, egui::StrokeKind::Inside);
    // Front display
    let front = Rect::from_center_size(Pos2::new(c.x - 2.0, c.y + 2.0), egui::vec2(10.0, 7.5));
    painter.rect_filled(front, r, Color32::from_black_alpha(160));
    painter.rect_stroke(front, r, stroke, egui::StrokeKind::Inside);
}

/// AirDrop / wireless broadcast concentric wave rings.
pub fn paint_airdrop(painter: &egui::Painter, rect: Rect, color: Color32) {
    let c = rect.center() + egui::vec2(0.0, 1.0);
    let stroke = Stroke::new(1.2_f32, color);
    painter.circle_filled(c, 1.4, color);
    for r in [3.4_f32, 5.6] {
        let start = -std::f32::consts::FRAC_PI_2 - 1.1;
        let sweep = 2.2_f32;
        let steps = 12;
        let arc: Vec<Pos2> = (0..=steps)
            .map(|i| {
                let a = start + sweep * (i as f32 / steps as f32);
                c + egui::vec2(a.cos() * r, a.sin() * r)
            })
            .collect();
        painter.add(egui::Shape::line(arc, stroke));
    }
}

/// Music note icon.
pub fn paint_music(painter: &egui::Painter, rect: Rect, color: Color32) {
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

/// Skip next track media playback button.
pub fn paint_next(painter: &egui::Painter, rect: Rect, color: Color32) {
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

/// Horseshoe magnet icon for Torrents.
pub fn paint_magnet(painter: &egui::Painter, rect: Rect, color: Color32) {
    let c = rect.center() + egui::vec2(0.0, -1.0);
    let stroke = Stroke::new(1.6_f32, color);
    let r = 4.5_f32;
    let p_left = Pos2::new(c.x - r, c.y - 2.5);
    let p_right = Pos2::new(c.x + r, c.y - 2.5);
    let arc_l = Pos2::new(c.x - r, c.y + 1.0);
    let arc_r = Pos2::new(c.x + r, c.y + 1.0);

    painter.line_segment([p_left, arc_l], stroke);
    painter.line_segment([p_right, arc_r], stroke);

    let steps = 14;
    let arc: Vec<Pos2> = (0..=steps)
        .map(|i| {
            let a = (i as f32 / steps as f32) * std::f32::consts::PI;
            Pos2::new(c.x + a.cos() * r, c.y + 1.0 + a.sin() * r)
        })
        .collect();
    painter.add(egui::Shape::line(arc, stroke));

    // Pole tips
    painter.line_segment([Pos2::new(p_left.x - 1.2, p_left.y), Pos2::new(p_left.x + 1.2, p_left.y)], Stroke::new(2.2_f32, color));
    painter.line_segment([Pos2::new(p_right.x - 1.2, p_right.y), Pos2::new(p_right.x + 1.2, p_right.y)], Stroke::new(2.2_f32, color));
}

/// Clean subtle 'X' close icon.
pub fn paint_close(painter: &egui::Painter, rect: Rect, color: Color32) {
    let c = rect.center();
    let s = (rect.width().min(rect.height()) * 0.28).max(3.5_f32);
    let stroke = Stroke::new(1.4_f32, color);
    painter.line_segment([c + egui::vec2(-s, -s), c + egui::vec2(s, s)], stroke);
    painter.line_segment([c + egui::vec2(s, -s), c + egui::vec2(-s, s)], stroke);
}

/// Wastebasket / trash can icon with lid and ribs.
pub fn paint_trash(painter: &egui::Painter, rect: Rect, color: Color32) {
    let c = rect.center();
    let stroke = Stroke::new(1.3_f32, color);
    // Lid horizontal line
    painter.line_segment([c + egui::vec2(-5.5, -4.5), c + egui::vec2(5.5, -4.5)], stroke);
    // Handle on lid
    painter.line_segment([c + egui::vec2(-2.0, -6.0), c + egui::vec2(2.0, -6.0)], stroke);
    // Can body
    let body = [
        c + egui::vec2(-4.2, -3.5),
        c + egui::vec2(-3.4, 5.5),
        c + egui::vec2(3.4, 5.5),
        c + egui::vec2(4.2, -3.5),
    ];
    painter.add(egui::Shape::closed_line(body.to_vec(), stroke));
    // Vertical ribs inside can
    painter.line_segment([c + egui::vec2(-1.4, -2.0), c + egui::vec2(-1.1, 3.8)], stroke);
    painter.line_segment([c + egui::vec2(1.4, -2.0), c + egui::vec2(1.1, 3.8)], stroke);
}

/// Status indicator dot.
pub fn paint_status_dot(painter: &egui::Painter, rect: Rect, color: Color32) {
    let c = rect.center();
    let r = (rect.width().min(rect.height()) * 0.25).clamp(2.5_f32, 4.0_f32);
    painter.circle_filled(c, r, color);
}

/// Circular checkmark badge.
pub fn paint_circle_check(painter: &egui::Painter, rect: Rect, color: Color32) {
    let c = rect.center();
    let r = (rect.width().min(rect.height()) * 0.42).max(4.5_f32);
    let stroke = Stroke::new(1.3_f32, color);
    painter.circle_stroke(c, r, stroke);
    // checkmark
    let p1 = c + egui::vec2(-r * 0.45, 0.0);
    let p2 = c + egui::vec2(-r * 0.08, r * 0.38);
    let p3 = c + egui::vec2(r * 0.45, -r * 0.35);
    painter.add(egui::Shape::line(vec![p1, p2, p3], stroke));
}

/// Warning triangle badge.
pub fn paint_warning(painter: &egui::Painter, rect: Rect, color: Color32) {
    let c = rect.center();
    let stroke = Stroke::new(1.3_f32, color);
    let top = c + egui::vec2(0.0, -5.5);
    let bl = c + egui::vec2(-5.5, 4.5);
    let br = c + egui::vec2(5.5, 4.5);
    painter.add(egui::Shape::closed_line(vec![top, br, bl], stroke));
    painter.line_segment([c + egui::vec2(0.0, -2.2), c + egui::vec2(0.0, 1.2)], stroke);
    painter.circle_filled(c + egui::vec2(0.0, 2.9), 0.9_f32, color);
}

/// Document / file icon with folded corner.
pub fn paint_file(painter: &egui::Painter, rect: Rect, color: Color32) {
    let c = rect.center();
    let stroke = Stroke::new(1.3_f32, color);
    let w = 9.0_f32;
    let h = 12.0_f32;
    let r = Rect::from_center_size(c, egui::vec2(w, h));
    let fold = 3.2_f32;
    let pts = vec![
        r.left_top(),
        Pos2::new(r.right() - fold, r.top()),
        Pos2::new(r.right(), r.top() + fold),
        r.right_bottom(),
        r.left_bottom(),
    ];
    painter.add(egui::Shape::closed_line(pts, stroke));
    painter.line_segment([Pos2::new(r.right() - fold, r.top()), Pos2::new(r.right() - fold, r.top() + fold)], stroke);
    painter.line_segment([Pos2::new(r.right() - fold, r.top() + fold), Pos2::new(r.right(), r.top() + fold)], stroke);
}

pub fn paint_chevron_down(painter: &egui::Painter, rect: Rect, color: Color32) {
    let c = rect.center();
    let stroke = Stroke::new(1.4_f32, color);
    let span = 3.5_f32;
    let y = c.y - 1.0_f32;
    painter.line_segment([Pos2::new(c.x - span, y), Pos2::new(c.x, y + span)], stroke);
    painter.line_segment([Pos2::new(c.x, y + span), Pos2::new(c.x + span, y)], stroke);
}

pub fn paint_minimize(painter: &egui::Painter, rect: Rect, color: Color32) {
    let c = rect.center();
    let stroke = Stroke::new(1.3_f32, color);
    painter.line_segment([Pos2::new(c.x - 4.5_f32, c.y), Pos2::new(c.x + 4.5_f32, c.y)], stroke);
}

pub fn paint_maximize(painter: &egui::Painter, rect: Rect, color: Color32) {
    let c = rect.center();
    let stroke = Stroke::new(1.2_f32, color);
    let r = Rect::from_center_size(c, egui::vec2(9.0_f32, 9.0_f32));
    painter.rect_stroke(r, CornerRadius::same(1), stroke, egui::StrokeKind::Inside);
}

pub fn paint_link(painter: &egui::Painter, rect: Rect, color: Color32) {
    let c = rect.center();
    let stroke = Stroke::new(1.3_f32, color);
    let r = Rect::from_center_size(Pos2::new(c.x - 2.5_f32, c.y - 2.5_f32), egui::vec2(7.0_f32, 5.0_f32));
    painter.rect_stroke(r, CornerRadius::same(2), stroke, egui::StrokeKind::Inside);
    let r2 = Rect::from_center_size(Pos2::new(c.x + 2.5_f32, c.y + 2.5_f32), egui::vec2(7.0_f32, 5.0_f32));
    painter.rect_stroke(r2, CornerRadius::same(2), stroke, egui::StrokeKind::Inside);
    painter.line_segment([Pos2::new(c.x - 1.5_f32, c.y), Pos2::new(c.x + 1.5_f32, c.y)], stroke);
}

pub fn paint_arrow_down(painter: &egui::Painter, rect: Rect, color: Color32) {
    let c = rect.center();
    let stroke = Stroke::new(1.3_f32, color);
    painter.line_segment([Pos2::new(c.x, c.y - 4.5_f32), Pos2::new(c.x, c.y + 4.5_f32)], stroke);
    painter.line_segment([Pos2::new(c.x - 3.0_f32, c.y + 1.5_f32), Pos2::new(c.x, c.y + 4.5_f32)], stroke);
    painter.line_segment([Pos2::new(c.x + 3.0_f32, c.y + 1.5_f32), Pos2::new(c.x, c.y + 4.5_f32)], stroke);
}

pub fn paint_arrow_up(painter: &egui::Painter, rect: Rect, color: Color32) {
    let c = rect.center();
    let stroke = Stroke::new(1.3_f32, color);
    painter.line_segment([Pos2::new(c.x, c.y + 4.5_f32), Pos2::new(c.x, c.y - 4.5_f32)], stroke);
    painter.line_segment([Pos2::new(c.x - 3.0_f32, c.y - 1.5_f32), Pos2::new(c.x, c.y - 4.5_f32)], stroke);
    painter.line_segment([Pos2::new(c.x + 3.0_f32, c.y - 1.5_f32), Pos2::new(c.x, c.y - 4.5_f32)], stroke);
}

pub fn paint_peer_swarm(painter: &egui::Painter, rect: Rect, color: Color32) {
    let c = rect.center();
    let stroke = Stroke::new(1.1_f32, color);
    let r = 2.0_f32;
    let p_left = Pos2::new(c.x - 4.5_f32, c.y);
    let p_top_r = Pos2::new(c.x + 3.5_f32, c.y - 4.0_f32);
    let p_bot_r = Pos2::new(c.x + 3.5_f32, c.y + 4.0_f32);
    painter.circle_stroke(p_left, r, stroke);
    painter.circle_stroke(p_top_r, r, stroke);
    painter.circle_stroke(p_bot_r, r, stroke);
    painter.line_segment([p_left, p_top_r], stroke);
    painter.line_segment([p_left, p_bot_r], stroke);
}



