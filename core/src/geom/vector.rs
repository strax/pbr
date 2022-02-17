use std::ops::*;
use std::fmt::{self, Debug, Formatter};
use crate::math::Abs;
use crate::macros::{replace_expr, forward_binop, strip_plus};
use crate::types::{Scalar, Float};
use super::{Point2, Point3};

#[cfg(test)]
use quickcheck::{Arbitrary, Gen};
use crate::geom::DotProduct;

macro_rules! impl_vector_scaling_ops {
    ($name:ident, $($field:ident),+) => {
        //#region Scalar multiplication
        impl<T> const Mul<T> for $name<T> where T: ~const Copy + ~const Mul<Output=T> {
            type Output = $name<T>;

            #[inline]
            fn mul(self, rhs: T) -> Self::Output {
                Self {
                    $(
                       $field: Mul::mul(self.$field, rhs)
                    ),+
                }
            }
        }

        impl<'a, T> const Mul<T> for &'a $name<T> where T: ~const Copy, &'a T: ~const Mul<T, Output=T> {
            type Output = $name<T>;

            #[inline]
            fn mul(self, rhs: T) -> Self::Output {
                $name {
                    $(
                        $field: Mul::mul(&self.$field, rhs)
                    ),+
                }
            }
        }

        impl<'a, T> const Mul<&'a T> for $name<T> where T: ~const Drop + ~const Mul<&'a T, Output=T> {
            type Output = $name<T>;

            #[inline]
            fn mul(self, rhs: &'a T) -> Self::Output {
                $name {
                    $(
                        $field: Mul::mul(self.$field, rhs)
                    ),+
                }
            }
        }

        impl<'a, T> const Mul<&'a T> for &'a $name<T> where &'a T: ~const Mul<&'a T, Output=T> {
            type Output = $name<T>;

            #[inline]
            fn mul(self, rhs: &'a T) -> Self::Output {
                $name {
                    $(
                        $field: Mul::mul(&self.$field, rhs)
                    ),+
                }
            }
        }

        impl<T> const MulAssign<T> for $name<T> where T: ~const Copy + ~const MulAssign {
            #[inline]
            fn mul_assign(&mut self, rhs: T) {
                $(
                    MulAssign::mul_assign(&mut self.$field, rhs);
                )+
            }
        }
        impl<'a, T> const MulAssign<&'a T> for $name<T> where T: ~const MulAssign<&'a T> {
            #[inline]
            fn mul_assign(&mut self, rhs: &'a T) {
                $(
                    MulAssign::mul_assign(&mut self.$field, rhs);
                )+
            }
        }
        //#endregion

        impl<T> const Div<T> for $name<T> where T: ~const Copy + ~const Div<Output=T> {
            type Output = $name<T>;

            #[inline]
            fn div(self, rhs: T) -> Self::Output {
                Self {
                    $(
                       $field: Div::div(self.$field, rhs)
                    ),+
                }
            }
        }

        impl<'a, T> const Div<T> for &'a $name<T> where T: ~const Copy, &'a T: ~const Div<T, Output=T> {
            type Output = $name<T>;

            #[inline]
            fn div(self, rhs: T) -> Self::Output {
                $name {
                    $(
                        $field: Div::div(&self.$field, rhs)
                    ),+
                }
            }
        }

        impl<'a, T> const Div<&'a T> for $name<T> where T: ~const Drop + ~const Div<&'a T, Output=T> {
            type Output = $name<T>;

            #[inline]
            fn div(self, rhs: &'a T) -> Self::Output {
                $name {
                    $(
                        $field: Div::div(self.$field, rhs)
                    ),+
                }
            }
        }

        impl<'a, T> const Div<&'a T> for &'a $name<T> where &'a T: ~const Div<&'a T, Output=T> {
            type Output = $name<T>;

            #[inline]
            fn div(self, rhs: &'a T) -> Self::Output {
                $name {
                    $(
                        $field: Div::div(&self.$field, rhs)
                    ),+
                }
            }
        }

        impl<T> const DivAssign<T> for $name<T> where T: ~const Copy + ~const DivAssign {
            #[inline]
            fn div_assign(&mut self, rhs: T) {
                $(
                    DivAssign::div_assign(&mut self.$field, rhs);
                )+
            }
        }
        impl<'a, T> const DivAssign<&'a T> for $name<T> where T: ~const DivAssign<&'a T> {
            #[inline]
            fn div_assign(&mut self, rhs: &'a T) {
                $(
                    DivAssign::div_assign(&mut self.$field, rhs);
                )+
            }
        }
    }
}

