use crate::*;

mod sphere;
pub use self::sphere::*;
mod plane;
pub use self::plane::*;

pub trait Shape: std::fmt::Debug {
    // transformation matrix for world space -> Shape's local object space
    fn world_to_object(&self) -> Matrix4;
    fn material(&self) -> PhongMaterial;
    fn local_normal_at(&self, point: Tuple) -> Tuple;
    fn normal_at(&self, point: Tuple) -> Tuple {
        let world_to_object = self.world_to_object();
        let object_point = world_to_object * point;
        let object_normal = self.local_normal_at(object_point);
        // https://computergraphics.stackexchange.com/a/1506 for `transpose()` justification
        let mut world_normal = world_to_object.transpose() * object_normal;
        world_normal.w = 0.;
        return world_normal.normalize();
    }
}
