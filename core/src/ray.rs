use std::cell::Cell;
use crate::{Point3, Vector3};

#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub struct Ray {
    pub o: Point3<f32>,
    pub d: Vector3<f32>,
    pub tmax: Cell<f32>,
    pub time: f32,
    // medium
}

impl const Default for Ray {
    fn default() -> Self {
        Ray {
            o: Point3::new(0.0, 0.0, 0.0),
            d: Vector3::new(0.0, 0.0, 0.0),
            tmax: Cell::new(f32::INFINITY),
            time: 0.0
        }
    }
}

impl Ray {
    pub const fn new(o: Point3<f32>, d: Vector3<f32>) -> Self {
        Self { o, d, ..Default::default() }
    }

    pub fn at(&self, t: f32) -> Point3<f32> {
        self.o + self.d * t
    }
}