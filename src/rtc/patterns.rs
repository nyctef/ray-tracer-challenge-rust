use crate::*;

// TODO: try out trait vs enum when looking into performance work -
// how much of a difference does it make? (check for Shape as well)
pub trait SamplePattern {
    fn sample_pattern_at(&self, p: Tuple) -> Color;
}

// this enum/impl pair means we can put Pattern directly into a struct
// without having to worry about boxing+lifetimes or trait objects.
// see https://users.rust-lang.org/t/11957 for inspiration + some discussion on this idea
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Pattern {
    Solid(SolidColor),
    Stripe(Stripe),
}
impl SamplePattern for Pattern {
    fn sample_pattern_at(&self, p: Tuple) -> Color {
        match *self {
            Pattern::Solid(s) => s.sample_pattern_at(p),
            Pattern::Stripe(s) => s.sample_pattern_at(p),
        }
    }
}

// TODO: uv-test-pattern-style coordinate pattern?

pub fn solid(color: Color) -> Pattern {
    Pattern::Solid(SolidColor::new(color))
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SolidColor {
    a: Color,
}

impl SolidColor {
    pub fn new(a: Color) -> SolidColor {
        SolidColor { a }
    }
}

impl SamplePattern for SolidColor {
    fn sample_pattern_at(&self, _p: Tuple) -> Color {
        self.a
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Stripe {
    a: Color,
    b: Color,
    // transformation from object space to pattern space
    object_to_pattern: Matrix4,
}

impl Stripe {
    pub fn new(a: Color, b: Color, transform: Matrix4) -> Stripe {
        let object_to_pattern = transform
            .try_inverse()
            .expect("Stripe transform needs to be invertible");

        Stripe {
            a,
            b,
            object_to_pattern,
        }
    }

    pub fn col(a: Color, b: Color) -> Stripe {
        Stripe::new(a, b, Matrix4::identity())
    }
}

impl SamplePattern for Stripe {
    fn sample_pattern_at(&self, p: Tuple) -> Color {
        let p2 = self.object_to_pattern * p;

        if p2.x.floor() % 2. == 0. {
            self.a
        } else {
            self.b
        }
    }
}

pub fn sample_pattern_at_object(p: Pattern, s: &dyn Shape, point: Tuple) -> Color {
    // TODO: world_to_object should probably just be a method on Shape
    let object_point = s.transformation().try_inverse().unwrap() * point;
    p.sample_pattern_at(object_point)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stripe_pattern_alternates_in_x() {
        let p = Stripe::col(white(), black());
        // stripe is constant in y
        assert_eq!(white(), p.sample_pattern_at(point(0., 1., 0.)));
        assert_eq!(white(), p.sample_pattern_at(point(0., 2., 0.)));
        assert_eq!(white(), p.sample_pattern_at(point(0., 3., 0.)));

        // stripe is constant in z
        assert_eq!(white(), p.sample_pattern_at(point(0., 0., 1.)));
        assert_eq!(white(), p.sample_pattern_at(point(0., 0., 2.)));
        assert_eq!(white(), p.sample_pattern_at(point(0., 0., 3.)));

        // stripe alternates in x
        assert_eq!(black(), p.sample_pattern_at(point(1., 0., 0.)));
        assert_eq!(black(), p.sample_pattern_at(point(1.5, 0., 0.)));
        assert_eq!(white(), p.sample_pattern_at(point(2., 0., 0.)));
        assert_eq!(white(), p.sample_pattern_at(point(2.5, 0., 0.)));
        assert_eq!(black(), p.sample_pattern_at(point(3., 0., 0.)));
    }

    #[test]
    fn stripe_pattern_with_pattern_transform() {
        // 2 here means we want the pattern to appear twice as wide on the object
        let p = Stripe::new(white(), black(), scaling(2., 1., 1.));

        assert_eq!(white(), p.sample_pattern_at(point(0., 0., 0.)));
        assert_eq!(white(), p.sample_pattern_at(point(1., 0., 0.)));

        assert_eq!(black(), p.sample_pattern_at(point(2., 0., 0.)));
        assert_eq!(black(), p.sample_pattern_at(point(3., 0., 0.)));

        assert_eq!(white(), p.sample_pattern_at(point(4., 0., 0.)));
        assert_eq!(white(), p.sample_pattern_at(point(5., 0., 0.)));
    }

    #[test]
    fn apply_stripe_to_object_with_object_transform() {
        // the object is twice as big, so the pattern again should appear twice as wide
        let s = &Sphere::pos_r(point(0., 0., 0.), 2.);
        let p = Pattern::Stripe(Stripe::col(white(), black()));

        assert_eq!(white(), sample_pattern_at_object(p, s, point(0., 0., 0.)));
        assert_eq!(white(), sample_pattern_at_object(p, s, point(1., 0., 0.)));

        assert_eq!(black(), sample_pattern_at_object(p, s, point(2., 0., 0.)));
        assert_eq!(black(), sample_pattern_at_object(p, s, point(3., 0., 0.)));

        assert_eq!(white(), sample_pattern_at_object(p, s, point(4., 0., 0.)));
        assert_eq!(white(), sample_pattern_at_object(p, s, point(5., 0., 0.)));
    }
}
