mod ray_sphere;
pub use self::ray_sphere::*;
mod ray_world;
pub use self::ray_world::*;
mod ray_plane;
pub use self::ray_plane::*;

use crate::*;
use std::cmp::Ordering::Equal;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum IntersectionObject {
    Sphere(Sphere),
    Plane(Plane),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Intersection {
    pub t: f32,
    pub obj: IntersectionObject,
}

impl Intersection {
    pub fn ray_sphere(sphere: Sphere, t: f32) -> Intersection {
        Intersection {
            t,
            obj: IntersectionObject::Sphere(sphere),
        }
    }

    pub fn ray_plane(plane: Plane, t: f32) -> Intersection {
        Intersection {
            t,
            obj: IntersectionObject::Plane(plane),
        }
    }

    pub fn hit(intersections: &[Intersection]) -> Option<&Intersection> {
        let mut sorted = intersections
            .iter()
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
        let s = Sphere::unit();

        // should pick smallest non-negative t value
        let i1 = [
            Intersection::ray_sphere(s, 1.),
            Intersection::ray_sphere(s, 2.),
        ];
        let h1 = Intersection::hit(&i1);
        assert_eq!(&i1[0], h1.unwrap());

        let i2 = [
            Intersection::ray_sphere(s, -1.),
            Intersection::ray_sphere(s, 1.),
        ];
        let h2 = Intersection::hit(&i2);
        assert_eq!(&i2[1], h2.unwrap());

        // should return None when there is no non-negative t value
        let i3 = [
            Intersection::ray_sphere(s, -1.),
            Intersection::ray_sphere(s, -2.),
        ];
        let h3 = Intersection::hit(&i3);
        assert_eq!(None, h3);

        // should sort entries before returning the first hit
        let i4 = [
            Intersection::ray_sphere(s, 5.),
            Intersection::ray_sphere(s, 7.),
            Intersection::ray_sphere(s, -3.),
            Intersection::ray_sphere(s, 2.),
        ];
        let h4 = Intersection::hit(&i4);
        assert_eq!(&i4[3], h4.unwrap());
    }
}
