extern crate rtc;
use rtc::*;

fn main() {
    let resolution = 500;
    let mut c = PngCanvas::new(resolution, resolution);

    let mut material = PhongMaterial::default();
    material.color = Color::new(1., 0.2, 1.);
    let sphere = Sphere::pos_r_m(Tuple::point(0., 0., 0.), 3., material);

    let light = PointLight::new(Color::new(1., 1., 1.), Tuple::point(-10., 10., -10.));

    let camera_origin = Tuple::point(0., 0., -5.);
    let canvas_size = 50.;
    let canvas_z = 10_f32;

    let px_per_canvas_width = resolution as f32 / canvas_size;

    for x in 0..resolution {
        for y in 0..resolution {
            let x2 = (x as f32) / px_per_canvas_width - (canvas_size / 2.);
            let y2 = (y as f32) / px_per_canvas_width - (canvas_size / 2.);
            // println!("{} {}", x2, y2);
            let ray = Ray::new(
                camera_origin,
                (Tuple::point(x2, y2, canvas_z) - camera_origin).normalize(),
            );

            light_ray(ray, sphere).map(|hit| {
                let eye = -ray.direction;
                let color = lighting(hit.material, light, hit.point, eye, hit.surface_normal);

                c.write_pixel(&color, x, resolution - y - 1);
            });
        }
    }

    println!("Done");

    c.write_to_file("output.png").unwrap();
}
