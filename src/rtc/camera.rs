use crate::*;

#[derive(Debug, Clone)]
pub struct Camera {
    pub hsize: usize,
    pub vsize: usize,
    // field of view angle in radians
    pub fov: f32,
    pub view_transform: Matrix4,
    pub half_width: f32,
    pub half_height: f32,
    pub pixel_size: f32,
}

impl Camera {
    pub fn new(hsize: usize, vsize: usize, fov: f32, view_transform: Matrix4) -> Camera {
        // - for this camera we assume the "canvas" is one unit away.
        // - fov/2 gives us the angle between edge and center of the canvas.
        // - since tan(fov/2) gives us the ratio of the opposite/adjacent tri sides,
        //   and we define the adjacent to be one unit long, half_view becomes
        //   half of the canvas size
        let half_view = (fov / 2.).tan();
        let aspect_ratio = hsize as f32 / vsize as f32;

        // let's say that fov describes a circle which our canvas fits into.
        let (half_width, half_height) = if aspect_ratio >= 1. {
            // when width > height, then the width fills the diameter of the circle
            // and the height must be smaller than the width
            (half_view, half_view / aspect_ratio)
        } else {
            // and vice-versa
            (half_view * aspect_ratio, half_view)
        };

        // the pixel size is the full width of the canvas divided by the width in pixels
        // we assume square pixels so the height calculation would be equivalent
        let pixel_size = (half_width * 2.) / hsize as f32;

        Camera {
            hsize,
            vsize,
            fov,
            view_transform,
            half_width,
            half_height,
            pixel_size,
        }
    }

    pub fn from_size(hsize: usize, vsize: usize, fov: f32) -> Camera {
        Camera::new(hsize, vsize, fov, Matrix4::identity())
    }

    pub fn ray_for_pixel(&self, x: usize, y: usize) -> Ray {
        // the offset from the edge of the canvas to the pixel's *center*
        let xoffset = (x as f32 + 0.5) * self.pixel_size;
        let yoffset = (y as f32 + 0.5) * self.pixel_size;

        // the default camera looks towards -z, so we subtract these values
        let pixel_world_x = self.half_width - xoffset;
        let pixel_world_y = self.half_height - yoffset;

        let camera_to_world = self
            .view_transform
            .try_inverse()
            .expect("Panic! Camera transform can't be inverted!");

        let pixel_pos = camera_to_world * Tuple::point(pixel_world_x, pixel_world_y, -1.);
        let origin = camera_to_world * Tuple::point(0., 0., 0.);
        let direction = (pixel_pos - origin).normalize();

        Ray::new(origin, direction)
    }

    pub fn render_to<T: Canvas>(&self, world: &World, canvas: &mut T) {
        for y in 0..self.vsize - 1 {
            for x in 0..self.hsize - 1 {
                let ray = self.ray_for_pixel(x, y);
                let color = color_at(world, ray);
                canvas.write_pixel(&color, x, y);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::PI;

    #[test]
    fn pixel_size_for_horizontal_canvas() {
        let c = Camera::from_size(200, 125, PI / 2.);
        assert_eq!(0.01, c.pixel_size)
    }
    #[test]
    fn pixel_size_for_vertical_canvas() {
        let c = Camera::from_size(125, 200, PI / 2.);
        assert_eq!(0.01, c.pixel_size)
    }

    #[test]
    fn ray_for_pixel_at_center() {
        let c = Camera::from_size(201, 101, PI / 2.);
        let r = c.ray_for_pixel(100, 50);
        assert_ray_eq!(
            Ray::new(Tuple::point(0., 0., 0.), Tuple::vec(0., 0., -1.)),
            r
        );
    }

    #[test]
    fn ray_for_pixel_at_corner() {
        let c = Camera::from_size(201, 101, PI / 2.);
        let r = c.ray_for_pixel(0, 0);
        assert_ray_eq!(
            Ray::new(
                Tuple::point(0., 0., 0.),
                Tuple::vec(0.66519, 0.33259, -0.66851)
            ),
            r,
            epsilon = 0.00001
        );
    }

    #[test]
    fn ray_for_pixel_at_with_transformed_camera() {
        let mut c = Camera::from_size(201, 101, PI / 2.);
        c.view_transform = rotation_y(PI / 4.) * translation(0., -2., 5.);
        let r = c.ray_for_pixel(100, 50);
        let s22 = 2_f32.sqrt() / 2.;
        assert_ray_eq!(
            Ray::new(Tuple::point(0., 2., -5.), Tuple::vec(s22, 0., -s22)),
            r
        );
    }
}
