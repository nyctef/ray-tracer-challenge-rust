use crate::*;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Plane {
    pub transformation: Matrix4,
    pub material: PhongMaterial,
}

impl Plane {
    pub fn new(transformation: Matrix4, material: PhongMaterial) -> Plane {
        Plane {
            transformation,
            material,
        }
    }

    pub fn xz() -> Plane {
        Plane::new(Matrix4::identity(), PhongMaterial::default())
    }

    pub fn local_normal_at(&self, _point: Tuple) -> Tuple {
        Tuple::vec(0., 1., 0.)
    }

    pub fn normal_at(&self, point: Tuple) -> Tuple {
        let world_to_sphere = self
            .transformation
            .try_inverse()
            .expect("Panic! Plane transformation not invertible!");

        let object_point = world_to_sphere * point;
        let object_normal = self.local_normal_at(object_point);
        // https://computergraphics.stackexchange.com/a/1506 for `transpose()` justification
        let mut world_normal = world_to_sphere.transpose() * object_normal;
        world_normal.w = 0.;

        world_normal.normalize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn the_normal_of_a_plane_is_constant_everywhere() {
        let p = Plane::xz();
        assert_eq!(
            Tuple::vec(0., 1., 0.),
            p.local_normal_at(Tuple::point(0., 0., 0.))
        );
        assert_eq!(
            Tuple::vec(0., 1., 0.),
            p.local_normal_at(Tuple::point(1., 2., 3.))
        );
        assert_eq!(
            Tuple::vec(0., 1., 0.),
            p.local_normal_at(Tuple::point(10., 10., 10.,))
        );
    }
}
