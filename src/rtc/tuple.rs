extern crate float_cmp;
use self::float_cmp::{ApproxEq, F32Margin};

use std::ops;

#[derive(Debug, Copy, Clone)]
pub struct Tuple {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Tuple {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Tuple {
        Tuple {
            x: x,
            y: y,
            z: z,
            w: w,
        }
    }

    pub fn is_point(&self) -> bool {
        // TODO: should we be forcing w back to exactly 0 / 1?
        // approx_eq!(1.0, self.w) // TODO
        (self.w - 1.0).abs() < 0.0001
    }

    pub fn is_vec(&self) -> bool {
        // approx_eq!(1.0, self.w) // TODO
        (self.w).abs() < 0.0001
    }

    pub fn magnitude(&self) -> f32 {
        (self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0)).sqrt()
    }

    pub fn normalize(&self) -> Tuple {
        self / self.magnitude()
    }

    pub fn dot(self, other: Tuple) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    pub fn cross(&self, other: &Tuple) -> Tuple {
        // only implementing 3d version
        vec(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }
}

pub fn point(x: f32, y: f32, z: f32) -> Tuple {
    Tuple::new(x, y, z, 1.0)
}

pub fn vec(x: f32, y: f32, z: f32) -> Tuple {
    Tuple::new(x, y, z, 0.0)
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Tuple) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z && self.w == other.w
    }
}

/// some magic to make ApproxEq work for Tuple
/// based on https://docs.rs/float-cmp/0.6.0/float_cmp/index.html
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

// using https://docs.rs/impl_ops/0.1.1/impl_ops/index.html to avoid lots of boilerplate here
impl_op_ex!(+|a:&Tuple, b:&Tuple| -> Tuple { 
    Tuple::new(a.x + b.x, a.y + b.y, a.z + b.z, a.w + b.w)
});
impl_op_ex!(-|a: &Tuple, b: &Tuple| -> Tuple {
    Tuple::new(a.x - b.x, a.y - b.y, a.z - b.z, a.w - b.w)
});
impl_op_ex!(*|a: &Tuple, b: f32| -> Tuple { Tuple::new(a.x * b, a.y * b, a.z * b, a.w * b) });
impl_op_ex!(/|a: &Tuple, b: f32| -> Tuple {
    Tuple::new(a.x / b, a.y / b, a.z / b, a.w / b)
});
impl_op_ex!(-|a: &Tuple| -> Tuple { Tuple::new(-a.x, -a.y, -a.z, -a.w) });

#[cfg(test)]
mod tests {
    use self::float_cmp::approx_eq;
    use super::*;

    #[test]
    fn tuple_creation() {
        let a = point(4.3, -4.2, 3.1);
        assert!(a.is_point());
        assert!(!a.is_vec());

        let b = vec(4.3, -4.2, 3.1);
        assert!(!b.is_point());
        assert!(b.is_vec());
    }

    #[test]
    fn approx_equality() {
        // using example from https://docs.rs/float-cmp/0.6.0/float_cmp/index.html#the-problem
        // for some reason 0.1+0.2==0.3 works in rust
        let x: f32 = 0.15 + 0.15 + 0.15;
        let y: f32 = 0.1 + 0.1 + 0.25;
        let a = point(x, x, x);
        let b = point(y, y, y);

        assert_ne!(a, b);
        assert!(approx_eq!(Tuple, a, b))
    }

    #[test]
    fn adding_tuples() {
        let a = Tuple::new(3.0, -2.0, 5.0, 1.0);
        let b = Tuple::new(-2.0, 3.0, 1.0, 0.0);

        // adding a point and a vector gives a point
        assert_eq!(Tuple::new(1.0, 1.0, 6.0, 1.0), a + b)
    }

    #[test]
    fn subtracting_tuples() {
        let a = point(3.0, 2.0, 1.0);
        let b = point(5.0, 6.0, 7.0);

        // subtracting two points gives a vector
        assert_eq!(vec(-2.0, -4.0, -6.0), a - b)
    }

    #[test]
    fn negating_tuples() {
        // negating a tuple doesn't negate the w value
        assert_eq!(-point(1.0, 2.0, 3.0), Tuple::new(-1.0, -2.0, -3.0, -1.0));
        assert_eq!(-vec(1.0, 2.0, 3.0), vec(-1.0, -2.0, -3.0));
    }

    #[test]
    fn scalar_multiplication_division() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);

        assert_eq!(Tuple::new(3.5, -7.0, 10.5, -14.0), &a * 3.5);
        assert_eq!(Tuple::new(0.5, -1.0, 1.5, -2.0), &a * 0.5);
        assert_eq!(Tuple::new(0.5, -1.0, 1.5, -2.0), &a / 2.0);
    }

    #[test]
    fn tuple_magnitude() {
        assert_eq!(1.0, vec(1.0, 0.0, 0.0).magnitude());
        assert_eq!(1.0, vec(0.0, 1.0, 0.0).magnitude());
        assert_eq!(1.0, vec(0.0, 0.0, 1.0).magnitude());
        assert_eq!(1.0, point(0.0, 0.0, 1.0).magnitude());

        assert_eq!(14.0_f32.sqrt(), vec(1.0, 2.0, 3.0).magnitude());
        assert_eq!(14.0_f32.sqrt(), vec(-1.0, -2.0, -3.0).magnitude());
    }

    #[test]
    fn vector_normalization() {
        let a = vec(1.0, 1.0, 1.0);
        let b = vec(1.0, 2.0, 3.0);

        assert!(approx_eq!(f32, 1.0, a.normalize().magnitude()));
        assert!(approx_eq!(f32, 1.0, b.normalize().magnitude()));
    }

    #[test]
    fn vector_dot_product() {
        let a = vec(1.0, 2.0, 3.0);
        let b = vec(2.0, 3.0, 4.0);

        assert_eq!(20.0, Tuple::dot(a, b))
    }

    #[test]
    fn vector_cross_product() {
        let a = vec(1.0, 2.0, 3.0);
        let b = vec(2.0, 3.0, 4.0);

        assert_eq!(vec(-1.0, 2.0, -1.0), Tuple::cross(&a, &b));
        assert_eq!(vec(1.0, -2.0, 1.0), Tuple::cross(&b, &a));
    }
}
