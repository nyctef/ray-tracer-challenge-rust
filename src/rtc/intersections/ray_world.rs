use crate::*;

impl RayIntersection for &World {
    type OutputType = Vec<Intersection>;

    fn ray_intersection(self, ray: Ray) -> Self::OutputType {
        let mut result = Vec::<Intersection>::new();

        for obj in &self.objects {
            obj.ray_intersection(ray)
                .map(|i| result.extend_from_slice(&i));
        }

        result.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap_or(std::cmp::Ordering::Equal));

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn intersection_with_default_world() {
        let w = &World::default();
        let r = Ray::new(Tuple::point(0., 0., -5.), Tuple::vec(0., 0., 1.));
        let intersections = w.ray_intersection(r);

        assert_eq!(4, intersections.len());

        // the ray should pass through the two default spheres in order
        assert_eq!(4., intersections[0].t);
        assert_eq!(4.5, intersections[1].t);
        assert_eq!(5.5, intersections[2].t);
        assert_eq!(6., intersections[3].t);
    }
}
