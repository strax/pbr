use std::fmt::{Debug, Display};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use std::simd::StdFloat;
use std::str::FromStr;
use bytemuck::{Pod, Zeroable};

#[marker]
pub trait ClosedAdd<Rhs = Self>: Add<Rhs, Output = Self> + AddAssign<Rhs> {}
impl<T, Rhs> ClosedAdd<Rhs> for T where T: Add<Rhs, Output = Self> + AddAssign<Rhs> {}

#[marker]
pub trait ClosedSub<Rhs = Self>: Sub<Rhs, Output = Self> + SubAssign<Rhs> {}
impl<T, Rhs> ClosedSub<Rhs> for T where T: Sub<Rhs, Output = Self> + SubAssign<Rhs> {}

#[marker]
pub trait ClosedMul<Rhs = Self>: Mul<Rhs, Output = Self> + MulAssign<Rhs> {}
impl<T, Rhs> ClosedMul<Rhs> for T where T: Mul<Rhs, Output = Self> + MulAssign<Rhs> {}

#[marker]
pub trait ClosedDiv<Rhs = Self>: Div<Rhs, Output = Self> + DivAssign<Rhs> {}
impl<T, Rhs> ClosedDiv<Rhs> for T where T: Div<Rhs, Output = Self> + DivAssign<Rhs> {}

#[marker]
pub trait ClosedNeg: Neg<Output = Self> {}
impl<T> ClosedNeg for T where T: Neg<Output = Self> {}

#[doc(hidden)]
mod private {
    #[marker]
    pub unsafe trait PrimitiveMarker {}

    super::unsafe_impl_const!(PrimitiveMarker, bool, char, i8, u8, i16, u16, i32, u32, i64, u64, isize, usize, f32, f64);
}

macro unsafe_impl_const($trait_name:ty, $($t:ty),+) {
    $(
        unsafe impl const $trait_name for $t {}
    )+
}

#[marker]
pub unsafe trait Primitive: 'static + Copy + Clone + Send + Sync + Unpin + Debug + Default + FromStr + Display + private::PrimitiveMarker {}

unsafe_impl_const!(Primitive, bool, char, i8, u8, i16, u16, i32, u32, i64, u64, isize, usize, f32, f64);

/// Marker trait for signed integer types.
pub unsafe trait Signed {
    type Unsigned: Unsigned;

    fn abs(self) -> Self;
}

unsafe impl const Signed for i8 {
    type Unsigned = u8;

    fn abs(self) -> Self {
        i8::abs(self)
    }
}
unsafe impl const Signed for i16 {
    type Unsigned = u16;

    fn abs(self) -> Self {
        i16::abs(self)
    }
}
unsafe impl const Signed for i32 {
    type Unsigned = u32;

    fn abs(self) -> Self {
        i32::abs(self)
    }
}
unsafe impl const Signed for i64 {
    type Unsigned = u64;

    fn abs(self) -> Self {
        i64::abs(self)
    }
}
unsafe impl const Signed for isize {
    type Unsigned = usize;

    fn abs(self) -> Self {
        isize::abs(self)
    }
}
unsafe impl const Signed for i128 {
    type Unsigned = u128;

    fn abs(self) -> Self {
        i128::abs(self)
    }
}

pub unsafe trait Unsigned {
    type Signed: Signed;
}

unsafe impl const Unsigned for u8 {
    type Signed = i8;
}
unsafe impl const Unsigned for u16 {
    type Signed = i16;
}
unsafe impl const Unsigned for u32 {
    type Signed = i32;
}
unsafe impl const Unsigned for u64 {
    type Signed = i64;
}
unsafe impl const Unsigned for u128 {
    type Signed = i128;
}
unsafe impl const Unsigned for usize {
    type Signed = isize;
}

pub trait Field: Sized + ClosedAdd + ClosedSub + ClosedMul + ClosedDiv {}
impl<T: ClosedAdd + ClosedSub + ClosedMul + ClosedDiv> Field for T {}

pub trait Float: Field + Debug + PartialEq + PartialOrd + ClosedNeg + Copy + Clone + Default + Zeroable + Pod
{
    fn is_nan(self) -> bool;
    fn fract(self) -> Self;
    fn mul_add(self, a: Self, b: Self) -> Self;
    fn sqrt(self) -> Self;
    fn ceil(self) -> Self;
    fn floor(self) -> Self;
    fn round(self) -> Self;
    fn trunc(self) -> Self;
}

impl Float for f32 {
    fn is_nan(self) -> bool {
        f32::is_nan(self)
    }

    #[inline]
    fn fract(self) -> Self {
        f32::fract(self)
    }

    #[inline]
    fn mul_add(self, a: Self, b: Self) -> Self {
        f32::mul_add(self, a, b)
    }

    #[inline]
    fn sqrt(self) -> Self {
        f32::sqrt(self)
    }

    #[inline]
    fn ceil(self) -> Self {
        f32::ceil(self)
    }

    #[inline]
    fn floor(self) -> Self {
        f32::floor(self)
    }

    #[inline]
    fn round(self) -> Self {
        f32::round(self)
    }

    #[inline]
    fn trunc(self) -> Self {
        f32::trunc(self)
    }
}

impl Float for f64 {
    fn is_nan(self) -> bool {
        f64::is_nan(self)
    }

    #[inline]
    fn fract(self) -> Self {
        f64::fract(self)
    }

    #[inline]
    fn mul_add(self, a: Self, b: Self) -> Self {
        f64::mul_add(self, a, b)
    }

    #[inline]
    fn sqrt(self) -> Self {
        f64::sqrt(self)
    }

    #[inline]
    fn ceil(self) -> Self {
        f64::ceil(self)
    }

    #[inline]
    fn floor(self) -> Self {
        f64::floor(self)
    }

    #[inline]
    fn round(self) -> Self {
        f64::round(self)
    }

    #[inline]
    fn trunc(self) -> Self {
        f64::trunc(self)
    }
}

macro_rules! impl_bounded {
    ($($t:ident),+) => {
        $(
            impl Bounded for $t {
                const MIN: $t = $t::MIN;
                const MAX: $t = $t::MAX;
            }
        )+
    }
}

pub trait Bounded {
    const MIN: Self;
    const MAX: Self;
}

impl_bounded!(i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize, f32, f64);

#[marker]
pub trait Scalar: 'static + Copy + PartialEq + Debug {}

impl<T: 'static + Copy + PartialEq + Debug> Scalar for T {}