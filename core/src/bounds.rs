use std::cmp;
use std::fmt::Debug;
use crate::{Bounded, Scalar, Vector3};
use crate::math::Lerp;
use crate::types::Field;
use crate::{Point2, Point3};

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
pub struct Bounds2<T: Scalar> {
    pub min: Point2<T>,
    pub max: Point2<T>
}

impl<T: Scalar + Ord> Bounds2<T> {
    pub fn new(p1: Point2<T>, p2: Point2<T>) -> Self {
        let min = Point2::new(cmp::min(p1.x, p2.x), cmp::min(p1.y, p2.y));
        let max = Point2::new(cmp::max(p1.x, p2.x), cmp::max(p1.x, p2.y));
        Bounds2 { min, max }
    }

    pub fn area(&self) -> T where T: Field {
        let d = self.max - self.min;
        d.x * d.y
    }
}

impl<T: Scalar + Bounded> const Default for Bounds2<T> {
    #[inline]
    fn default() -> Self {
        let min = Point2::new(T::MIN, T::MIN);
        let max = Point2::new(T::MAX, T::MAX);
        Bounds2 { min, max }
    }
}

impl<T: Scalar> From<Point2<T>> for Bounds2<T> {
    #[inline]
    fn from(p: Point2<T>) -> Self {
        Bounds2 { min: p.clone(), max: p }
    }
}

impl<T: Scalar> From<(Point2<T>, Point2<T>)> for Bounds2<T> {
    #[inline]
    fn from((min, max): (Point2<T>, Point2<T>)) -> Self {
        Bounds2 { min, max }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
pub struct Bounds3<T: Scalar> {
    pub min: Point3<T>,
    pub max: Point3<T>
}

impl<T: Scalar + Bounded> const Default for Bounds3<T> {
    #[inline]
    fn default() -> Self {
        let min = Point3::new(T::MIN, T::MIN, T::MIN);
        let max = Point3::new(T::MAX, T::MAX, T::MAX);
        Bounds3 { min, max }
    }
}

impl<T: Scalar + Ord + Field> Bounds3<T> {
    pub fn new(p1: Point3<T>, p2: Point3<T>) -> Self {
        let min = Point3::new(cmp::min(p1.x, p2.x), cmp::min(p1.y, p2.y), cmp::min(p1.z, p2.z));
        let max = Point3::new(cmp::max(p1.x, p2.x), cmp::max(p1.x, p2.y), cmp::max(p1.z, p2.z));
        Bounds3 { min, max }
    }

    pub const fn diagonal(&self) -> Vector3<T> {
        self.max - self.min
    }

    pub fn volume(&self) -> T {
        let d = self.diagonal();
        d.x * d.y * d.z
    }

    pub fn surface_area(&self) -> T {
        todo!()
    }

    pub fn maximum_extent(&self) -> Axis3 {
        let d = self.diagonal();
        if d.x > d.y && d.x > d.z {
            Axis3::X
        } else if d.y > d.z {
            Axis3::Y
        } else {
            Axis3::Z
        }
    }

    pub fn lerp(&self, t: &Point3<T>) -> Point3<T> where T: Lerp {
        Point3::new(
            Lerp::lerp(t.x, self.min.x, self.max.x),
            Lerp::lerp(t.y, self.min.y, self.max.y),
            Lerp::lerp(t.z, self.min.z, self.max.z)
        )
    }
}


impl<T: Scalar> From<Point3<T>> for Bounds3<T> {
    #[inline]
    fn from(p: Point3<T>) -> Self {
        Bounds3 { min: p.clone(), max: p }
    }
}

impl<T: Scalar> From<(Point3<T>, Point3<T>)> for Bounds3<T> {
    #[inline]
    fn from((min, max): (Point3<T>, Point3<T>)) -> Self {
        Bounds3 { min, max }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
pub enum Axis3 {
    X, Y, Z
}