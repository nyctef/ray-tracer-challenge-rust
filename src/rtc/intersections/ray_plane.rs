use crate::*;

impl LocalRayIntersection for Plane {
    // TODO: maybe this should just be a Vec<Intersection> for all implementations?
    type OutputType = Option<Intersection>;

    fn local_ray_intersection(self, ray: Ray) -> Self::OutputType {
        if approx_eq!(f32, ray.direction.y, 0.) {
            // ray doesn't move in y axis, so it's parallel or coplanar with the xz plane
            return None;
        }

        // we need to find the t value for the point in the ray where y=0.
        // the ray equation is y = origin_y + direction_y * t
        // therefore we calculate t = -origin_y / direction_y
        Some(Intersection::ray_plane(
            self,
            -ray.origin.y / ray.direction.y,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate float_cmp;
    use self::float_cmp::*;

    #[test]
    fn ray_parallel_to_plane_misses() {
        let r = Ray::new(Tuple::point(0., 1., 0.), Tuple::vec(0., 0., 1.));
        let p = Plane::xz();
        assert_eq!(None, p.ray_intersection(r));
    }

    #[test]
    fn ray_coplanar_to_plane_is_defined_as_missing() {
        // technically, a ray that lies inside a plane hits infinitely many times,
        // but since the plan is infinitely thin, if we actually looked down
        // the ray then we wouldn't see anything.

        let r = Ray::new(Tuple::point(0., 0., 0.), Tuple::vec(0., 0., 1.));
        let p = Plane::xz();
        assert_eq!(None, p.ray_intersection(r));
    }

    #[test]
    fn ray_hitting_plane_from_above_hits() {
        let r = Ray::new(
            Tuple::point(-1., 1., -1.),
            Tuple::vec(1., -1., 1.).normalize(),
        );
        let p = Plane::xz();
        let s33 = 3_f32.sqrt();
        let intersection = p.ray_intersection(r).unwrap();
        assert!(approx_eq!(f32, s33, intersection.t));
    }
}
