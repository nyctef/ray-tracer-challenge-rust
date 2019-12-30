use crate::*;
use std::f32::consts::PI;

// TODO: try out trait vs enum when looking into performance work -
// how much of a difference does it make? (check for Shape as well)
pub trait SamplePattern {
    fn sample_pattern_at(&self, p: Tuple) -> Color;
}

// this enum/impl pair means we can put Pattern directly into a struct
// without having to worry about trait objects or lifetimes (hopefully!).
// see https://users.rust-lang.org/t/11957 for inspiration + some discussion on this idea
#[derive(Debug, Clone, PartialEq)]
pub enum Pattern {
    Solid(SolidColor),
    Stripe(Stripe),
    Gradient(Gradient),
    Ring(Ring),
    Checkerboard(Checkerboard),
    SphereMap(SphereMap),
}
impl SamplePattern for Pattern {
    fn sample_pattern_at(&self, p: Tuple) -> Color {
        match self {
            Pattern::Solid(s) => s.sample_pattern_at(p),
            Pattern::Stripe(s) => s.sample_pattern_at(p),
            Pattern::Gradient(g) => g.sample_pattern_at(p),
            Pattern::Ring(r) => r.sample_pattern_at(p),
            Pattern::Checkerboard(c) => c.sample_pattern_at(p),
            Pattern::SphereMap(s) => s.sample_pattern_at(p),
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

#[derive(Debug, Clone, PartialEq)]
pub struct Checkerboard {
    a: Box<Pattern>,
    b: Box<Pattern>,
    // transformation from object space to pattern space
    object_to_pattern: Matrix4,
}
impl Checkerboard {
    pub fn new(a: Pattern, b: Pattern, transform: Matrix4) -> Checkerboard {
        let object_to_pattern = transform
            .try_inverse()
            .expect("Stripe transform needs to be invertible");

        Checkerboard {
            a: Box::new(a),
            b: Box::new(b),
            object_to_pattern,
        }
    }

    pub fn col(a: Color, b: Color) -> Checkerboard {
        Checkerboard::new(
            Pattern::Solid(SolidColor::new(a)),
            Pattern::Solid(SolidColor::new(b)),
            Matrix4::identity(),
        )
    }
}
impl SamplePattern for Checkerboard {
    fn sample_pattern_at(&self, p: Tuple) -> Color {
        let mut p2 = self.object_to_pattern * p;
        // HACK: since we might want to draw planes that are exactly aligned with a checkerboard boundary
        // (eg the xy plane) but this causes speckles due to float imprecision
        // therefore add an epsilon so that (0,0,0) is firmly within one of the checkerboard cells
        p2 = p2 + vec(0.0001, 0.0001, 0.0001);

        let fac = p2.x.floor() + p2.y.floor() + p2.z.floor();
        if fac % 2. == 0. {
            self.a.sample_pattern_at(p2)
        } else {
            self.b.sample_pattern_at(p2)
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SphereMap {
    a: Box<Pattern>,
    // transformation from object space to pattern space
    object_to_pattern: Matrix4,
}
impl SphereMap {
    pub fn new(a: Pattern, transform: Matrix4) -> SphereMap {
        let object_to_pattern = transform
            .try_inverse()
            .expect("Stripe transform needs to be invertible");

        SphereMap {
            a: Box::new(a),
            object_to_pattern,
        }
    }

    pub fn col(a: Pattern) -> SphereMap {
        SphereMap::new(a, Matrix4::identity())
    }
}
impl SamplePattern for SphereMap {
    fn sample_pattern_at(&self, p: Tuple) -> Color {
        let p2 = self.object_to_pattern * p;

        // based on https://en.wikipedia.org/wiki/UV_mapping#Finding_UV_on_a_sphere
        // Assuming we have a sphere with y pointing up:
        //   arctan2(z, x) gives us the angle from the positive x axis around the y axis
        //   arcsin(y) gives us the angle above or below the xz plane
        // I think this assumes p2 is a point on the unit sphere
        //   (since otherwise we'd need to divide y by the
        //    hypotenuse for the correct value to put in asin())

        let u = 0.5 + (p2.z.atan2(p2.x) / (2. * PI));
        let v = 0.5 - (p2.y.asin() / PI);

        self.a.sample_pattern_at(point(u, v, 0.))
    }
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
