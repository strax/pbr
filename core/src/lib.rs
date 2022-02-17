#![feature(const_trait_impl)]
#![feature(const_fn_trait_bound)]
#![feature(const_replace)]
#![feature(const_mut_refs)]
#![feature(const_swap)]
#![feature(min_specialization)]
#![feature(rustc_attrs)]
#![feature(marker_trait_attr)]
#![feature(decl_macro)]
#![feature(portable_simd)]
#![feature(const_convert)]
#![feature(inline_const)]
#![feature(concat_idents)]
#![feature(const_float_classify)]
#![feature(const_fn_floating_point_arithmetic)]
#![feature(unboxed_closures)]
#![feature(fn_traits)]

extern crate core;

pub mod math;
pub mod geom;
pub mod types;
pub mod shape;
pub mod bounds;
pub mod transform;
pub mod spectrum;
pub mod ray;
pub mod interaction;
pub mod primitive;

mod macros;

pub use ray::*;
pub use spectrum::*;
pub use geom::{Vector3, Vector2, Point2, Point3};
pub use transform::Transform;
pub use interaction::Interaction;
pub use primitive::*;

pub use types::{Scalar, Bounded};
use crate::bounds::Bounds3;
use crate::geom::Normal3;

pub type Vector2f = Vector2<f32>;
pub type Vector2i = Vector2<i32>;

pub type Vector3f = Vector3<f32>;
pub type Vector3i = Vector3<i32>;

pub type Point2f = Point2<f32>;
pub type Point2i = Point2<i32>;

pub type Point3f = Point3<f32>;
pub type Point3i = Point3<i32>;

pub type Bounds3f = Bounds3<f32>;
pub type Bounds3i = Bounds3<i32>;

pub type Normal3f = Normal3<f32>;
pub type Normal3i = Normal3<i32>;

#[inline]
pub const fn vec3<T: Scalar>(x: T, y: T, z: T) -> Vector3<T> {
    Vector3::new(x, y, z)
}

#[inline]
pub const fn point2<T: Scalar>(x: T, y: T) -> Point2<T> {
    Point2::new(x, y)
}