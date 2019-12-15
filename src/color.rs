extern crate float_cmp;

use self::float_cmp::{approx_eq, ApproxEq, F32Margin};

use std::ops;

#[derive(Debug, PartialEq)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Color {
        Color { r, g, b }
    }
}

/// some magic to make ApproxEq work for Tuple
/// based on https://docs.rs/float-cmp/0.6.0/float_cmp/index.html
/// TODO: come back and figure out how this works better
impl ApproxEq for Color {
    type Margin = F32Margin;

    fn approx_eq<T: Into<Self::Margin>>(self, other: Self, margin: T) -> bool {
        let margin = margin.into();
        self.r.approx_eq(other.r, margin)
            && self.g.approx_eq(other.g, margin)
            && self.b.approx_eq(other.b, margin)
    }
}

impl_op_ex!(+|c1:&Color, c2:&Color| -> Color {
    Color::new(c1.r + c2.r, c1.g + c2.g, c1.b + c2.b)
});
impl_op_ex!(-|c1: &Color, c2: &Color| -> Color {
    Color::new(c1.r - c2.r, c1.g - c2.g, c1.b - c2.b)
});
impl_op_ex!(*|c1: &Color, c2: &Color| -> Color {
    Color::new(c1.r * c2.r, c1.g * c2.g, c1.b * c2.b)
});
impl_op_ex!(*|c1: &Color, x: f32| -> Color { Color::new(c1.r * x, c1.g * x, c1.b * x) });

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn color_creation() {
        let c = Color::new(-0.5, 0.4, 1.7);

        assert_eq!(-0.5, c.r);
        assert_eq!(0.4, c.g);
        assert_eq!(1.7, c.b);
    }

    #[test]
    fn color_operations() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        assert!(approx_eq!(Color, Color::new(1.6, 0.7, 1.0), c1 + c2));

        let c3 = Color::new(0.9, 0.6, 0.75);
        let c4 = Color::new(0.7, 0.1, 0.25);
        assert!(approx_eq!(Color, Color::new(0.2, 0.5, 0.5), c3 - c4));

        let c5 = Color::new(0.2, 0.3, 0.4);
        assert!(approx_eq!(Color, Color::new(0.4, 0.6, 0.8), c5 * 2.0));

        let c6 = Color::new(1.0, 0.2, 0.4);
        let c7 = Color::new(0.9, 1.0, 0.1);
        assert!(approx_eq!(Color, Color::new(0.9, 0.2, 0.04), c6 * c7));
    }
}
