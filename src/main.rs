// TODO: should this extern crate still be required?
extern crate rtc;
use rtc::canvas::{Canvas, TestCanvas};
use rtc::color::Color;
use rtc::tuple::Tuple;
use std::fs;

#[derive(Debug)]
struct Environment {
    gravity: Tuple,
    wind: Tuple,
}

#[derive(Debug)]
struct Projectile {
    position: Tuple,
    velocity: Tuple,
}

fn tick(env: &Environment, proj: &Projectile) -> Projectile {
    let position = &proj.position + &proj.velocity;
    let velocity = &proj.velocity + &env.gravity + &env.wind;
    Projectile { position, velocity }
}

fn main() {
    let mut p = Projectile {
        position: Tuple::point(0.0, 0.1, 0.0),
        velocity: Tuple::vec(1.0, 1.8, 0.0).normalize() * 11.25,
    };
    let e = Environment {
        gravity: Tuple::vec(0.0, -0.1, 0.0),
        wind: Tuple::vec(-0.01, 0.0, 0.0),
    };
    let mut c = TestCanvas::new(900, 550);

    while p.position.y > 0.0 {
        println!("{:?}", p);
        c.write_pixel(
            &Color::red(),
            p.position.x as usize,
            549 - (p.position.y as usize),
        );
        p = tick(&e, &p);
    }

    println!("Done");

    fs::write("output.ppm", c.to_ppm()).unwrap();
}
