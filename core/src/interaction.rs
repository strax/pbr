use std::cell::Cell;
use std::sync::Arc;
use crate::geom::Normal3;
use crate::{Normal3f, Point2f, Point3f, Ray, Vector3f};
use crate::shape::Shape;

pub trait Interaction {
    fn normal(&self) -> &Normal3f;

    fn is_surface_interaction(&self) -> bool {
        self.normal() == &Normal3::new(0.0, 0.0, 0.0)
    }
}

pub struct Shading {
    pub n: Normal3f,
    pub dpdu: Vector3f,
    pub dpdv: Vector3f,
    pub dndu: Normal3f,
    pub dndv: Normal3f
}

pub struct SurfaceInteraction {
    //#region Common Interaction fields
    pub p: Point3f,
    pub time: f32,
    pub p_error: Vector3f,
    pub wo: Vector3f,
    pub n: Normal3f,
    // pub medium: MediumInterface
    //#endregion
    pub uv: Point2f,
    pub dpdu: Vector3f,
    pub dpdv: Vector3f,
    pub dndu: Normal3f,
    pub dndv: Normal3f,
    pub shape: Option<Arc<dyn Shape>>,
    pub shading: Shading,
    // Primitive, BSDF, BSSRDF
    pub dpdx: Cell<Vector3f>,
    pub dpdy: Cell<Vector3f>,
    pub dud: Cell<(f32, f32)>,
    pub dvd: Cell<(f32, f32)>
}

impl Interaction for SurfaceInteraction {
    #[inline]
    fn normal(&self) -> &Normal3f {
        &self.n
    }


}