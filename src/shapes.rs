use tuple::Tuple;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Sphere {
    pub center: Tuple,
    pub radius: f32,
}

impl Sphere {
    pub fn unit() -> Sphere {
        Sphere {
            center: Tuple::point(0., 0., 0.),
            radius: 1.,
        }
    }
}
