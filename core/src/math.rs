use crate::types::*;

mod matrix4x4;
pub use matrix4x4::*;

pub trait Abs {
    fn abs(self) -> Self;
}

impl<T: Signed> Abs for T {
    fn abs(self) -> Self {
        Signed::abs(self)
    }
}

impl Abs for f32 {
    fn abs(self) -> Self {
        f32::abs(self)
    }
}

impl Abs for f64 {
    fn abs(self) -> Self {
        f64::abs(self)
    }
}

pub trait Lerp {
    fn lerp(t: Self, v1: Self, v2: Self) -> Self;
}

impl Lerp for f32 {
    #[inline]
    fn lerp(t: f32, v0: f32, v1: f32) -> f32 {
        if cfg!(target_feature = "fma") {
            // v0 + t * (v1 - v0)
            f32::mul_add(t, v1 - v0, v0)
        } else {
            (1.0 - t) * v0 + t * v1
        }
    }
}

impl Lerp for f64 {
    #[inline]
    fn lerp(t: f64, v0: f64, v1: f64) -> f64 {
        if cfg!(target_feature = "fma") {
            // v0 + t * (v1 - v0)
            f64::mul_add(t, v1 - v0, v0)
        } else {
            (1.0 - t) * v0 + t * v1
        }
    }
}