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
}

impl Shape for Plane {
    fn transformation(&self) -> Matrix4 {
        self.transformation
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
