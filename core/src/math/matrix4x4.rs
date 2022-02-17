use std::fmt::{Debug, Formatter};
use std::mem;
use std::ops::{Index, IndexMut, Mul};
use std::simd::f32x4;
use approx::{AbsDiffEq, RelativeEq, UlpsEq};
use bytemuck::{Pod, Zeroable};
use cblas::{sgemm, Transpose};
use lapacke::{sgetri, sgetrf, dgetrf, dgetri, dgetrf2};

// NOTE: the type uses row-major order internally
// Align to matrix row (16 bytes)
#[repr(C, align(16))]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
pub struct Matrix4x4([f32; 16]);

impl Matrix4x4 {
    #[allow(clippy::too_many_arguments)]
    pub const fn new(m11: f32, m12: f32, m13: f32, m14: f32,
                     m21: f32, m22: f32, m23: f32, m24: f32,
                     m31: f32, m32: f32, m33: f32, m34: f32,
                     m41: f32, m42: f32, m43: f32, m44: f32) -> Self {
        Self([m11, m12, m13, m14, m21, m22, m23, m24, m31, m32, m33, m34, m41, m42, m43, m44])
    }

    pub const fn zero() -> Self { Matrix4x4([0.0; 16]) }

    pub const fn identity() -> Self {
        Self::diag(1.0, 1.0, 1.0, 1.0)
    }

    pub const fn diag(m11: f32, m22: f32, m33: f32, m44: f32) -> Self {
        let mut m = Self::default();
        m.set(0, 0, m11);
        m.set(1, 1, m22);
        m.set(2, 2, m33);
        m.set(3, 3, m44);
        m
    }

    pub fn inverse(&self) -> Option<Self> {
        // Compute inverse with double precision and convert back to float in the end
        let mut inv: [f64; 16] = self.0.map(|x| x as f64);
        let mut ipiv = [0;4];
        unsafe {
            let info = dgetrf(lapacke::Layout::RowMajor, 4, 4, inv.as_mut_slice(), 4, ipiv.as_mut_slice());
            if info < 0 {
                panic!("sgetrf: parameter {info} had an invalid value");
            } else if info > 0 {
                // Matrix is singular
                return None
            }
            let info = dgetri(lapacke::Layout::RowMajor, 4, inv.as_mut_slice(), 4, ipiv.as_slice());
            if info < 0 {
                panic!("dgetri: parameter {info} had an invalid value");
            } else if info > 0 {
                // Matrix is singular
                return None
            }
        }
        Some(Matrix4x4(inv.map(|x| x as f32)))
    }

    #[inline]
    pub const fn set(&mut self, i: usize, j: usize, x: f32) {
        *self.get_mut(i, j) = x;
    }

    #[inline]
    pub const fn get_mut(&mut self, i: usize, j: usize) -> &mut f32 {
        assert!(i < 4);
        assert!(j < 4);
        &mut self.0[j + 4 * i]
    }

    #[inline]
    pub const fn swap(&mut self, i0: usize, j0: usize, i1: usize, j1: usize) {
        self.0.swap(j0 + 4 * i0, j1 + 4 * i1);
    }

    pub const fn transpose(&self) -> Self {
        let mut m = *self;
        m.swap(0, 1, 1, 0);
        m.swap(0, 2, 2, 0);
        m.swap(0, 3, 3, 0);
        m.swap(1, 2, 2, 1);
        m.swap(1, 3, 3, 1);
        m.swap(2, 3, 3, 2);
        m
    }

    pub fn gemm(&self, alpha: f32, rhs: &Self, beta: f32, c: &mut Matrix4x4) {
        unsafe {
            sgemm(
                cblas::Layout::RowMajor,
                Transpose::None,
                Transpose::None,
                4,
                4,
                4,
                alpha,
                self.0.as_slice(),
                4,
                rhs.0.as_slice(),
                4,
                beta,
                c.0.as_mut_slice(),
                4
            );
        }
    }
}

impl PartialEq for Matrix4x4 {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl AbsDiffEq for Matrix4x4 {
    type Epsilon = <f32 as AbsDiffEq>::Epsilon;

    fn default_epsilon() -> Self::Epsilon {
        f32::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.0.abs_diff_eq(&other.0, epsilon)
    }
}

impl RelativeEq for Matrix4x4 {
    fn default_max_relative() -> Self::Epsilon {
        f32::default_max_relative()
    }

    fn relative_eq(&self, other: &Self, epsilon: Self::Epsilon, max_relative: Self::Epsilon) -> bool {
        self.0.relative_eq(&other.0, epsilon, max_relative)
    }
}

impl UlpsEq for Matrix4x4 {
    fn default_max_ulps() -> u32 {
        f32::default_max_ulps()
    }

    fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
        self.0.ulps_eq(&other.0, epsilon, max_ulps)
    }
}

impl const Index<(usize, usize)> for Matrix4x4 {
    type Output = f32;

    fn index(&self, (i, j): (usize, usize)) -> &f32 {
        &self.0[j + 4 * i]
    }
}

impl const IndexMut<(usize, usize)> for Matrix4x4 {
    fn index_mut(&mut self, (i, j): (usize, usize)) -> &mut f32 {
        &mut self.0[j + 4 * i]
    }
}

impl const Default for Matrix4x4 {
    fn default() -> Self {
        Matrix4x4([0.0; 16])
    }
}

impl Mul for Matrix4x4 {
    type Output = Matrix4x4;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut c = Matrix4x4::zero();
        self.gemm(1.0, &rhs, 0.0, &mut c);
        c
    }
}

#[cfg(test)]
mod tests {
    use crate::math::Matrix4x4;
    use approx::{assert_abs_diff_eq, assert_relative_eq, assert_ulps_eq};

    #[test]
    fn test_inverse() {
        let mat = Matrix4x4::new(
            5.0, 6.0, 6.0, 8.0,
            2.0, 2.0, 2.0, 8.0,
            6.0, 6.0, 2.0, 8.0,
            2.0, 3.0, 6.0, 7.0
        );
        let inv = mat.inverse().unwrap();
        assert_eq!(
            inv,
            Matrix4x4::new(
                -17.0, -9.0, 12.0, 16.0,
                17.0, 8.75, -11.75, -16.0,
                -4.0, -2.25, 2.75, 4.0,
                1.0, 0.75, -0.75, -1.0
            )
        )
    }

    #[test]
    fn test_transpose() {
        let m = Matrix4x4::new(
            1., 2., 3., 4.,
            5., 6., 7., 8.,
            9., 10., 11., 12.,
            13., 14., 15., 16.
        );
        let t = Matrix4x4::new(
            1., 5., 9., 13.,
            2., 6., 10., 14.,
            3., 7., 11., 15.,
            4., 8., 12., 16.
        );
        assert_eq!(m.transpose(), t);
    }
}