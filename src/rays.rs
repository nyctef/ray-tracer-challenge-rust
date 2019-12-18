use crate::tuple::Tuple;

#[derive(Debug)]
pub struct Ray {
    pub origin: Tuple,
    pub direction: Tuple,
}

impl Ray {
    pub fn new(origin: Tuple, direction: Tuple) -> Ray {
        assert!(origin.is_point());
        assert!(direction.is_vec());
        Ray { origin, direction }
    }

    pub fn position(&self, t: f32) -> Tuple {
        &self.origin + &self.direction * t
    }
}

pub trait RayIntersection {
    type OutputType;

    fn ray_intersection(self, ray: Ray) -> Self::OutputType;
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn creating_a_ray() {
        let origin = Tuple::point(1., 2., 3.);
        let direction = Tuple::vec(4., 5., 6.);
        let ray = Ray::new(origin, direction);

        assert_eq!(1., ray.origin.x);
        assert_eq!(4., ray.direction.x);
    }

    #[test]
    fn can_interpolate_along_ray() {
        let origin = Tuple::point(2., 3., 4.);
        let direction = Tuple::vec(1., 0., 0.);
        let r = Ray::new(origin, direction);

        assert_eq!(Tuple::point(2., 3., 4.), r.position(0.));
        assert_eq!(Tuple::point(3., 3., 4.), r.position(1.));
        assert_eq!(Tuple::point(1., 3., 4.), r.position(-1.));
        assert_eq!(Tuple::point(4.5, 3., 4.), r.position(2.5));
    }
}
