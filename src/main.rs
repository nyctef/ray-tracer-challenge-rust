// TODO: should this extern crate still be required?
extern crate rtc;
use rtc::tuple::Tuple;

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
        velocity: Tuple::vec(1.0, 1.0, 0.0).normalize(),
    };
    let e = Environment {
        gravity: Tuple::vec(0.0, -0.1, 0.0),
        wind: Tuple::vec(-0.01, 0.0, 0.0),
    };

    while p.position.y > 0.0 {
        println!("{:?}", p);
        p = tick(&e, &p);
    }

    println!("Done")
}
