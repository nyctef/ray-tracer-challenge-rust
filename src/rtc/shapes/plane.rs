use crate::*;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Plane {
    world_to_object: Matrix4,
    pub material: PhongMaterial,
}

impl Plane {
    pub fn new(transformation: Matrix4, material: PhongMaterial) -> Plane {
        let world_to_object = transformation
            .try_inverse()
            .expect("Panic! Shape transformation not invertible");
        Plane {
            world_to_object,
            material,
        }
    }

    pub fn xz() -> Plane {
        Plane::new(Matrix4::identity(), PhongMaterial::default())
    }

    pub fn t(transformation: Matrix4) -> Plane {
        Plane::new(transformation, PhongMaterial::default())
    }
}

impl Shape for Plane {
    fn world_to_object(&self) -> Matrix4 {
        self.world_to_object
    }
    fn material(&self) -> PhongMaterial {
        self.material
    }

    fn local_normal_at(&self, _point: Tuple) -> Tuple {
        vec(0., 1., 0.)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn the_normal_of_a_plane_is_constant_everywhere() {
        let p = Plane::xz();
        assert_eq!(vec(0., 1., 0.), p.local_normal_at(point(0., 0., 0.)));
        assert_eq!(vec(0., 1., 0.), p.local_normal_at(point(1., 2., 3.)));
        assert_eq!(vec(0., 1., 0.), p.local_normal_at(point(10., 10., 10.,)));
    }
}
