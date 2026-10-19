//! Hyprland-equivalent animation engine ported to ChickenWM (`chickenwm-core::animation`).
//! Implements `CAnimatedVariable<T>` supporting cubic Bézier curves, duration, retargeting,
//! and VBlank stepping.

use crate::config::BezierCurve;

pub trait Lerpable: Copy + Send + Sync + 'static {
    fn lerp(a: Self, b: Self, t: f32) -> Self;
}

impl Lerpable for f32 {
    #[inline]
    fn lerp(a: Self, b: Self, t: f32) -> Self {
        a + (b - a) * t
    }
}

impl Lerpable for [f32; 2] {
    #[inline]
    fn lerp(a: Self, b: Self, t: f32) -> Self {
        [a[0] + (b[0] - a[0]) * t, a[1] + (b[1] - a[1]) * t]
    }
}

impl Lerpable for [f32; 4] {
    #[inline]
    fn lerp(a: Self, b: Self, t: f32) -> Self {
        [
            a[0] + (b[0] - a[0]) * t,
            a[1] + (b[1] - a[1]) * t,
            a[2] + (b[2] - a[2]) * t,
            a[3] + (b[3] - a[3]) * t,
        ]
    }
}

/// Hyprland `CAnimatedVariable<T>` equivalent.
/// Smoothly animates a value from its current position to `target` using a `BezierCurve`.
#[derive(Debug, Clone, PartialEq)]
pub struct AnimatedVariable<T: Lerpable> {
    pub value: T,
    pub start: T,
    pub target: T,
    pub elapsed: f32,
    pub duration: f32,
    pub curve: BezierCurve,
    pub is_animating: bool,
}

impl<T: Lerpable> AnimatedVariable<T> {
    pub fn new(initial: T, curve: BezierCurve) -> Self {
        Self {
            value: initial,
            start: initial,
            target: initial,
            elapsed: 0.0,
            duration: 0.0,
            curve,
            is_animating: false,
        }
    }

    #[inline]
    pub fn value(&self) -> T {
        self.value
    }

    #[inline]
    pub fn target(&self) -> T {
        self.target
    }

    #[inline]
    pub fn is_animating(&self) -> bool {
        self.is_animating
    }

    /// Sets a new target destination with a specified duration and optional curve override.
    pub fn set_target(&mut self, new_target: T, duration: f32, curve: Option<BezierCurve>) {
        self.start = self.value;
        self.target = new_target;
        self.elapsed = 0.0;
        self.duration = duration.max(0.001);
        if let Some(c) = curve {
            self.curve = c;
        }
        self.is_animating = duration > 0.001;
    }

    /// Immediately sets the variable without animation.
    pub fn set_immediate(&mut self, value: T) {
        self.value = value;
        self.start = value;
        self.target = value;
        self.elapsed = 0.0;
        self.duration = 0.0;
        self.is_animating = false;
    }

    /// Steps the animation by `dt` seconds and returns the new value.
    pub fn step(&mut self, dt: f32) -> T {
        if !self.is_animating {
            return self.value;
        }
        self.elapsed += dt;
        if self.elapsed >= self.duration {
            self.value = self.target;
            self.is_animating = false;
        } else {
            let t = (self.elapsed / self.duration).clamp(0.0, 1.0);
            let progress = self.curve.sample(t);
            self.value = T::lerp(self.start, self.target, progress);
        }
        self.value
    }
}

/// Holds Hyprland's animated visual properties for a managed window.
#[derive(Debug, Clone)]
#[allow(non_snake_case)]
pub struct WindowVisualVariables {
    pub hwnd: u64,
    /// `m_vRealPosition`: Visual top-left coordinate `[x, y]`
    pub m_vRealPosition: AnimatedVariable<[f32; 2]>,
    /// `m_vRealSize`: Visual dimensions `[w, h]`
    pub m_vRealSize: AnimatedVariable<[f32; 2]>,
    /// `m_fAlpha`: Visual opacity `0.0..=1.0`
    pub m_fAlpha: AnimatedVariable<f32>,
    /// `m_fBorderAngle`: Animated border rotation angle in radians
    pub m_fBorderAngle: AnimatedVariable<f32>,
}

impl WindowVisualVariables {
    pub fn new(hwnd: u64, rect: [i32; 4], alpha: f32, default_curve: BezierCurve) -> Self {
        let pos = [rect[0] as f32, rect[1] as f32];
        let size = [rect[2] as f32, rect[3] as f32];
        Self {
            hwnd,
            m_vRealPosition: AnimatedVariable::new(pos, default_curve.clone()),
            m_vRealSize: AnimatedVariable::new(size, default_curve.clone()),
            m_fAlpha: AnimatedVariable::new(alpha, default_curve.clone()),
            m_fBorderAngle: AnimatedVariable::new(0.0, default_curve),
        }
    }

    pub fn is_animating(&self) -> bool {
        self.m_vRealPosition.is_animating()
            || self.m_vRealSize.is_animating()
            || self.m_fAlpha.is_animating()
            || self.m_fBorderAngle.is_animating()
    }

    pub fn step(&mut self, dt: f32) -> ([i32; 4], f32, f32) {
        let pos = self.m_vRealPosition.step(dt);
        let size = self.m_vRealSize.step(dt);
        let alpha = self.m_fAlpha.step(dt);
        let angle = self.m_fBorderAngle.step(dt);
        let rect = [
            pos[0].round() as i32,
            pos[1].round() as i32,
            size[0].round() as i32,
            size[1].round() as i32,
        ];
        (rect, alpha, angle)
    }
}
