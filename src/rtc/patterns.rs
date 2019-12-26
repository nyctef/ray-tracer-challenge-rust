use crate::*;

// TODO: try out trait vs enum when looking into performance work -
// how much of a difference does it make? (check for Shape as well)
pub trait SamplePattern {
    fn sample_pattern_at(&self, p: Tuple) -> Color;
}

// this enum/impl pair means we can put Pattern directly into a struct
// without having to worry about boxing+lifetimes or trait objects.
// see https://users.rust-lang.org/t/11957 for inspiration + some discussion on this idea
#[derive(Debug, Clone, PartialEq)]
pub enum Pattern {
    Solid(SolidColor),
    Stripe(Stripe),
}
impl SamplePattern for Pattern {
    fn sample_pattern_at(&self, p: Tuple) -> Color {
        match self {
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

#[derive(Debug, Clone, PartialEq)]
pub struct Stripe {
    a: Box<Pattern>,
    b: Box<Pattern>,
    // transformation from object space to pattern space
    object_to_pattern: Matrix4,
}

impl Stripe {
    pub fn new(a: Color, b: Color, transform: Matrix4) -> Stripe {
        let object_to_pattern = transform
            .try_inverse()
            .expect("Stripe transform needs to be invertible");

        Stripe {
            a: Box::new(solid(a)),
            b: Box::new(solid(b)),
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
            self.a.sample_pattern_at(p2)
        } else {
            self.b.sample_pattern_at(p2)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Gradient {
    a: Color,
    b: Color,
    // transformation from object space to pattern space
    object_to_pattern: Matrix4,
}
impl Gradient {
    pub fn new(a: Color, b: Color, transform: Matrix4) -> Gradient {
        let object_to_pattern = transform
            .try_inverse()
            .expect("Stripe transform needs to be invertible");

        Gradient {
            a,
            b,
            object_to_pattern,
        }
    }

    pub fn col(a: Color, b: Color) -> Gradient {
        Gradient::new(a, b, Matrix4::identity())
    }
}
impl SamplePattern for Gradient {
    fn sample_pattern_at(&self, p: Tuple) -> Color {
        let p2 = self.object_to_pattern * p;

        let a_fac = 1. - 0_f32.max(p2.x).min(1.);
        let b_fac = 1. - a_fac;
        (self.a * a_fac + self.b * b_fac).clamp()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ring {
    a: Color,
    b: Color,
    // transformation from object space to pattern space
    object_to_pattern: Matrix4,
}
impl Ring {
    pub fn new(a: Color, b: Color, transform: Matrix4) -> Ring {
        let object_to_pattern = transform
            .try_inverse()
            .expect("Stripe transform needs to be invertible");

        Ring {
            a,
            b,
            object_to_pattern,
        }
    }

    pub fn col(a: Color, b: Color) -> Ring {
        Ring::new(a, b, Matrix4::identity())
    }
}
impl SamplePattern for Ring {
    fn sample_pattern_at(&self, p: Tuple) -> Color {
        let p2 = self.object_to_pattern * p;

        let fac = (p2.x * p2.x + p2.z * p2.z).sqrt();
        if fac.floor() % 2. == 0. {
            self.a
        } else {
            self.b
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Checkerboard {
    a: Color,
    b: Color,
    // transformation from object space to pattern space
    object_to_pattern: Matrix4,
}
impl Checkerboard {
    pub fn new(a: Color, b: Color, transform: Matrix4) -> Checkerboard {
        let object_to_pattern = transform
            .try_inverse()
            .expect("Stripe transform needs to be invertible");

        Checkerboard {
            a,
            b,
            object_to_pattern,
        }
    }

    pub fn col(a: Color, b: Color) -> Checkerboard {
        Checkerboard::new(a, b, Matrix4::identity())
    }
}
impl SamplePattern for Checkerboard {
    fn sample_pattern_at(&self, p: Tuple) -> Color {
        let p2 = self.object_to_pattern * p;

        let fac = p2.x.floor() + p2.y.floor() + p2.z.floor();
        if fac % 2. == 0. {
            self.a
        } else {
            self.b
        }
    }
}

// ended up going with a different implementation
// pub fn sample_pattern_at_object(p: Pattern, s: &dyn Shape, point: Tuple) -> Color {
//     let object_point = s.world_to_object() * point;
//     p.sample_pattern_at(object_point)
// }

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

    // #[test]
    // fn apply_stripe_to_object_with_object_transform() {
    //     // the object is twice as big, so the pattern again should appear twice as wide
    //     let s = &Sphere::pos_r(point(0., 0., 0.), 2.);
    //     let p = Pattern::Stripe(Stripe::col(white(), black()));

    //     assert_eq!(white(), sample_pattern_at_object(p, s, point(0., 0., 0.)));
    //     assert_eq!(white(), sample_pattern_at_object(p, s, point(1., 0., 0.)));

    //     assert_eq!(black(), sample_pattern_at_object(p, s, point(2., 0., 0.)));
    //     assert_eq!(black(), sample_pattern_at_object(p, s, point(3., 0., 0.)));

    //     assert_eq!(white(), sample_pattern_at_object(p, s, point(4., 0., 0.)));
    //     assert_eq!(white(), sample_pattern_at_object(p, s, point(5., 0., 0.)));
    // }

    #[test]
    fn gradient_lerps_between_colors_on_x_axis() {
        let g = Gradient::col(black(), white());

        assert_eq!(black(), g.sample_pattern_at(point(0., 0., 0.)));
        assert_eq!(grey(0.25), g.sample_pattern_at(point(0.25, 0., 0.)));
        assert_eq!(grey(0.50), g.sample_pattern_at(point(0.50, 0., 0.)));
        assert_eq!(grey(0.75), g.sample_pattern_at(point(0.75, 0., 0.)));
        assert_eq!(white(), g.sample_pattern_at(point(1., 0., 0.)));
    }

    #[test]
    fn ring_pattern_uses_x_and_z_values() {
        let p = Ring::col(white(), black());

        assert_eq!(white(), p.sample_pattern_at(point(0., 0., 0.)));
        assert_eq!(white(), p.sample_pattern_at(point(0.5, 0., 0.)));
        assert_eq!(white(), p.sample_pattern_at(point(0., 0., 0.5)));
        assert_eq!(black(), p.sample_pattern_at(point(1., 0., 1.)));
    }

    #[test]
    fn checkerboard_repeats_in_each_dimension() {
        let p = Checkerboard::col(white(), black());

        assert_eq!(white(), p.sample_pattern_at(point(0., 0., 0.)));
        assert_eq!(white(), p.sample_pattern_at(point(0., 0.99, 0.)));
        assert_eq!(black(), p.sample_pattern_at(point(0., 1.01, 0.)));

        assert_eq!(white(), p.sample_pattern_at(point(0., 0., 0.)));
        assert_eq!(white(), p.sample_pattern_at(point(0.99, 0., 0.)));
        assert_eq!(black(), p.sample_pattern_at(point(1.01, 0., 0.)));

        assert_eq!(white(), p.sample_pattern_at(point(0., 0., 0.)));
        assert_eq!(white(), p.sample_pattern_at(point(0., 0., 0.99)));
        assert_eq!(black(), p.sample_pattern_at(point(0., 0., 1.01)));
    }
}
