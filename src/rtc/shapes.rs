use crate::*;

mod sphere;
pub use self::sphere::*;
mod plane;
pub use self::plane::*;

pub trait Shape {
    fn transformation(&self) -> Matrix4;
    fn material(&self) -> PhongMaterial;
    fn local_normal_at(&self, point: Tuple) -> Tuple;
    fn normal_at(&self, point: Tuple) -> Tuple {
        let world_to_sphere = self
            .transformation()
            .try_inverse()
            .expect("Panic! Shape transformation not invertible!");

        let object_point = world_to_sphere * point;
        let object_normal = self.local_normal_at(object_point);
        // https://computergraphics.stackexchange.com/a/1506 for `transpose()` justification
        let mut world_normal = world_to_sphere.transpose() * object_normal;
        world_normal.w = 0.;
        return world_normal.normalize();
    }
}
