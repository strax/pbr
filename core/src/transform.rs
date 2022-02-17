use crate::math::Matrix4x4;
use crate::{Vector3, Point3};

pub struct Transform {
    forward: Matrix4x4,
    inverse: Matrix4x4,
}

impl Transform {
    pub fn new(forward: Matrix4x4) -> Self {
        let inverse = forward
            .inverse()
            .expect("transformation matrix is singular");
        Self { forward, inverse }
    }

    /// Creates a new transform from the transformation matrix and its inverse.
    ///
    /// # Safety
    ///
    /// The caller guarantees that `inverse` is the inverse matrix of `forward`, e.g. `inverse * forward == Matrix4x4::identity()`.
    pub const unsafe fn with_inverse_unchecked(forward: Matrix4x4, inverse: Matrix4x4) -> Self {
        Self { forward, inverse }
    }

    pub const fn inverse(&self) -> Self {
        unsafe { Self::with_inverse_unchecked(self.inverse, self.forward) }
    }

    pub fn is_identity(&self) -> bool {
        self.forward == Matrix4x4::identity()
    }

    pub const fn transpose(&self) -> Self {
        unsafe { Self::with_inverse_unchecked(self.forward.transpose(), self.inverse.transpose()) }
    }
}

// Constructors for different transforms
impl Transform {
    #[rustfmt::skip]
    pub const fn translate(delta: Vector3<f32>) -> Self {
        unsafe {
            Self::with_inverse_unchecked(
                Matrix4x4::new(
                    1.0, 0.0, 0.0, delta.x,
                    0.0, 1.0, 0.0, delta.y,
                    0.0, 0.0, 1.0, delta.z,
                    0.0, 0.0, 0.0,     1.0
                ),
                Matrix4x4::new(
                    1.0, 0.0, 0.0, -delta.x,
                    0.0, 1.0, 0.0, -delta.y,
                    0.0, 0.0, 1.0, -delta.z,
                    0.0, 0.0, 0.0,      1.0
                )
            )
        }
    }

    pub const fn scale(x: f32, y: f32, z: f32) -> Self {
        unsafe {
            // SAFETY: matrix is diagonal, so its inverse is the reciprocal of the diagonal entries
            Self::with_inverse_unchecked(
                Matrix4x4::diag(x, y, z, 1.0),
                Matrix4x4::diag(1.0 / x, 1.0 / y, 1.0 / z, 1.0),
            )
        }
    }

    #[rustfmt::skip]
    pub fn rotate_x(theta: f32) -> Self {
        let (sin, cos) = theta.to_radians().sin_cos();
        let mat = Matrix4x4::new(
            1.0, 0.0,  0.0, 0.0,
            0.0, cos, -sin, 0.0,
            0.0, sin,  cos, 0.0,
            0.0, 0.0,  0.0, 1.0
        );
        unsafe {
            // SAFETY: mat is orthogonal, so its transpose is its inverse
            Self::with_inverse_unchecked(mat, mat.transpose())
        }
    }

    #[rustfmt::skip]
    pub fn rotate_y(theta: f32) -> Self {
        let (sin, cos) = theta.to_radians().sin_cos();
        let mat = Matrix4x4::new(
             cos, 0.0, sin, 0.0,
             0.0, 1.0, 0.0, 0.0,
            -sin, 0.0, cos, 0.0,
             0.0, 0.0, 0.0, 1.0,
        );
        unsafe {
            // SAFETY: mat is orthogonal, so its transpose is its inverse
            Self::with_inverse_unchecked(mat, mat.transpose())
        }
    }

    #[rustfmt::skip]
    pub fn rotate_z(theta: f32) -> Self {
        let (sin, cos) = theta.to_radians().sin_cos();
        let mat = Matrix4x4::new(
            cos, -sin, 0.0, 0.0,
            sin,  cos, 0.0, 0.0,
            0.0,  0.0, 1.0, 0.0,
            0.0,  0.0, 0.0, 1.0,
        );
        unsafe {
            // SAFETY: mat is orthogonal, so its transpose is its inverse
            Self::with_inverse_unchecked(mat, mat.transpose())
        }
    }

    #[rustfmt::skip]
    pub fn look_at(pos: &Point3<f32>, look: &Point3<f32>, up: &Vector3<f32>) -> Self {
        let mut camera_to_world = Matrix4x4::zero();
        camera_to_world.set(0, 3, pos.x);
        camera_to_world.set(1, 3, pos.x);
        camera_to_world.set(2, 3, pos.x);
        camera_to_world.set(3, 3, 1.0);

        let dir = (look - pos).normalize();
        let right = up.normalize().cross(&dir).normalize();
        let up = dir.cross(&right);
        let camera_to_world = Matrix4x4::new(
            right.x, up.x, dir.x, pos.x,
            right.y, up.y, dir.y, pos.y,
            right.z, up.z, dir.z, pos.z,
                0.0,  0.0,   0.0,   1.0,
        );
        Self::new(camera_to_world).inverse()
    }
}

impl Transform {
    pub fn transform<T>(&self, src: T) -> T where T: Into<(f32, f32, f32)> + From<(f32, f32, f32)> {
        let (x, y, z) = src.into();
        let m = &self.forward;

        let xp = m[(0,0)] * x + m[(0,1)] * y + m[(0,2)] * z + m[(0, 3)];
        let yp = m[(1,0)] * x + m[(1,1)] * y + m[(1,2)] * z + m[(1, 3)];
        let zp = m[(2,0)] * x + m[(2,1)] * y + m[(2,2)] * z + m[(2, 3)];
        let wp = m[(3,0)] * x + m[(3,1)] * y + m[(3,2)] * z + m[(3, 3)];
        if wp == 1.0 {
            (xp, yp, zp).into()
        } else {
            T::from((xp / wp, yp / wp, zp / wp))
        }
    }
}