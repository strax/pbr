use crate::bounds::Bounds3;
use crate::Ray;

pub trait Shape {
    fn object_bound(&self) -> Bounds3<f32>;
    fn world_bound(&self) -> Bounds3<f32>;
    fn intersect(&self, ray: &Ray, hit: *mut f32, /* surface_interaction: SurfaceInteraction, test_alpha_texture: bool */) -> bool;
    fn area(&self) -> f32;
}