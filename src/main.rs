extern crate rtc;
use rtc::*;
extern crate sdl2;

use std::f32::consts::PI;

use std::io::stdout;
use std::io::Write;

fn create_scene(resolution: usize) -> (World, Camera) {
    let mut material = PhongMaterial::default();
    material.color = Color::new(1., 0.5, 1.);
    let sphere = Sphere::pos_r_m(Tuple::point(0., 0., 0.), 3., material);

    let mut material2 = PhongMaterial::default();
    material2.color = Color::new(0.5, 0.5, 1.);
    let sphere2 = Sphere::pos_r_m(Tuple::point(4., 0., 0.), 1., material);

    let light = PointLight::new(Color::new(1., 1., 1.), Tuple::point(-10., 10., -10.));

    let world = World::new(vec![sphere, sphere2], vec![light]);

    let mut camera = Camera::from_size(resolution, resolution, std::f32::consts::PI / 3.);
    camera.view_transform = view_transform(
        Tuple::point(0., 0., -10.),
        Tuple::point(0., 0., 0.),
        Tuple::vec(0., 1., 0.),
    );

    // TODO: should the world contain the camera and render_to()?
    (world, camera)
}

fn create_scene_2(resolution: usize) -> (World, Camera) {
    let mut floor = Sphere::unit();
    floor.transformation = scaling(10., 0.01, 10.);
    floor.material.color = Color::new(1., 0.9, 0.9);
    floor.material.specular = 0.;

    let mut left_wall = Sphere::unit();
    left_wall.transformation = translation(0., 0., 5.)
        * rotation_y(-PI / 4.)
        * rotation_x(PI / 2.)
        * scaling(10., 0.01, 10.);
    left_wall.material = floor.material;

    let mut right_wall = Sphere::unit();
    right_wall.transformation = translation(0., 0., 5.)
        * rotation_y(PI / 4.)
        * rotation_x(PI / 2.)
        * scaling(10., 0.01, 10.);

    let mut middle_sphere = Sphere::pos_r(Tuple::point(-1.5, 1., 0.5), 1.);
    middle_sphere.material.color = Color::new(0.1, 1., 0.5);
    middle_sphere.material.diffuse = 0.7;
    middle_sphere.material.specular = 0.3;

    let mut right_sphere = Sphere::pos_r(Tuple::point(1.5, 0.5, -0.5), 0.5);
    right_sphere.material.color = Color::new(0.5, 1., 0.1);
    right_sphere.material.diffuse = 0.7;
    right_sphere.material.specular = 0.3;

    let mut left_sphere = Sphere::pos_r(Tuple::point(-1.5, 0.33, -0.75), 0.33);
    left_sphere.material.color = Color::new(1., 0.8, 1.);
    left_sphere.material.diffuse = 0.7;
    left_sphere.material.specular = 0.7;

    let light = PointLight::new(Color::white(), Tuple::point(-10., 10., -10.));

    let world = World::new(
        vec![
            floor,
            left_wall,
            right_wall,
            left_sphere,
            middle_sphere,
            right_sphere,
        ],
        vec![light],
    );
    let camera = Camera::new(
        resolution,
        resolution,
        PI / 3.,
        view_transform(
            Tuple::point(0., 1.5, -5.),
            Tuple::point(0., 1., 0.),
            Tuple::vec(0., 1., 0.),
        ),
    );

    (world, camera)
}

struct SdlCanvas<'a>(&'a mut sdl2::render::WindowCanvas);

impl SdlCanvas<'_> {
    fn present(&mut self) {
        // since SdlCanvas owns a mutable reference to the underlying canvas
        // we can't call canvas.present() from an external function; we need
        // to forward the call through this struct.
        self.0.present();
    }
}

impl Canvas for SdlCanvas<'_> {
    fn write_pixel(&mut self, c: &Color, x: usize, y: usize) {
        let (r, g, b) = c.clamp().to_u8();
        self.0.set_draw_color(sdl2::pixels::Color::RGB(r, g, b));
        self.0
            .draw_point(sdl2::rect::Point::new(x as i32, y as i32))
            .unwrap();
    }
}

fn main() {
    let resolution: usize = 500;
    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();
    let window = video
        .window("rtc", resolution as u32, resolution as u32)
        .build()
        .unwrap();
    let mut timer = sdl.timer().unwrap();
    let mut canvas = window.into_canvas().build().unwrap();

    let (world, mut camera) = create_scene_2(resolution);
    let mut c = SdlCanvas(&mut canvas);

    let mut event_pump = sdl.event_pump().unwrap();
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                _ => {}
            }
        }

        // let ticks = timer.ticks() as f32;
        // let x = (ticks / 1000.).sin();
        // let y = (ticks / 1000.).cos();
        // camera.view_transform = view_transform(
        //     Tuple::point(x, y, -10.),
        //     Tuple::point(0., 0., 0.),
        //     Tuple::vec(0., 1., 0.),
        // );

        camera.render_to(&world, &mut c);
        c.present();
        print!(".");
        stdout().flush().unwrap();
    }

    let mut png_canvas = PngCanvas::new(resolution, resolution);
    camera.render_to(&world, &mut png_canvas);
    png_canvas.write_to_file("output.png").unwrap();
}
