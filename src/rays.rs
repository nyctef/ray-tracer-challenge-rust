use crate::matrixes::Matrix4;
use crate::tuple::Tuple;
use std::ops::Mul;

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

impl Mul<Ray> for Matrix4 {
    type Output = Ray;

    fn mul(self, other: Ray) -> Ray {
        Ray::new(self * other.origin, self * other.direction)
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::transformations::{scaling, translation};

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

    #[test]
    fn translating_a_ray() {
        let r = Ray::new(Tuple::point(1., 2., 3.), Tuple::vec(0., 1., 0.));
        let transform = translation(3., 4., 5.);
        let r2 = transform * r;

        assert_eq!(Tuple::point(4., 6., 8.), r2.origin);
        assert_eq!(Tuple::vec(0., 1., 0.), r2.direction);
    }

    #[test]
    fn scaling_a_ray() {
        let r = Ray::new(Tuple::point(1., 2., 3.), Tuple::vec(0., 1., 0.));
        let transform = scaling(2., 3., 4.);
        let r2 = transform * r;

        assert_eq!(Tuple::point(2., 6., 12.), r2.origin);
        assert_eq!(Tuple::vec(0., 3., 0.), r2.direction);
    }
}
