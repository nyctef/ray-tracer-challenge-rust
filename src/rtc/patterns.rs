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
}

impl Stripe {
    pub fn new(a: Color, b: Color) -> Stripe {
        Stripe { a, b }
    }
}

impl SamplePattern for Stripe {
    fn sample_pattern_at(&self, p: Tuple) -> Color {
        if p.x.floor() % 2. == 0. {
            self.a
        } else {
            self.b
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stripe_pattern_alternates_in_x() {
        let p = Stripe::new(white(), black());
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
}
