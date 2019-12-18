use intersections::Intersection;
use rays::{Ray, RayIntersection};
use shapes::Sphere;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum RaySphereIntersection {
    // the two values are the t parameter for the ray at the intersection point
    Intersects(Intersection, Intersection),
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

        RaySphereIntersection::Intersects(
            Intersection::ray_sphere(self, t1),
            Intersection::ray_sphere(self, t2),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::RaySphereIntersection::*;
    use super::*;
    use crate::tuple::Tuple;

    #[test]
    fn intersecting_rays_with_spheres() {
        let s = Sphere::unit();

        // normal hit
        let r1 = Ray::new(Tuple::point(0., 0., -5.), Tuple::vec(0., 0., 1.));
        let i1 = s.ray_intersection(r1);
        assert_eq!(
            Intersects(
                Intersection::ray_sphere(s, 4.),
                Intersection::ray_sphere(s, 6.)
            ),
            i1
        );

        // tangent hit still produces two collision points
        let r2 = Ray::new(Tuple::point(0., 1., -5.), Tuple::vec(0., 0., 1.));
        let i2 = s.ray_intersection(r2);
        assert_eq!(
            Intersects(
                Intersection::ray_sphere(s, 5.),
                Intersection::ray_sphere(s, 5.)
            ),
            i2
        );

        // missing entirely
        let r3 = Ray::new(Tuple::point(0., 2., -5.), Tuple::vec(0., 0., 1.));
        let i3 = s.ray_intersection(r3);
        assert_eq!(Misses, i3);

        // a ray can intersect from behind its origin point
        let r4 = Ray::new(Tuple::point(0., 0., 5.), Tuple::vec(0., 0., 1.));
        let i4 = s.ray_intersection(r4);
        assert_eq!(
            Intersects(
                Intersection::ray_sphere(s, -6.),
                Intersection::ray_sphere(s, -4.)
            ),
            i4
        );
    }
}
