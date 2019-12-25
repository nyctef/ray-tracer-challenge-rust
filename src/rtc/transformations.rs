use crate::*;
extern crate approx;

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
        0.,         0.,        1., 0.,
        0.,         0.,        0., 1.,
    )
}

#[rustfmt::skip]
pub fn shearing(x_from_y: f32, x_from_z: f32, y_from_x: f32, y_from_z: f32, z_from_x: f32, z_from_y: f32) -> Matrix4 {
    Matrix4::new(
        1.,       x_from_y, x_from_z, 0.,
        y_from_x, 1.,       y_from_z, 0.,
        z_from_x, z_from_y, 1.,       0.,
        0.,       0.,       0.,       1.,
    )
}

// a transform from world space into camera space, where the camera is
// at `from`, facing `to`, with `up` defining the camera rotation
pub fn view_transform(from: Tuple, to: Tuple, up: Tuple) -> Matrix4 {
    assert!(from.is_point());
    assert!(to.is_point());
    assert!(up.is_vec());

    let forward = (to - from).normalize();
    let left = forward.cross(&up.normalize());
    // TODO: figure out the difference between `up` and `true_up`
    let true_up = left.cross(&forward);
    let orientation = Matrix4::new(
        left.x, left.y, left.z, 0., //
        true_up.x, true_up.y, true_up.z, 0., //
        -forward.x, -forward.y, -forward.z, 0., //
        0., 0., 0., 1.,
    );

    orientation * translation(-from.x, -from.y, -from.z)
}

#[cfg(test)]
mod tests {
    extern crate float_cmp;
    use self::approx::assert_relative_eq;
    use self::float_cmp::approx_eq;
    use super::*;
    use std::f32::consts::PI;

    #[test]
    fn translating_points_should_move_them() {
        let transform = translation(5., -3., 2.);
        let p = Tuple::point(-3., 4., 5.);

        assert_eq!(Tuple::point(2., 1., 7.), transform * p);

        let inv = transform.try_inverse().unwrap();
        assert_eq!(Tuple::point(-8., 7., 3.), inv * p);
    }

    #[test]
    fn translating_vectors_should_not_change_them() {
        let transform = translation(5., -3., 2.);
        let v = Tuple::vec(-3., 4., 5.);

        assert_eq!(v, transform * v);
    }

    #[test]
    fn scaling_points_should_move_them() {
        let transform = scaling(2., 3., 4.);
        let p = Tuple::point(-4., 6., 8.);

        assert_eq!(Tuple::point(-8., 18., 32.), transform * p);
    }

    #[test]
    fn scaling_vectors_should_resize_them() {
        let transform = scaling(2., 3., 4.);
        let p = Tuple::vec(-4., 6., 8.);

        assert_eq!(Tuple::vec(-8., 18., 32.), transform * p)
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
            half_quarter_x_rotation * p1
        ));

        assert!(approx_eq!(
            Tuple,
            Tuple::point(0., half_root_2, -half_root_2),
            (half_quarter_x_rotation.try_inverse().unwrap()) * p1
        ));

        assert!(approx_eq!(
            Tuple,
            Tuple::point(0., 0., 1.),
            quarter_x_rotation * p1
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
            half_quarter_y_rotation * p1
        ));

        assert!(approx_eq!(
            Tuple,
            Tuple::point(-half_root_2, 0., half_root_2),
            half_quarter_y_rotation.try_inverse().unwrap() * p1
        ));

        assert!(approx_eq!(
            Tuple,
            Tuple::point(1., 0., 0.),
            quarter_y_rotation * p1
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
            half_quarter_z_rotation * p1
        ));

        assert!(approx_eq!(
            Tuple,
            Tuple::point(half_root_2, half_root_2, 0.),
            half_quarter_z_rotation.try_inverse().unwrap() * p1
        ));

        assert!(approx_eq!(
            Tuple,
            Tuple::point(-1., 0., 0.),
            quarter_z_rotation * p1
        ));
    }

    #[test]
    fn shearing_moves_axes_in_proportion_to_each_other() {
        let t1 = shearing(1., 0., 0., 0., 0., 0.);
        let p1 = Tuple::point(2., 3., 4.);
        assert_eq!(Tuple::point(5., 3., 4.), t1 * p1);

        let t2 = shearing(0., 1., 0., 0., 0., 0.);
        let p2 = Tuple::point(2., 3., 4.);
        assert_eq!(Tuple::point(6., 3., 4.), t2 * p2);

        let t3 = shearing(0., 0., 1., 0., 0., 0.);
        let p3 = Tuple::point(2., 3., 4.);
        assert_eq!(Tuple::point(2., 5., 4.), t3 * p3);

        let t4 = shearing(0., 0., 0., 1., 0., 0.);
        let p4 = Tuple::point(2., 3., 4.);
        assert_eq!(Tuple::point(2., 7., 4.), t4 * p4);

        let t5 = shearing(0., 0., 0., 0., 1., 0.);
        let p5 = Tuple::point(2., 3., 4.);
        assert_eq!(Tuple::point(2., 3., 6.), t5 * p5);

        let t6 = shearing(0., 0., 0., 0., 0., 1.);
        let p6 = Tuple::point(2., 3., 4.);
        assert_eq!(Tuple::point(2., 3., 7.), t6 * p6);
    }

    #[test]
    fn default_view_transform_is_identity_matrix() {
        // the default view transform actually looks along the z axis in the negative direction-
        // adding a camera requires things to be flipped
        let vt = view_transform(
            Tuple::point(0., 0., 0.),
            Tuple::point(0., 0., -1.),
            Tuple::vec(0., 1., 0.),
        );
        assert_eq!(Matrix4::identity(), vt);
    }

    #[test]
    fn view_transform_for_positive_z_axis_direction() {
        let vt = view_transform(
            Tuple::point(0., 0., 0.),
            Tuple::point(0., 0., 1.),
            Tuple::vec(0., 1., 0.),
        );
        // now we flip the z and x axes
        assert_eq!(scaling(-1., 1., -1.), vt);
    }

    #[test]
    fn the_view_transform_moves_the_world() {
        let vt = view_transform(
            Tuple::point(0., 0., 8.),
            Tuple::point(0., 0., 0.),
            Tuple::vec(0., 1., 0.),
        );
        // if the camera is at z=8, then we shift the whole world -8 on z axis to put the camera at the origin
        assert_eq!(translation(0., 0., -8.), vt);
    }

    #[test]
    fn an_arbitrary_view_transform() {
        // example from book
        let vt = view_transform(
            Tuple::point(1., 3., 2.),
            Tuple::point(4., -2., 8.),
            Tuple::vec(1., 1., 0.),
        );
        let expected = Matrix4::new(
            -0.50709, 0.50709, 0.67612, -2.36643, //
            0.76772, 0.60609, 0.12122, -2.82843, //
            -0.35857, 0.59761, -0.71714, 0.00000, //
            0.00000, 0.00000, 0.00000, 1.00000, //
        );
        assert_relative_eq!(expected, vt, epsilon = 0.00001);
    }
}
