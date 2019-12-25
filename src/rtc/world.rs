use crate::*;

// this extra type is needed to avoid E0225
// because of https://github.com/rust-lang/rust/issues/32220
pub trait IntersectableShape: Shape + RayIntersection {}
impl<T: Shape + RayIntersection> IntersectableShape for T {}

#[derive(Debug)]
pub struct World {
    pub objects: Vec<Box<dyn IntersectableShape>>,
    pub lights: Vec<PointLight>,
}

impl World {
    pub fn new(objects: Vec<Box<dyn IntersectableShape>>, lights: Vec<PointLight>) -> World {
        World { objects, lights }
    }

    pub fn default() -> World {
        let mut s1 = Sphere::unit();
        s1.material = PhongMaterial::new(Color::new(0.8, 1., 0.6), 0.1, 0.7, 0.2, 200.);
        let s2 = Sphere::pos_r(Tuple::point(0., 0., 0.), 0.5);
        let l1 = PointLight::new(Color::new(1., 1., 1.), Tuple::point(-10., 10., -10.));

        World {
            objects: vec![Box::new(s1), Box::new(s2)],
            lights: vec![l1],
        }
    }
}
