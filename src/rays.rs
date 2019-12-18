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

trait RayIntersection {
    type OutputType;

    fn ray_intersection(self, ray: Ray) -> Self::OutputType;
}

#[derive(Debug, Copy, Clone)]
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

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum RaySphereIntersection {
    // the two values are the t parameter for the ray at the intersection point
    Intersects(f32, f32),
    Misses,
}
impl RayIntersection for Sphere {
    type OutputType = RaySphereIntersection;

    fn ray_intersection(self, ray: Ray) -> RaySphereIntersection {
        let sphere_to_ray = ray.origin - self.center;
        let dir = ray.direction;
        let a = dir.dot(dir);
        let b = 2. * dir.dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.;

        let discriminant = b.powf(2.) - 4. * a * c;

        if discriminant < 0. {
            return RaySphereIntersection::Misses;
        }

        let t1 = (-b - discriminant.sqrt()) / (2. * a);
        let t2 = (-b + discriminant.sqrt()) / (2. * a);

        RaySphereIntersection::Intersects(t1, t2)
    }
}

#[cfg(test)]
pub mod tests {
    use super::RaySphereIntersection::*;
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

    #[test]
    fn intersecting_rays_with_spheres() {
        let s = Sphere::unit();

        // normal hit
        let r1 = Ray::new(Tuple::point(0., 0., -5.), Tuple::vec(0., 0., 1.));
        let i1 = s.ray_intersection(r1);
        assert_eq!(Intersects(4., 6.), i1);

        // tangent hit still produces two collision points
        let r2 = Ray::new(Tuple::point(0., 1., -5.), Tuple::vec(0., 0., 1.));
        let i2 = s.ray_intersection(r2);
        assert_eq!(Intersects(5., 5.), i2);

        // missing entirely
        let r3 = Ray::new(Tuple::point(0., 2., -5.), Tuple::vec(0., 0., 1.));
        let i3 = s.ray_intersection(r3);
        assert_eq!(Misses, i3);

        // a ray can intersect from behind its origin point
        let r4 = Ray::new(Tuple::point(0., 0., 5.), Tuple::vec(0., 0., 1.));
        let i4 = s.ray_intersection(r4);
        assert_eq!(Intersects(-6., -4.), i4);
    }
}
