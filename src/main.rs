extern crate rtc;
use rtc::*;
extern crate sdl2;

fn create_scene(resolution: usize) -> (World, Camera) {
    let mut material = PhongMaterial::default();
    material.color = Color::new(1., 0.5, 1.);
    let sphere = Sphere::pos_r_m(Tuple::point(0., 0., 0.), 3., material);

    let light = PointLight::new(Color::new(1., 1., 1.), Tuple::point(-10., 10., -10.));

    let world = World::new(vec![sphere], vec![light]);

    let mut camera = Camera::from_size(resolution, resolution, std::f32::consts::PI / 3.);
    camera.view_transform = view_transform(
        Tuple::point(0., 0., -10.),
        Tuple::point(0., 0., 0.),
        Tuple::vec(0., 1., 0.),
    );

    // TODO: should the world contain the camera and render_to()?
    (world, camera)
}

struct SdlCanvas<'a>(&'a mut sdl2::render::WindowCanvas);

impl Canvas for SdlCanvas<'_> {
    fn new(width: usize, height: usize) -> Self {
        // TODO: should this be in the trait if we can't implement it here?
        panic!("Can't create a SdlCanvas without a window parent, sorry")
    }
    fn width(&self) -> usize {
        0 // TODO
    }
    fn height(&self) -> usize {
        0 // TODO
    }
    fn pixel_at(&self, x: usize, y: usize) -> &Color {
        panic!(); // TODO
    }
    fn write_pixel(&mut self, c: &Color, x: usize, y: usize) {
        // maybe this is the only really important method on the trait?
        let (r, g, b) = c.to_u8();
        self.0.set_draw_color(sdl2::pixels::Color::RGB(r, g, b));
        self.0
            .draw_point(sdl2::rect::Point::new(x as i32, y as i32))
            .unwrap();
    }

    fn write_to_file(&self, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
        // TODO
        Ok(())
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
    let mut canvas = window.into_canvas().build().unwrap();

    let (world, camera) = create_scene(resolution);
    let mut c = SdlCanvas(&mut canvas);
    camera.render_to(&world, &mut c);
    canvas.present();

    //event loop
    let mut event_pump = sdl.event_pump().unwrap();
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                _ => {}
            }
        }
    }

    // TODO: render to screen
    // TODO: render two spheres and try a basic animation
    //       eg sway the camera based on a sine wave
}
