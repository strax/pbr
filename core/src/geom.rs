use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

mod vector;
mod point;
mod macros;
mod normal;

pub use vector::*;
pub use point::*;
pub use normal::*;

pub trait DotProduct<Rhs = Self> {
    type Output;

    fn dot(&self, rhs: &Rhs) -> Self::Output;
}
