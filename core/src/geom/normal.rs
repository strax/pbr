use std::fmt;
use std::ops::*;
use crate::{Scalar, Vector3};
use bytemuck::{Zeroable, Pod};
#[cfg(test)]
use quickcheck::{Arbitrary, Gen};
use crate::geom::DotProduct;
use crate::types::Float;

use super::macros::forward_binop_generic;

#[repr(C)]
#[derive(Eq, PartialEq, Ord, PartialOrd, Default, Copy, Clone, Hash)]
pub struct Normal3<T: Scalar> {
    pub x: T,
    pub y: T,
    pub z: T
}

impl<T: Scalar> fmt::Debug for Normal3<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Normal3").field(&self.x).field(&self.y).field(&self.z).finish()
    }
}

unsafe impl<T: Scalar + Zeroable> Zeroable for Normal3<T> {}
unsafe impl<T: Scalar + Pod> Pod for Normal3<T> {}

#[cfg(test)]
impl<T: Scalar + Arbitrary> Arbitrary for Normal3<T> {
    fn arbitrary(g: &mut Gen) -> Self {
        Normal3::new(T::arbitrary(g), T::arbitrary(g), T::arbitrary(g))
    }
}

impl<T: Scalar> Normal3<T> {
    #[inline]
    pub const fn new(x: T, y: T, z: T) -> Normal3<T> {
        trait NewSpec<T: Scalar> {
            fn new(x: T, y: T, z: T) -> Self;
        }
        impl<T: Scalar> const NewSpec<T> for Normal3<T> {
            #[inline]
            default fn new(x: T, y: T, z: T) -> Normal3<T> {
                Normal3 { x, y, z }
            }
        }
        impl const NewSpec<f32> for Normal3<f32> {
            #[inline]
            fn new(x: f32, y: f32, z: f32) -> Normal3<f32> {
                debug_assert!(!x.is_nan());
                debug_assert!(!y.is_nan());
                debug_assert!(!z.is_nan());
                Normal3 { x, y, z }
            }
        }
        impl const NewSpec<f64> for Normal3<f64> {
            #[inline]
            fn new(x: f64, y: f64, z: f64) -> Normal3<f64> {
                debug_assert!(!x.is_nan());
                debug_assert!(!y.is_nan());
                debug_assert!(!z.is_nan());
                Normal3 { x, y, z }
            }
        }
        NewSpec::new(x, y, z)
    }
}

impl<T: Scalar> const From<Vector3<T>> for Normal3<T> {
    #[inline]
    fn from(v: Vector3<T>) -> Self {
        Self::new(v.x, v.y, v.z)
    }
}

//#region Operators
impl<T: Scalar + ~const Add<Output=T>> const Add for Normal3<T> {
    type Output = Normal3<T>;

    #[inline]
    fn add(self, rhs: Normal3<T>) -> Self::Output {
        Normal3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}
forward_binop_generic!(Add::add for Normal3);

impl<T: Scalar + ~const Sub<Output=T>> const Sub for Normal3<T> {
    type Output = Normal3<T>;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Normal3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}
forward_binop_generic!(Sub::sub for Normal3);

impl<T: Scalar + ~const Mul<Output=T>> const Mul<T> for Normal3<T> {
    type Output = Normal3<T>;

    #[inline]
    fn mul(self, rhs: T) -> Self::Output {
        Normal3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}
impl<'a, T: Scalar + ~const Mul<Output=T>> const Mul<T> for &'a Normal3<T> {
    type Output = Normal3<T>;

    #[inline]
    fn mul(self, rhs: T) -> Self::Output {
        Mul::mul(*self, rhs)
    }
}

impl<T: Scalar + ~const Div<Output=T>> const Div<T> for Normal3<T> {
    type Output = Normal3<T>;

    #[inline]
    fn div(self, rhs: T) -> Self::Output {
        Normal3::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}
impl<'a, T: Scalar + ~const Div<Output=T>> const Div<T> for &'a Normal3<T> {
    type Output = Normal3<T>;

    #[inline]
    fn div(self, rhs: T) -> Self::Output {
        Div::div(*self, rhs)
    }
}

impl<T: Scalar + ~const Neg<Output=T>> const Neg for Normal3<T> {
    type Output = Normal3<T>;

    #[inline]
    fn neg(self) -> Self::Output {
        Normal3::new(-self.x, -self.y, -self.z)
    }
}

impl<T: Scalar + ~const AddAssign<T>> const AddAssign for Normal3<T> {
    #[inline]
    fn add_assign(&mut self, rhs: Normal3<T>) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}
impl<'a, T: Scalar + ~const AddAssign<T>> const AddAssign<&'a Normal3<T>> for Normal3<T> {
    #[inline]
    fn add_assign(&mut self, rhs: &'a Normal3<T>) {
        AddAssign::add_assign(self, *rhs)
    }
}

impl<T: Scalar + ~const SubAssign<T>> const SubAssign for Normal3<T> {
    #[inline]
    fn sub_assign(&mut self, rhs: Normal3<T>) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}
impl<'a, T: Scalar + ~const SubAssign<T>> const SubAssign<&'a Normal3<T>> for Normal3<T> {
    #[inline]
    fn sub_assign(&mut self, rhs: &'a Normal3<T>) {
        SubAssign::sub_assign(self, *rhs)
    }
}

impl<T: Scalar + ~const MulAssign<T>> const MulAssign for Normal3<T> {
    #[inline]
    fn mul_assign(&mut self, rhs: Normal3<T>) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl<T: Scalar + ~const DivAssign<T>> const DivAssign for Normal3<T> {
    #[inline]
    fn div_assign(&mut self, rhs: Normal3<T>) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
    }
}



//#endregion

impl<T: Float> Normal3<T> {
    #[inline]
    pub fn length(&self) -> T {
        T::sqrt(self.x * self.x + self.y * self.y + self.z * self.z)
    }

    pub fn normalize(&self) -> Normal3<T> {
        self / self.length()
    }
}

impl<T: Scalar + ~const Add<Output=T> + ~const Mul<Output=T>> const DotProduct for Normal3<T> {
    type Output = T;

    #[inline]
    fn dot(&self, rhs: &Self) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

impl<T: Scalar + ~const Add<Output=T> + ~const Mul<Output=T>> const DotProduct<Vector3<T>> for Normal3<T> {
    type Output = T;

    #[inline]
    fn dot(&self, rhs: &Vector3<T>) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

impl<T: Scalar + ~const Add<Output=T> + ~const Mul<Output=T>> const DotProduct<Normal3<T>> for Vector3<T> {
    type Output = T;

    #[inline]
    fn dot(&self, rhs: &Normal3<T>) -> Self::Output {
        rhs.dot(self)
    }
}

impl<T: Scalar> Normal3<T> {
    pub const fn dot<U>(&self, rhs: &U) -> T where Self: ~const DotProduct<U, Output=T> {
        DotProduct::dot(self, rhs)
    }
}

