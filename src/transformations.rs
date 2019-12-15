use crate::matrixes::Matrix4;

pub fn translation(x: f32, y: f32, z: f32) -> Matrix4 {
    Matrix4::new(
        1., 0., 0., x, //
        0., 1., 0., y, //
        0., 0., 1., z, //
        0., 0., 0., 1.,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tuple::Tuple;

    #[test]
    fn multiplying_by_transformation_matrixes() {
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
}
