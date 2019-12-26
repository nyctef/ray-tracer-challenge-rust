extern crate float_cmp;
use self::float_cmp::{ApproxEq, F32Margin};
use std::ops;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Color {
        Color { r, g, b }
    }

    pub fn set(&mut self, other: &Color) {
        self.r = other.r;
        self.g = other.g;
        self.b = other.b;
    }

    pub fn clamp(&self) -> Color {
        Color::new(
            clamp(self.r, 0.0, 1.0),
            clamp(self.g, 0.0, 1.0),
            clamp(self.b, 0.0, 1.0),
        )
    }

    pub fn to_u8(&self) -> (u8, u8, u8) {
        (
            (self.r * 255.).round() as u8,
            (self.g * 255.).round() as u8,
            (self.b * 255.).round() as u8,
        )
    }
}

pub fn black() -> Color {
    Color::new(0.0, 0.0, 0.0)
}
pub fn red() -> Color {
    Color::new(1.0, 0.0, 0.0)
}
pub fn white() -> Color {
    Color::new(1.0, 1.0, 1.0)
}
pub fn grey(x: f32) -> Color {
    Color::new(x, x, x)
}

fn clamp(value: f32, min: f32, max: f32) -> f32 {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

/// based on https://docs.rs/float-cmp/0.6.0/float_cmp/index.html
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

impl ops::AddAssign<Color> for Color {
    fn add_assign(&mut self, c2: Color) {
        self.r += c2.r;
        self.g += c2.g;
        self.b += c2.b;
    }
}

#[cfg(test)]
mod tests {
    use self::float_cmp::approx_eq;
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

    #[test]
    fn color_clamp() {
        let c1 = Color::new(-0.5, 0.5, 1.5);
        assert_eq!(Color::new(0.0, 0.5, 1.0), c1.clamp())
    }
}
