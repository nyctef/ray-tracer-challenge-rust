extern crate float_cmp;

use self::float_cmp::{approx_eq, ApproxEq, F32Margin};

use std::ops;

#[derive(Debug)]
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
}
