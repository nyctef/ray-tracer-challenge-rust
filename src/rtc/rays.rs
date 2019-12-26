use crate::*;
use std::ops::Mul;
extern crate float_cmp;
use self::float_cmp::{ApproxEq, F32Margin};

#[derive(Debug, Clone, Copy, PartialEq)]
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

/// based on https://docs.rs/float-cmp/0.6.0/float_cmp/index.html
impl ApproxEq for Ray {
    type Margin = F32Margin;

    fn approx_eq<T: Into<Self::Margin>>(self, other: Self, margin: T) -> bool {
        let margin = margin.into();
        self.origin.approx_eq(other.origin, margin)
            && self.direction.approx_eq(other.direction, margin)
    }
}

// do a ray intersection against an object in world space
pub trait RayIntersection {
    fn ray_intersection(&self, ray: Ray) -> Vec<Intersection>;
}

// do a ray intersection against an object in object space
pub trait LocalRayIntersection {
    fn local_ray_intersection(&self, ray: Ray) -> Vec<Intersection>;
}

// if we have an object space ray intersection implementation, we can
// implement one in world space by transforming the ray into local object space
impl<T: LocalRayIntersection + Shape> RayIntersection for T {
    fn ray_intersection(&self, ray: Ray) -> Vec<Intersection> {
        // we use the inverse of the objects's transformation to move the ray
        // into the object's local (object) space, then do a local ray intersection
        let local_ray = self.world_to_object() * ray;

        self.local_ray_intersection(local_ray)
    }
}

impl Mul<Ray> for Matrix4 {
    type Output = Ray;

    fn mul(self, other: Ray) -> Ray {
        Ray::new(self * other.origin, self * other.direction)
    }
}

pub fn reflect(vector: Tuple, normal: Tuple) -> Tuple {
    assert!(vector.is_vec());
    assert!(normal.is_vec());
    vector - normal * 2. * vector.dot(normal)
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn creating_a_ray() {
        let origin = point(1., 2., 3.);
        let direction = vec(4., 5., 6.);
        let ray = Ray::new(origin, direction);

        assert_eq!(1., ray.origin.x);
        assert_eq!(4., ray.direction.x);
    }

    #[test]
    fn can_interpolate_along_ray() {
        let origin = point(2., 3., 4.);
        let direction = vec(1., 0., 0.);
        let r = Ray::new(origin, direction);

        assert_eq!(point(2., 3., 4.), r.position(0.));
        assert_eq!(point(3., 3., 4.), r.position(1.));
        assert_eq!(point(1., 3., 4.), r.position(-1.));
        assert_eq!(point(4.5, 3., 4.), r.position(2.5));
    }

    #[test]
    fn translating_a_ray() {
        let r = Ray::new(point(1., 2., 3.), vec(0., 1., 0.));
        let transform = translation(3., 4., 5.);
        let r2 = transform * r;

        assert_eq!(point(4., 6., 8.), r2.origin);
        assert_eq!(vec(0., 1., 0.), r2.direction);
    }

    #[test]
    fn scaling_a_ray() {
        let r = Ray::new(point(1., 2., 3.), vec(0., 1., 0.));
        let transform = scaling(2., 3., 4.);
        let r2 = transform * r;

        assert_eq!(point(2., 6., 12.), r2.origin);
        assert_eq!(vec(0., 3., 0.), r2.direction);
    }

    #[test]
    fn reflecting_a_vector() {
        let v = vec(1., -1., 0.);
        let n = vec(0., 1., 0.);
        assert_tuple_eq!(vec(1., 1., 0.), reflect(v, n));

        let v2 = vec(0., -1., 0.);
        let s22 = 2_f32.sqrt() / 2.;
        let n2 = vec(s22, s22, 0.);
        assert_tuple_eq!(vec(1., 0., 0.), reflect(v2, n2));
    }
}
