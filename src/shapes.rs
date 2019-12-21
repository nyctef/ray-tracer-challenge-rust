use matrixes::Matrix4;

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
}
