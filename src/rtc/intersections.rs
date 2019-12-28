mod ray_sphere;
pub use self::ray_sphere::*;
mod ray_world;
pub use self::ray_world::*;
mod ray_plane;
pub use self::ray_plane::*;

use crate::*;
use std::cmp::Ordering::Equal;

#[derive(Debug, Clone, Copy)]
pub struct Intersection<'a> {
    pub t: f32,
    pub obj: &'a dyn Shape,
}

impl Intersection<'_> {
    pub fn none() -> Vec<Intersection<'static>> {
        vec![]
    }

    pub fn ray_sphere(sphere: &Sphere, t: f32) -> Intersection {
        Intersection { t, obj: sphere }
    }

    pub fn ray_plane(plane: &Plane, t: f32) -> Intersection {
        Intersection { t, obj: plane }
    }

    pub fn hit<'a>(intersections: &'a [Intersection<'a>]) -> Option<&'a Intersection<'a>> {
        let mut sorted = intersections
            .into_iter()
            .filter(|a| a.t >= 0.)
            .collect::<Vec<_>>();

        // f32 doesn't implement Ord because NaN values mean there isn't a total ordering
        // we have to decide how to cope with that ourselves:
        sorted.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap_or(Equal));
        // we want the first element with a positive t value
        sorted.into_iter().next()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finding_first_hit_from_intersection_list() {
        let s = &Sphere::unit();

        // should pick smallest non-negative t value
        let i1 = [
            Intersection::ray_sphere(s, 1.),
            Intersection::ray_sphere(s, 2.),
        ];
        let h1 = Intersection::hit(&i1);
        assert_eq!(1., h1.unwrap().t);

        let i2 = [
            Intersection::ray_sphere(s, -1.),
            Intersection::ray_sphere(s, 1.),
        ];
        let h2 = Intersection::hit(&i2);
        assert_eq!(1., h2.unwrap().t);

        // should return None when there is no non-negative t value
        let i3 = [
            Intersection::ray_sphere(s, -1.),
            Intersection::ray_sphere(s, -2.),
        ];
        let h3 = Intersection::hit(&i3);
        assert!(h3.is_none());

        // should sort entries before returning the first hit
        let i4 = [
            Intersection::ray_sphere(s, 5.),
            Intersection::ray_sphere(s, 7.),
            Intersection::ray_sphere(s, -3.),
            Intersection::ray_sphere(s, 2.),
        ];
        let h4 = Intersection::hit(&i4);
        assert_eq!(2., h4.unwrap().t);
    }
}
