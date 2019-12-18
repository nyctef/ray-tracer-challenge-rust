use crate::matrixes::Matrix4;

pub fn translation(x: f32, y: f32, z: f32) -> Matrix4 {
    Matrix4::new(
        1., 0., 0., x, //
        0., 1., 0., y, //
        0., 0., 1., z, //
        0., 0., 0., 1.,
    )
}

pub fn scaling(x: f32, y: f32, z: f32) -> Matrix4 {
    Matrix4::new(
        x, 0., 0., 0., //
        0., y, 0., 0., //
        0., 0., z, 0., //
        0., 0., 0., 1.,
    )
}

#[rustfmt::skip]
pub fn rotation_x(rad: f32) -> Matrix4 {
    Matrix4::new(
        1., 0.,        0.,         0.,
        0., rad.cos(), -rad.sin(), 0.,
        0., rad.sin(), rad.cos(),  0.,
        0., 0.,        0.,         1.,
    )
}

#[rustfmt::skip]
pub fn rotation_y(rad: f32) -> Matrix4 {
    Matrix4::new(
        rad.cos(),  0., rad.sin(), 0.,
        0.,         1., 0.,        0.,
        -rad.sin(), 0., rad.cos(), 0.,
        0.,         0., 0.,        1.,
    )
}

#[rustfmt::skip]
pub fn rotation_z(rad: f32) -> Matrix4 {
    Matrix4::new(
        rad.cos(), -rad.sin(), 0., 0.,
        rad.sin(),  rad.cos(), 0., 0.,
        0.,         0.,        0., 0.,
        0.,         0.,        0., 1.,
    )
}

#[cfg(test)]
mod tests {
    extern crate float_cmp;
    use self::float_cmp::approx_eq;
    use super::*;
    use crate::tuple::Tuple;
    use std::f32::consts::PI;

    #[test]
    fn translating_points_should_move_them() {
        let transform = translation(5., -3., 2.);
        let p = Tuple::point(-3., 4., 5.);

        assert_eq!(Tuple::point(2., 1., 7.), &transform * &p);

        let inv = transform.try_inverse().unwrap();
        assert_eq!(Tuple::point(-8., 7., 3.), &inv * &p);
    }

    #[test]
    fn translating_vectors_should_not_change_them() {
        let transform = translation(5., -3., 2.);
        let v = Tuple::vec(-3., 4., 5.);

        assert_eq!(v, &transform * &v);
    }

    #[test]
    fn scaling_points_should_move_them() {
        let transform = scaling(2., 3., 4.);
        let p = Tuple::point(-4., 6., 8.);

        assert_eq!(Tuple::point(-8., 18., 32.), &transform * &p);
    }

    #[test]
    fn scaling_vectors_should_resize_them() {
        let transform = scaling(2., 3., 4.);
        let p = Tuple::vec(-4., 6., 8.);

        assert_eq!(Tuple::vec(-8., 18., 32.), &transform * &p)
    }

    #[test]
    fn rotation_around_x_axis() {
        let p1 = Tuple::point(0., 1., 0.);

        let half_quarter_x_rotation = rotation_x(PI / 4.);
        let quarter_x_rotation = rotation_x(PI / 2.);
        let half_root_2 = 2_f32.sqrt() / 2.;

        assert!(approx_eq!(
            Tuple,
            Tuple::point(0., half_root_2, half_root_2),
            &half_quarter_x_rotation * &p1
        ));

        assert!(approx_eq!(
            Tuple,
            Tuple::point(0., half_root_2, -half_root_2),
            &(half_quarter_x_rotation.try_inverse().unwrap()) * &p1
        ));

        assert!(approx_eq!(
            Tuple,
            Tuple::point(0., 0., 1.),
            &quarter_x_rotation * &p1
        ));
    }

    #[test]
    fn rotation_around_y_axis() {
        let p1 = Tuple::point(0., 0., 1.);

        let half_quarter_y_rotation = rotation_y(PI / 4.);
        let quarter_y_rotation = rotation_y(PI / 2.);
        let half_root_2 = 2_f32.sqrt() / 2.;

        assert!(approx_eq!(
            Tuple,
            Tuple::point(half_root_2, 0., half_root_2),
            &half_quarter_y_rotation * &p1
        ));

        // TODO: is this transformation not invertible?
        // assert!(approx_eq!(
        //     Tuple,
        //     Tuple::point(-half_root_2, 0., -half_root_2),
        //     &(half_quarter_y_rotation.try_inverse().unwrap()) * &p1
        // ));

        assert!(approx_eq!(
            Tuple,
            Tuple::point(1., 0., 0.),
            &quarter_y_rotation * &p1
        ));
    }

    #[test]
    fn rotation_around_z_axis() {
        let p1 = Tuple::point(0., 1., 0.);

        let half_quarter_z_rotation = rotation_z(PI / 4.);
        let quarter_z_rotation = rotation_z(PI / 2.);
        let half_root_2 = 2_f32.sqrt() / 2.;

        assert!(approx_eq!(
            Tuple,
            Tuple::point(-half_root_2, half_root_2, 0.),
            &half_quarter_z_rotation * &p1
        ));

        // TODO: is this transformation not invertible?
        // assert!(approx_eq!(
        //     Tuple,
        //     Tuple::point(half_root_2, half_root_2, 0.),
        //     &(half_quarter_z_rotation.try_inverse().unwrap()) * &p1
        // ));

        assert!(approx_eq!(
            Tuple,
            Tuple::point(-1., 0., 0.),
            &quarter_z_rotation * &p1
        ));
    }
}
