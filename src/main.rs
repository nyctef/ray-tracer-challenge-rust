extern crate rtc;
use rtc::RaySphereIntersection::*;
use rtc::*;
use std::fs;

fn main() {
    let canvas_size = 50;
    let cs2 = (canvas_size as f32) / 2.;
    let mut c = TestCanvas::new(canvas_size, canvas_size);

    let sphere = Sphere::pos_r(Tuple::point(0., 0., 0.), 2.);
    let camera_origin = Tuple::point(0., 0., -5.);
    let canvas_z = 10_f32;

    for x in 0..canvas_size {
        for y in 0..canvas_size {
            let x2 = (x as f32) - cs2;
            let y2 = (y as f32) - cs2;
            let ray = Ray::new(
                camera_origin,
                (Tuple::point(x2, y2, canvas_z) - camera_origin).normalize(),
            );
            println!("{:?}", ray);
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