macro_rules! impl_vector_struct {
    ($name:ident { $($field:ident),+ }) => {
        #[repr(C)]
        #[derive(Eq, PartialEq, Ord, PartialOrd, Default, Copy, Clone, Hash)]
        pub struct $name<T> {
            $(
                pub $field: T
            ),+
        }

        #[cfg(test)]
        impl<T: Arbitrary> Arbitrary for $name<T> {
            fn arbitrary(g: &mut Gen) -> Self {
                Self::new($(replace_expr!($field => Arbitrary::arbitrary(g))),+)
            }
        }

        unsafe impl<T: bytemuck::Zeroable> bytemuck::Zeroable for $name<T> {}
        unsafe impl<T: bytemuck::Pod> bytemuck::Pod for $name<T> {}

        impl<T: Debug> Debug for $name<T> {
            fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                let mut d = f.debug_tuple(stringify!($name));
                $(
                    d.field(&self.$field);
                )+
                d.finish()
            }
        }

        impl<T> $name<T> {
            pub const fn new($($field: T),+) -> Self {
                trait NewSpec<T> {
                    fn new($($field: T),+) -> Self;
                }

                impl<T> const NewSpec<T> for $name<T> {
                    #[inline]
                    default fn new($($field: T),+) -> Self {
                        Self {
                            $($field),+
                        }
                    }
                }

                impl const NewSpec<f32> for $name<f32> {
                    #[inline]
                    fn new($($field: f32),+) -> Self {
                        $(
                            assert!(!$field.is_nan());
                        )+
                        Self {
                            $($field),+
                        }
                    }
                }

                impl const NewSpec<f64> for $name<f64> {
                    #[inline]
                    fn new($($field: f64),+) -> Self {
                        $(
                            debug_assert!(!$field.is_nan());
                        )+
                        Self {
                            $($field),+
                        }
                    }
                }

                NewSpec::new($($field),+)
            }
        }

        impl<T> const Add for $name<T> where T: ~const Drop + ~const Add<Output=T> {
            type Output = $name<T>;

            #[inline]
            fn add(self, rhs: $name<T>) -> Self::Output {
                Self {
                    $(
                        $field: Add::add(self.$field, rhs.$field)
                    ),+
                }
            }
        }
        forward_binop!(Add::add for $name, $($field),+);

        impl<T> const AddAssign for $name<T> where T: ~const Drop + ~const AddAssign {
            #[inline]
            fn add_assign(&mut self, rhs: $name<T>) {
                $(
                    AddAssign::add_assign(&mut self.$field, rhs.$field);
                )+
            }
        }

        impl<'a, T> const AddAssign<&'a $name<T>> for $name<T> where T: ~const AddAssign<&'a T> {
            #[inline]
            fn add_assign(&mut self, rhs: &'a $name<T>) {
                $(
                    AddAssign::add_assign(&mut self.$field, &rhs.$field);
                )+
            }
        }

        impl<T> const Sub for $name<T> where T: ~const Drop + ~const Sub<Output=T> {
            type Output = $name<T>;

            #[inline]
            fn sub(self, rhs: $name<T>) -> Self::Output {
                Self {
                    $(
                       $field: Sub::sub(self.$field, rhs.$field)
                    ),+
                }
            }
        }
        forward_binop!(Sub::sub for $name, $($field),+);

        impl<T> const SubAssign for $name<T> where T: ~const Drop + ~const SubAssign {
            #[inline]
            fn sub_assign(&mut self, rhs: $name<T>) {
                $(
                    SubAssign::sub_assign(&mut self.$field, rhs.$field);
                )+
            }
        }

        impl<'a, T> const SubAssign<&'a $name<T>> for $name<T> where T: ~const SubAssign<&'a T> {
            #[inline]
            fn sub_assign(&mut self, rhs: &'a $name<T>) {
                $(
                    SubAssign::sub_assign(&mut self.$field, &rhs.$field);
                )+
            }
        }

        impl_vector_scaling_ops!($name, $($field),+);

        impl<T> const Neg for $name<T> where T: ~const Drop + ~const Neg<Output=T> {
            type Output = $name<T>;

            #[inline]
            fn neg(self) -> Self::Output {
                Self::new($(Neg::neg(self.$field)),+)
            }
        }

        impl<'a, T> const Neg for &'a $name<T> where &'a T: ~const Neg<Output=T> {
            type Output = $name<T>;

            #[inline]
            fn neg(self) -> Self::Output {
                $name::new($(Neg::neg(&self.$field)),+)
            }
        }

        impl<T> $name<T> {
            #[inline]
            pub fn abs(self) -> Self where T: Abs {
                Self::new($(self.$field.abs()),+)
            }
        }

        impl<T: Float> $name<T> {
            /// Returns the L2 (Euclidean) norm of this vector.
            #[inline]
            pub fn length(self) -> T {
                (strip_plus!($(+ (self.$field * self.$field))+)).sqrt()
            }

            #[inline]
            pub fn normalize(self) -> $name<T> {
                self / self.length()
            }
        }
    }
}

