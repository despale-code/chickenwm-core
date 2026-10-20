//! Shared window framing and edge resizing for borderless ChickenWM applications.
//!
//! Provides:
//! - 1-pixel border with vertical gradient: `#2b2b2b` (top) to `#1f1f1f` (bottom).
//! - Window edge resize handles for all 8 directions (North, South, East, West, corners).

#[cfg(feature = "egui")]
use eframe::egui::{self, Color32, CursorIcon, Pos2, Rect, ResizeDirection, Stroke, Vec2};

#[cfg(feature = "egui")]
pub fn paint_window_gradient_border(painter: &egui::Painter, screen: Rect, radius: f32) {
    let r = radius.clamp(0.0, screen.height() * 0.5).min(screen.width() * 0.5);
    let rect = screen.shrink(0.5);
    let top_y = rect.top();
    let bot_y = rect.bottom();
    let h = (bot_y - top_y).max(1.0);

    let mut pts = Vec::with_capacity(32);
    let steps = 6;
    let pi = std::f32::consts::PI;

    // Top-left arc (from PI to 1.5*PI)
    let tl = Pos2::new(rect.left() + r, rect.top() + r);
    for i in 0..=steps {
        let a = pi + (i as f32 / steps as f32) * (pi * 0.5);
        pts.push(tl + Vec2::new(a.cos() * r, a.sin() * r));
    }
    // Top-right arc (from -0.5*PI to 0)
    let tr = Pos2::new(rect.right() - r, rect.top() + r);
    for i in 0..=steps {
        let a = -pi * 0.5 + (i as f32 / steps as f32) * (pi * 0.5);
        pts.push(tr + Vec2::new(a.cos() * r, a.sin() * r));
    }
    // Bottom-right arc (from 0 to 0.5*PI)
    let br = Pos2::new(rect.right() - r, rect.bottom() - r);
    for i in 0..=steps {
        let a = (i as f32 / steps as f32) * (pi * 0.5);
        pts.push(br + Vec2::new(a.cos() * r, a.sin() * r));
    }
    // Bottom-left arc (from 0.5*PI to PI)
    let bl = Pos2::new(rect.left() + r, rect.bottom() - r);
    for i in 0..=steps {
        let a = pi * 0.5 + (i as f32 / steps as f32) * (pi * 0.5);
        pts.push(bl + Vec2::new(a.cos() * r, a.sin() * r));
    }

    let n = pts.len();
    for i in 0..n {
        let p1 = pts[i];
        let p2 = pts[(i + 1) % n];
        let mid_y = (p1.y + p2.y) * 0.5;
        let t = ((mid_y - top_y) / h).clamp(0.0, 1.0);
        // Gradient from #2b2b2b (43, 43, 43) at top to #1f1f1f (31, 31, 31) at bottom
        let val = (43.0 + (31.0 - 43.0) * t).round() as u8;
        let col = Color32::from_rgb(val, val, val);
        painter.line_segment([p1, p2], Stroke::new(1.0_f32, col));
    }
}

#[cfg(feature = "egui")]
pub fn handle_window_edge_resize(ctx: &egui::Context, screen: Rect, is_maximized: bool) {
    if is_maximized {
        return;
    }
    let Some(pointer) = ctx.input(|i| i.pointer.latest_pos()) else { return };
    if !screen.expand(3.0).contains(pointer) {
        return;
    }

    let border = 6.0_f32;
    let corner = 14.0_f32;

    let on_left = pointer.x >= screen.left() - 3.0 && pointer.x < screen.left() + border;
    let on_right = pointer.x <= screen.right() + 3.0 && pointer.x > screen.right() - border;
    let on_top = pointer.y >= screen.top() - 3.0 && pointer.y < screen.top() + border;
    let on_bottom = pointer.y <= screen.bottom() + 3.0 && pointer.y > screen.bottom() - border;

    let dir = if on_top && pointer.x < screen.left() + corner {
        Some((ResizeDirection::NorthWest, CursorIcon::ResizeNorthWest))
    } else if on_top && pointer.x > screen.right() - corner {
        Some((ResizeDirection::NorthEast, CursorIcon::ResizeNorthEast))
    } else if on_bottom && pointer.x < screen.left() + corner {
        Some((ResizeDirection::SouthWest, CursorIcon::ResizeSouthWest))
    } else if on_bottom && pointer.x > screen.right() - corner {
        Some((ResizeDirection::SouthEast, CursorIcon::ResizeSouthEast))
    } else if on_left {
        Some((ResizeDirection::West, CursorIcon::ResizeWest))
    } else if on_right {
        Some((ResizeDirection::East, CursorIcon::ResizeEast))
    } else if on_top {
        Some((ResizeDirection::North, CursorIcon::ResizeNorth))
    } else if on_bottom {
        Some((ResizeDirection::South, CursorIcon::ResizeSouth))
    } else {
        None
    };

    if let Some((resize_dir, cursor)) = dir {
        ctx.set_cursor_icon(cursor);
        if ctx.input(|i| i.pointer.any_pressed()) {
            ctx.send_viewport_cmd(egui::ViewportCommand::BeginResize(resize_dir));
        }
    }
}
