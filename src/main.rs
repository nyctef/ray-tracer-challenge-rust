// TODO: should this extern crate still be required?
extern crate rtc;
use rtc::RaySphereIntersection::*;
use rtc::*;
use std::fs;

fn main() {
    let canvas_size = 50;
    let mut c = TestCanvas::new(canvas_size, canvas_size);

    let sphere = Sphere::pos_r(Tuple::point(25., 25., 0.), 20.);

    for x in 0..canvas_size {
        for y in 0..canvas_size {
            let ray = Ray::new(
                Tuple::point(x as f32, y as f32, -50.),
                Tuple::vec(0., 0., 1.),
            );
            let intersects = sphere.ray_intersection(ray);
            match intersects {
                Misses => c.write_pixel(&Color::black(), x, y),
                Intersects(_) => c.write_pixel(&Color::red(), x, y),
            }
        }
    }

    println!("Done");

    fs::write("output.ppm", c.to_ppm()).unwrap();
}
