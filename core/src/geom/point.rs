use std::fmt::{Debug, Formatter};
use std::ops::{Add, Sub};
use bytemuck::{Pod, Zeroable};
use crate::{Scalar, Vector2, Vector3};
use crate::types::Float;

macro_rules! impl_point_new {
    ($Point:ident { $($field:ident),+ }) => {
        impl<T: Scalar> $Point<T> {
            pub const fn new($($field: T),+) -> Self {
                trait NewSpec<T: Scalar> {
                    fn new($($field: T),+) -> Self;
                }
                impl<T: Scalar> const NewSpec<T> for $Point<T> {
                    #[inline]
                    default fn new($($field: T),+) -> Self {
                        $Point { $($field),+ }
                    }
                }
                impl const NewSpec<f32> for $Point<f32> {
                    #[inline]
                    fn new($($field: f32),+) -> Self {
                        $(
                            debug_assert!(!$field.is_nan());
                        )+
                        $Point { $($field),+ }
                    }
                }
                impl const NewSpec<f64> for $Point<f64> {
                    #[inline]
                    fn new($($field: f64),+) -> Self {
                        $(
                            debug_assert!(!$field.is_nan());
                        )+
                        $Point { $($field),+ }
                    }
                }

                NewSpec::new($($field),+)
            }
        }
    }
}

macro_rules! impl_point {
    ($Point:ident { $($field:ident),+ }) => {
        #[repr(C)]
        #[derive(PartialOrd, PartialEq, Ord, Eq, Copy, Clone, Default, Hash)]
        pub struct $Point<T: Scalar> {
            $(pub $field: T),+
        }

        impl_point_new!($Point { $($field),+ });

        unsafe impl<T: Scalar + Zeroable> Zeroable for $Point<T> {}
        unsafe impl<T: Scalar + Pod> Pod for $Point<T> {}

        impl<T: Scalar> Debug for $Point<T> {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                f.debug_tuple(stringify!($Point))
                    $(.field(&self.$field))+
                    .finish()
            }
        }
    }
}

impl_point!(Point2 { x, y });
impl_point!(Point3 { x, y, z });

impl<T: Scalar + ~const Add<Output=T>> const Add<Vector2<T>> for Point2<T> {
    type Output = Point2<T>;

    fn add(self, rhs: Vector2<T>) -> Self::Output {
        Point2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T: Scalar + ~const Sub<Output=T>> const Sub<Point2<T>> for Point2<T> {
    type Output = Vector2<T>;

    fn sub(self, rhs: Point2<T>) -> Self::Output {
        Vector2::new(self.x - rhs.x, self.y - rhs.y)
    }
}
impl<'a, T> const Sub<&'a Point2<T>> for &'a Point2<T> where T: Scalar, &'a T: ~const Sub<Output=T> {
    type Output = Vector2<T>;

    fn sub(self, rhs: &'a Point2<T>) -> Self::Output {
        Vector2::new(&self.x - &rhs.x, &self.y - &rhs.y)
    }
}

impl<T: Float> Point2<T> {
    pub fn distance(p1: &Point2<T>, p2: &Point2<T>) -> T {
        (*p1 - *p2).length()
    }
}

impl<T: Scalar + ~const Add<Output=T>> const Add<Vector3<T>> for Point3<T> {
    type Output = Point3<T>;

    fn add(self, rhs: Vector3<T>) -> Self::Output {
        Point3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl<T: Scalar + ~const Sub<Output=T>> const Sub<Vector3<T>> for Point3<T> {
    type Output = Point3<T>;

    fn sub(self, rhs: Vector3<T>) -> Self::Output {
        Point3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}
impl<'a, T> const Sub<&'a Point3<T>> for &'a Point3<T> where T: Scalar, &'a T: ~const Sub<Output=T> {
    type Output = Vector3<T>;

    fn sub(self, rhs: &'a Point3<T>) -> Self::Output {
        Vector3::new(&self.x - &rhs.x, &self.y - &rhs.y, &self.z - &rhs.z)
    }
}

impl<T: Scalar + ~const Sub<Output=T>> const Sub<Point3<T>> for Point3<T> {
    type Output = Vector3<T>;

    fn sub(self, rhs: Point3<T>) -> Self::Output {
        Vector3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl<T: Float> Point3<T> {
    pub fn distance(p1: &Point3<T>, p2: &Point3<T>) -> T {
        (*p1 - *p2).length()
    }
}

impl<T: Scalar + ~const Drop> const From<Point3<T>> for Point2<T> {
    fn from(p: Point3<T>) -> Self {
        Point2::new(p.x, p.y)
    }
}

impl<T: Scalar> const From<(T, T)> for Point2<T> {
    fn from((x, y): (T, T)) -> Self {
        Point2::new(x, y)
    }
}

impl<T: Scalar> const From<(T, T, T)> for Point3<T> {
    fn from((x, y, z): (T, T, T)) -> Self {
        Point3::new(x, y, z)
    }
}

impl<T: Scalar> const From<Point2<T>> for (T, T) {
    fn from(p: Point2<T>) -> Self {
        (p.x, p.y)
    }
}
impl<T: Scalar> const From<Point3<T>> for (T, T, T) {
    fn from(p: Point3<T>) -> Self {
        (p.x, p.y, p.x)
    }
}