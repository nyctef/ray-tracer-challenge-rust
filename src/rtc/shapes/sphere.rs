use crate::*;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Sphere {
    pub transformation: Matrix4,
    pub material: PhongMaterial,
}

impl Sphere {
    pub fn new(transformation: Matrix4, material: PhongMaterial) -> Sphere {
        Sphere {
            transformation,
            material,
        }
    }

    pub fn unit() -> Sphere {
        Sphere {
            transformation: Matrix4::identity(),
            material: Default::default(),
        }
    }

    pub fn pos_r(position: Tuple, r: f32) -> Sphere {
        Sphere {
            transformation: translation(position.x, position.y, position.z)
                * scaling(r, r, r)
                * Matrix4::identity(),
            material: Default::default(),
        }
    }

    pub fn pos_r_m(position: Tuple, r: f32, material: PhongMaterial) -> Sphere {
        Sphere {
            transformation: translation(position.x, position.y, position.z)
                * scaling(r, r, r)
                * Matrix4::identity(),
            material,
        }
    }
}

impl Shape for Sphere {
    fn transformation(&self) -> Matrix4 {
        self.transformation
    }
    fn material(&self) -> PhongMaterial {
        self.material
    }

    fn local_normal_at(&self, p: Tuple) -> Tuple {
        p - point(0., 0., 0.)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::PI;

    #[test]
    fn normal_at_points_on_unit_sphere() {
        let s = Sphere::unit();

        // normal on x axis points in x direction
        let n1 = s.normal_at(point(1., 0., 0.));
        assert_tuple_eq!(vec(1., 0., 0.), n1);

        let r33 = 3_f32.sqrt() / 3_f32;
        let n2 = s.normal_at(point(r33, r33, r33));
        assert_tuple_eq!(vec(r33, r33, r33), n2);
    }

    #[test]
    fn normal_at_points_on_translated_sphere() {
        let s1 = Sphere::pos_r(point(0., 1., 0.), 1.);
        let n1 = s1.normal_at(point(0., 1.707111, -0.70711));
        assert_tuple_eq!(n1, vec(0., 0.70711, -0.70711), epsilon = 0.0001);

        let s2 = Sphere::new(
            scaling(1., 0.5, 1.) * rotation_z(PI / 5.),
            Default::default(),
        );
        let s22 = 2_f32.sqrt() / 2.;
        let n2 = s2.normal_at(point(0., s22, -s22));
        assert_tuple_eq!(n2, vec(0., 0.97014, -0.24254), epsilon = 0.00001);
    }
}
