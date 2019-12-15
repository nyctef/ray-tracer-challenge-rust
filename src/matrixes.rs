extern crate nalgebra as na;
use self::na::{ArrayStorage, Matrix, U2, U3, U4};
use crate::tuple::Tuple;
use std::ops::Mul;

pub type Matrix4 = Matrix<f32, U4, U4, ArrayStorage<f32, U4, U4>>;
pub type Matrix3 = Matrix<f32, U3, U3, ArrayStorage<f32, U3, U3>>;
pub type Matrix2 = Matrix<f32, U2, U2, ArrayStorage<f32, U2, U2>>;

// TODO: should probably just replace custom Tuple type with nalgebra as well
impl Mul<&Tuple> for &Matrix4 {
    type Output = Tuple;

    fn mul(self, other: &Tuple) -> Tuple {
        Tuple::new(
            self[(0, 0)] * other.x
                + self[(0, 1)] * other.y
                + self[(0, 2)] * other.z
                + self[(0, 3)] * other.w,
            self[(1, 0)] * other.x
                + self[(1, 1)] * other.y
                + self[(1, 2)] * other.z
                + self[(1, 3)] * other.w,
            self[(2, 0)] * other.x
                + self[(2, 1)] * other.y
                + self[(2, 2)] * other.z
                + self[(2, 3)] * other.w,
            self[(3, 0)] * other.x
                + self[(3, 1)] * other.y
                + self[(3, 2)] * other.z
                + self[(3, 3)] * other.w,
        )
    }
}

#[cfg(test)]
mod tests {
    // not really testing the library, just checking it behaves as we expect
    use super::*;

    #[test]
    fn constructing_a_4x4_matrix() {
        let m = Matrix4::new(
            1., 2., 3., 4., //
            5.5, 6.5, 7.5, 8.5, //
            9., 10., 11., 12., //
            13.5, 14.5, 15.5, 16.5,
        );

        assert_eq!(1., m[(0, 0)]);
        assert_eq!(4., m[(0, 3)]);
        assert_eq!(13.5, m[(3, 0)]);
    }

    #[test]
    fn matrix_equality() {
        let a = Matrix4::new(
            1., 2., 3., 4., //
            5., 6., 7., 8., //
            9., 10., 11., 12., //
            13., 14., 15., 16.,
        );
        let b = Matrix4::new(
            1., 2., 3., 4., //
            5., 6., 7., 8., //
            9., 10., 11., 12., //
            13., 14., 15., 16.,
        );

        assert_eq!(a, b);
    }

    #[test]
    fn matrix_multiplication() {
        let a = Matrix4::new(
            1., 2., 3., 4., //
            5., 6., 7., 8., //
            9., 8., 7., 6., //
            5., 4., 3., 2.,
        );

        let b = Matrix4::new(
            -2., 1., 2., 3., //
            3., 2., 1., -1., //
            4., 3., 6., 5., //
            1., 2., 7., 8.,
        );

        let expected = Matrix4::new(
            20., 22., 50., 48., //
            44., 54., 114., 108., //
            40., 58., 110., 102., //
            16., 26., 46., 42.,
        );

        assert_eq!(expected, a * b);

        assert_eq!(a, Matrix4::identity() * a);
        assert_eq!(b, Matrix4::identity() * b);
    }

    #[test]
    fn matrix_multiplication_with_tuple() {
        let a = Matrix4::new(
            1., 2., 3., 4., //
            2., 4., 4., 2., //
            8., 6., 4., 1., //
            0., 0., 0., 1.,
        );
        let b = Tuple::new(1., 2., 3., 1.);

        assert_eq!(Tuple::new(18., 24., 33., 1.), &a * &b);

        assert_eq!(b, &Matrix4::identity() * &b);
    }
}
