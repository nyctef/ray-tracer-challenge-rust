extern crate rtc;
use rtc::RaySphereIntersection::*;
use rtc::*;
use std::fs;

fn main() {
    // TODO: decouple resolution and "camera canvas size"
    let canvas_size = 500;
    let cs2 = (canvas_size as f32) / 2.;
    let mut c = TestCanvas::new(canvas_size, canvas_size);

    let mut material = PhongMaterial::default();
    material.color = Color::new(1., 0.2, 1.);
    let sphere = Sphere::pos_r_m(
        Tuple::point(0., 0., 0.),
        4.5, /*canvas_size as f32 / 20.*/
        material,
    );

    let light = PointLight::new(Color::new(1., 1., 1.), Tuple::point(-10., 10., -10.));

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
            // println!("{:?}", ray);
            // TODO: how to reduce nesting here?
            let intersects = sphere.ray_intersection(ray);
            match intersects {
                Misses => continue,
                Intersects(intersections) => {
                    match Intersection::hit(&intersections) {
                        None => continue,
                        Some(h) => {
                            // todo should really get this from the sphere in the hit
                            // rather than the one sphere we know is in the scene
                            let point = ray.position(h.t);
                            let n = sphere.normal_at(point);
                            let eye = -ray.direction;
                            let color = lighting(sphere.material, light, point, eye, n);
                            c.write_pixel(&color, x, canvas_size - y - 1);
                        }
                    };
                }
            }
        }
    }

    println!("Done");

    // TODO: write to screen or png instead of ppm
    fs::write("output.ppm", c.to_ppm()).unwrap();
}
