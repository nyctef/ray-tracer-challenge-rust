use matrixes::Matrix4;
use transformations::{scaling, translation};
use tuple::Tuple;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Sphere {
    pub transformation: Matrix4,
}

impl Sphere {
    pub fn unit() -> Sphere {
        Sphere {
            transformation: Matrix4::identity(),
        }
    }

    pub fn pos_r(position: Tuple, r: f32) -> Sphere {
        Sphere {
            transformation: translation(position.x, position.y, position.z)
                * scaling(r, r, r)
                * Matrix4::identity(),
        }
    }

    // TODO: possible trait?
    pub fn normal_at(&self, position: Tuple) -> Tuple {
        Tuple::vec(position.x, position.y, position.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn normal_at_points_on_unit_sphere() {
        let s = Sphere::unit();

        // normal on x axis points in x direction
        let n1 = s.normal_at(Tuple::point(1., 0., 0.));
        assert_eq!(Tuple::vec(1., 0., 0.), n1);

        let root_three_third = 3_f32.sqrt() / 3_f32;
        let n2 = s.normal_at(Tuple::point(
            root_three_third,
            root_three_third,
            root_three_third,
        ));
        assert_eq!(
            Tuple::vec(root_three_third, root_three_third, root_three_third),
            n2
        );
    }
}
