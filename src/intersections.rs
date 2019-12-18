pub mod ray_sphere;

use shapes::Sphere;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum IntersectionObject {
    Sphere(Sphere),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Intersection {
    t: f32,
    obj: IntersectionObject,
}

impl Intersection {
    fn ray_sphere(sphere: Sphere, t: f32) -> Intersection {
        Intersection {
            t,
            obj: IntersectionObject::Sphere(sphere),
        }
    }
}
