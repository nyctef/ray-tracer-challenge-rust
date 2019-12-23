extern crate rtc;
use rtc::*;

fn main() {
    let resolution = 500;
    let mut c = PngCanvas::new(resolution, resolution);

    let mut material = PhongMaterial::default();
    material.color = Color::new(1., 0.5, 1.);
    let sphere = Sphere::pos_r_m(Tuple::point(0., 0., 0.), 3., material);

    let light = PointLight::new(Color::new(1., 1., 1.), Tuple::point(-10., 10., -10.));

    let world = World::new(vec![sphere], vec![light]);

    let mut camera = Camera::from_size(500, 500, std::f32::consts::PI / 3.);
    camera.view_transform = view_transform(
        Tuple::point(0., 0., -10.),
        Tuple::point(0., 0., 0.),
        Tuple::vec(0., 1., 0.),
    );

    // TODO: should the world contain the camera and render_to()?
    camera.render_to(&world, &mut c);

    println!("Done");

    c.write_to_file("output.png").unwrap();
}
