#[macro_use]
mod asserts;
pub use asserts::*;

pub use asserts::*;
mod canvas;
pub use canvas::*;
mod color;
pub use color::*;
mod intersections;
pub use intersections::*;
mod matrixes;
pub use matrixes::*;
mod rays;
pub use rays::*;
mod shapes;
pub use shapes::*;
mod transformations;
pub use transformations::*;
mod tuple;
pub use tuple::*;

#[macro_use]
extern crate impl_ops;
