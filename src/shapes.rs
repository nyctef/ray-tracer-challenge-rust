use matrixes::Matrix4;
use transformations::{rotation_z, scaling, translation};
use tuple::Tuple;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Sphere {
    pub transformation: Matrix4,
}

impl Sphere {
    pub fn new(transformation: Matrix4) -> Sphere {
        Sphere { transformation }
    }

    pub fn unit() -> Sphere {
        Sphere {
            transformation: Matrix4::identity(),
        }
    }

    pub fn pos_r(position: Tuple, r: f32) -> Sphere {
        Sphere {
            transformation: translation(position.x, position.y, position.z)
                * scaling(r, r, r)
                * Matrix4::identity(),
        }
    }

    // TODO: possible trait?
    pub fn normal_at(&self, position: Tuple) -> Tuple {
        // TODO: should these transformations always be invertible? (see TODO in trasnformations.rs tests)
        let world_to_sphere = self
            .transformation
            .try_inverse()
            .expect("Panic! Sphere transformation not invertible!");

        let object_point = world_to_sphere * position;
        let object_normal = object_point - Tuple::point(0., 0., 0.);
        // TODO: figure out why this works and/or is necessary
        let mut world_normal = world_to_sphere.transpose() * object_normal;
        world_normal.w = 0.;
        return world_normal.normalize();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate float_cmp;
    use self::float_cmp::approx_eq;
    use std::f32::consts::PI;

    // TODO: learn more about how this assert is put together
    // (code crudely copied from https://doc.rust-lang.org/src/core/macros.rs.html#78-111)
    // TODO: use this macro more widely
    macro_rules! assert_tuple_eq {
        ($left:expr, $right:expr $(, $set:ident = $val:expr)*) => {{
            match (&$left, &$right) {
                (left_val, right_val) => {
                    if !approx_eq!(Tuple, *left_val, *right_val $(, $set = $val)*) {
                        panic!(
                            r#"assertion failed: `(left approxEquals right)`
   left: `{:?}`
  right: `{:?}`"#,
                            &*left_val, &*right_val
                        );
                    }
                }
            }
        }};
    }

    #[test]
    fn normal_at_points_on_unit_sphere() {
        let s = Sphere::unit();

        // normal on x axis points in x direction
        let n1 = s.normal_at(Tuple::point(1., 0., 0.));
        assert_tuple_eq!(Tuple::vec(1., 0., 0.), n1);

        let r33 = 3_f32.sqrt() / 3_f32;
        let n2 = s.normal_at(Tuple::point(r33, r33, r33));
        assert_tuple_eq!(Tuple::vec(r33, r33, r33), n2);
    }

    #[test]
    fn normal_at_points_on_translated_sphere() {
        let s1 = Sphere::pos_r(Tuple::point(0., 1., 0.), 1.);
        let n1 = s1.normal_at(Tuple::point(0., 1.707111, -0.70711));
        // TODO: is it possible to figure out if this is a sensible epsilon?
        // is there more precision we should be preserving?
        assert_tuple_eq!(n1, Tuple::vec(0., 0.70711, -0.70711), epsilon = 0.0001);

        let s2 = Sphere::new(scaling(1., 0.5, 1.) * rotation_z(PI / 5.));
        let s22 = 2_f32.sqrt() / 2.;
        let n2 = s2.normal_at(Tuple::point(0., s22, -s22));
        assert_tuple_eq!(n2, Tuple::vec(0., 0.97014, -0.24254), epsilon = 0.00001);
    }
}
