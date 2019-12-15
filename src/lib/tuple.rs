extern crate float_cmp;
use self::float_cmp::{approx_eq, ApproxEq, F32Margin};
use std::ops::Add;

#[derive(Debug)]
struct Tuple {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

impl Tuple {
    fn new(x: f32, y: f32, z: f32, w: f32) -> Tuple {
        Tuple {
            x: x,
            y: y,
            z: z,
            w: w,
        }
    }

    fn point(x: f32, y: f32, z: f32) -> Tuple {
        Tuple::new(x, y, z, 1.0)
    }

    fn vec(x: f32, y: f32, z: f32) -> Tuple {
        Tuple::new(x, y, z, 0.0)
    }

    // TODO: does w need an approximate comparison?
    fn is_point(&self) -> bool {
        self.w == 1.0
    }

    fn is_vec(&self) -> bool {
        self.w == 0.0
    }
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Tuple) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z && self.w == other.w
    }
}

/// some magic to make ApproxEq work for Tuple
/// based on https://docs.rs/float-cmp/0.6.0/float_cmp/index.html
/// TODO: come back and figure out how this works better
impl ApproxEq for Tuple {
    type Margin = F32Margin;

    fn approx_eq<T: Into<Self::Margin>>(self, other: Self, margin: T) -> bool {
        let margin = margin.into();
        self.x.approx_eq(other.x, margin)
            && self.y.approx_eq(other.y, margin)
            && self.z.approx_eq(other.z, margin)
            && self.w.approx_eq(other.w, margin)
    }
}

impl Add for Tuple {
    type Output = Tuple;

    fn add(self, other: Tuple) -> Tuple {
        Tuple::new(
            self.x + other.x,
            self.y + other.y,
            self.z + other.z,
            self.w + other.w,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tuple_creation() {
        let a = Tuple::point(4.3, -4.2, 3.1);
        assert!(a.is_point());
        assert!(!a.is_vec());

        let b = Tuple::vec(4.3, -4.2, 3.1);
        assert!(!b.is_point());
        assert!(b.is_vec());
    }

    #[test]
    fn approx_equality() {
        // using example from https://docs.rs/float-cmp/0.6.0/float_cmp/index.html#the-problem
        // for some reason 0.1+0.2==0.3 works in rust
        let x: f32 = 0.15 + 0.15 + 0.15;
        let y: f32 = 0.1 + 0.1 + 0.25;
        let a = Tuple::point(x, x, x);
        let b = Tuple::point(y, y, y);

        assert_ne!(a, b);
        assert!(approx_eq!(Tuple, a, b))
    }

    #[test]
    fn adding_tuples() {
        let a = Tuple::new(3.0, -2.0, 5.0, 1.0);
        let b = Tuple::new(-2.0, 3.0, 1.0, 0.0);

        assert_eq!(Tuple::new(1.0, 1.0, 6.0, 1.0), a + b)
    }
}
