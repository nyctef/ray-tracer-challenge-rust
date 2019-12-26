use crate::*;

// TODO: try out trait vs enum when looking into performance work -
// how much of a difference does it make? (check for Shape as well)
pub trait Pattern {
    fn sample_pattern_at(&self, p: Tuple) -> Color;
}

pub struct StripePattern {
    a: Color,
    b: Color,
}

impl StripePattern {
    pub fn new(a: Color, b: Color) -> StripePattern {
        StripePattern { a, b }
    }
}

impl Pattern for StripePattern {
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
        let p = StripePattern::new(white(), black());
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
