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
}