impl_vector_struct!(Vector2 { x, y });
impl_vector_struct!(Vector3 { x, y, z });

impl<T: ~const Drop> const From<(T, T)> for Vector2<T> {
    #[inline]
    fn from(src: (T, T)) -> Self {
        Vector2::new(src.0, src.1)
    }
}

impl<T: Scalar + ~const Add<Output=T> + ~const Mul<Output=T>> const DotProduct for Vector2<T> {
    type Output = T;

    fn dot(&self, rhs: &Self) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y
    }
}
impl<T: Scalar + ~const Add<Output=T> + ~const Mul<Output=T>> const DotProduct for Vector3<T> {
    type Output = T;

    fn dot(&self, rhs: &Self) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

impl<T> Vector3<T> {
    #[inline]
    pub fn cross(self, rhs: &Self) -> Self where T: Copy + Mul<Output=T> + Sub<Output=T> {
        // Specialize `cross` to increase accuracy for f32
        trait CrossSpec<T> where T: Copy + Mul<Output=T> + Sub<Output=T> {
            fn cross(self, rhs: &Self) -> Self;
        }
        
        impl<T> CrossSpec<T> for Vector3<T> where T: Copy + Mul<Output=T> + Sub<Output=T> {
            #[inline]
            default fn cross(self, rhs: &Self) -> Self {
                Vector3::new(
                    (self.y * rhs.z) - (self.z * rhs.y),
                    (self.z * rhs.x) - (self.x * rhs.z),
                    (self.x * rhs.y) - (self.y * rhs.x)
                )
            }
        }

        impl CrossSpec<f32> for Vector3<f32> {
            #[inline]
            fn cross(self, rhs: &Self) -> Self {
                // Cast to f64 for intermediate computations to avoid catastrophic cancellation
                let (x0, y0, z0) = (self.x as f64, self.y as f64, self.z as f64);
                let (x1, y1, z1) = (rhs.x as f64, rhs.y as f64, rhs.z as f64);
                Vector3::new(
                    ((y0 * z1) - (z0 * y1)) as f32,
                    ((z0 * x1) - (x0 * z1)) as f32,
                    ((x0 * y1) - (y0 * x1)) as f32
                )
            }
        }

        CrossSpec::cross(self, rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::*;
    use quickcheck_macros::quickcheck;

    #[quickcheck]
    fn vector_addition_1(x0: f64, y0: f64, x1: f64, y1: f64) -> TestResult {
        if !x0.is_finite() || !y0.is_finite() || !x1.is_finite() || !y1.is_finite() {
            return TestResult::discard()
        }
        TestResult::from_bool(
            Vector2::new(x0, y0) + Vector2::new(x1, y1) == Vector2::new(x0 + x1, y0 + y1)
        )
    }

    #[quickcheck]
    fn vector3_f32_cross_product_anticommutative(x0: f32, y0: f32, z0: f32, x1: f32, y1: f32, z1: f32) -> TestResult {
        if !x0.is_finite() || !y0.is_finite() || !z0.is_finite() || !x1.is_finite() || !y1.is_finite() || !z1.is_finite() {
            return TestResult::discard()
        }
        let v0 = Vector3::new(x0, y0, z0);
        let v1 = Vector3::new(x1, y1, z1);
        TestResult::from_bool(
            v0.cross(&v1) == (-v1).cross(&v0)
        )
    }

    #[test]
    fn test_vector2_new() {
        let v = Vector2::new(1i32, 2i32);
        assert_eq!(v.x, 1);
        assert_eq!(v.y, 2);
    }

    #[test]
    fn test_vector2_add() {
        let v = Vector2::new(1i32, 2i32);
        let u = Vector2::new(3i32, 4i32);
        assert_eq!(&v + &u, Vector2::new(4, 6));
    }
}