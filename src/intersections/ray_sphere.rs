use intersections::Intersection;
use rays::{Ray, RayIntersection};
use shapes::Sphere;
use tuple::Tuple;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum RaySphereIntersection {
    // the two values are the t parameter for the ray at the intersection point
    Intersects([Intersection; 2]),
    Misses,
}
impl RayIntersection for Sphere {
    type OutputType = RaySphereIntersection;

    fn ray_intersection(self, ray: Ray) -> RaySphereIntersection {
        // we use the inverse of the sphere's transformation to move the ray
        // into the sphere's object space
        let inverse = match self.transformation.try_inverse() {
            // TODO: some sort of warning for the failed inverse?
            None => return RaySphereIntersection::Misses,
            Some(x) => x,
        };
        let ray2 = inverse * ray;

        // now we can intersect ray2 with the unit sphere
        let sphere_to_ray = ray2.origin - Tuple::point(0., 0., 0.);
        let dir = ray2.direction;
        let a = dir.dot(dir);
        let b = 2. * dir.dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.;

        let discriminant = b.powf(2.) - 4. * a * c;

        if discriminant < 0. {
            return RaySphereIntersection::Misses;
        }

        let t1 = (-b - discriminant.sqrt()) / (2. * a);
        let t2 = (-b + discriminant.sqrt()) / (2. * a);

        RaySphereIntersection::Intersects([
            Intersection::ray_sphere(self, t1),
            Intersection::ray_sphere(self, t2),
        ])
    }
}

#[cfg(test)]
mod tests {
    use super::RaySphereIntersection::*;
    use super::*;
    use crate::tuple::Tuple;

    #[test]
    fn intersecting_rays_with_unit_spheres() {
        let s = Sphere::unit();

        // normal hit
        let r1 = Ray::new(Tuple::point(0., 0., -5.), Tuple::vec(0., 0., 1.));
        let i1 = s.ray_intersection(r1);
        assert_eq!(
            Intersects([
                Intersection::ray_sphere(s, 4.),
                Intersection::ray_sphere(s, 6.)
            ]),
            i1
        );

        // tangent hit still produces two collision points
        let r2 = Ray::new(Tuple::point(0., 1., -5.), Tuple::vec(0., 0., 1.));
        let i2 = s.ray_intersection(r2);
        assert_eq!(
            Intersects([
                Intersection::ray_sphere(s, 5.),
                Intersection::ray_sphere(s, 5.)
            ]),
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
            Intersects([
                Intersection::ray_sphere(s, -6.),
                Intersection::ray_sphere(s, -4.)
            ]),
            i4
        );
    }

    #[test]
    fn intersecting_rays_with_transformed_spheres() {
        let r1 = Ray::new(Tuple::point(0., 0., -5.), Tuple::vec(0., 0., 1.));
        let s1 = Sphere::pos_r(Tuple::point(0., 0., 0.), 2.);
        let i1 = s1.ray_intersection(r1);
        assert_eq!(
            Intersects([
                Intersection::ray_sphere(s1, 3.),
                Intersection::ray_sphere(s1, 7.)
            ]),
            i1
        );

        let r2 = Ray::new(Tuple::point(0., 0., -5.), Tuple::vec(0., 0., 1.));
        let s2 = Sphere::pos_r(Tuple::point(5., 0., 0.), 1.);
        let i2 = s2.ray_intersection(r2);
        assert_eq!(Misses, i2);
    }
}
