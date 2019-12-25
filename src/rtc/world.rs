use crate::*;

#[derive(Debug, Clone)]
pub struct World {
    pub objects: Vec<Sphere>,
    pub lights: Vec<PointLight>,
}

impl World {
    pub fn new(objects: Vec<Sphere>, lights: Vec<PointLight>) -> World {
        World { objects, lights }
    }

    pub fn default() -> World {
        let mut s1 = Sphere::unit();
        s1.material = PhongMaterial::new(Color::new(0.8, 1., 0.6), 0.1, 0.7, 0.2, 200.);
        let s2 = Sphere::pos_r(Tuple::point(0., 0., 0.), 0.5);
        let l1 = PointLight::new(Color::new(1., 1., 1.), Tuple::point(-10., 10., -10.));

        World {
            objects: vec![s1, s2],
            lights: vec![l1],
        }
    }
}
