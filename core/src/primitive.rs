use crate::{Bounds3f, Ray};
use crate::interaction::SurfaceInteraction;

pub trait Primitive {
    fn world_bound(&self) -> Bounds3f;
    fn intersect(&self, r: &Ray, interaction: &SurfaceInteraction) -> bool;
    fn intersect_p(&self, r: &Ray) -> bool;
    // fn area_light(&self) -> &dyn AreaLight;
    // fn material(&self) -> &dyn Material;
}