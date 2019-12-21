use matrixes::Matrix4;
use transformations::{scaling, translation};
use tuple::Tuple;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Sphere {
    pub transformation: Matrix4,
}

impl Sphere {
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
}
