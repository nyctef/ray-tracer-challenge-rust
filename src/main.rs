extern crate rtc;
use rtc::*;
extern crate sdl2;

use std::f32::consts::PI;

fn create_scene_2(resolution: usize) -> (World, Camera) {
    let mut floor = Plane::xz();
    floor.material.pattern = Pattern::Stripe(Stripe::new(
        red(),
        white(),
        rotation_y(PI / 4.) * scaling(0.1, 0.1, 0.1),
    ));
    floor.material.specular = 0.;

    let mut left_wall =
        Plane::t(translation(0., 0., 5.) * rotation_y(-PI / 4.) * rotation_x(PI / 2.));
    left_wall.material = floor.material.clone();
    left_wall.material.pattern = Pattern::Stripe(Stripe::col(red(), white()));

    let mut right_wall =
        Plane::t(translation(0., 0., 5.) * rotation_y(PI / 4.) * rotation_x(PI / 2.));
    right_wall.material.pattern = Pattern::Stripe(Stripe::col(red(), white()));

    let mut middle_sphere = Sphere::pos_r(point(-1.5, 1., 0.5), 1.);
    middle_sphere.material.pattern =
        Pattern::SphereMap(SphereMap::col(Pattern::Checkerboard(Checkerboard::new(
            black(),
            Color::new(0.85, 0.9, 1.1),
            scaling(0.05, 0.05, 0.05),
        ))));
    middle_sphere.material.diffuse = 0.7;
    middle_sphere.material.specular = 0.3;

    let mut right_sphere = Sphere::pos_r(point(1.5, 0.5, -0.5), 0.5);
    right_sphere.material.pattern = solid(Color::new(0.5, 1., 0.1));
    right_sphere.material.diffuse = 0.7;
    right_sphere.material.specular = 0.3;

    let mut left_sphere = Sphere::pos_r(point(-1.5, 0.33, -0.75), 0.33);
    left_sphere.material.pattern = solid(Color::new(1., 0.8, 1.));
    left_sphere.material.diffuse = 0.7;
    left_sphere.material.specular = 0.7;

    let light = PointLight::new(white(), point(-10., 10., -10.));

    let world = World::new(
        vec![
            Box::new(floor),
            Box::new(left_wall),
            Box::new(right_wall),
            Box::new(left_sphere),
            Box::new(middle_sphere),
            Box::new(right_sphere),
        ],
        vec![light],
    );
    let camera = Camera::new(
        resolution,
        resolution,
        PI / 3.,
        view_transform(point(0., 1.5, -5.), point(0., 1., 0.), vec(0., 1., 0.)),
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

fn draw_to_screen(resolution: usize, camera: &Camera, world: &World) {
    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();
    let window = video
        .window("rtc", resolution as u32, resolution as u32)
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let mut c = SdlCanvas(&mut canvas);
    camera.render_to(world, &mut c);
    c.present();

    let mut event_pump = sdl.event_pump().unwrap();
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                _ => {}
            }
        }
    }
}

fn main() {
    let resolution: usize = 500;
    let (world, camera) = create_scene_2(resolution);

    if false {
        draw_to_screen(resolution, &camera, &world)
    }

    let mut png_canvas = PngCanvas::new(resolution, resolution);
    camera.render_to(&world, &mut png_canvas);
    png_canvas.write_to_file("output.png").unwrap();
}
