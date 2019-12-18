// TODO: should this extern crate still be required?
extern crate rtc;
use rtc::canvas::{Canvas, TestCanvas};
use rtc::color::Color;
use rtc::transformations::{rotation_z, scaling, translation};
use rtc::tuple::Tuple;
use std::fs;

fn write_square<T: Canvas>(canvas: &mut T, color: &Color, x: usize, y: usize) {
    let size = 2;
    for x1 in 0..size {
        for y1 in 0..size {
            canvas.write_pixel(color, x + x1, y + y1);
        }
    }
}

fn main() {
    let canvas_size = 50;
    let mut c = TestCanvas::new(canvas_size, canvas_size);

    let points = (0..12)
        .map(|x| -> Tuple {
            let rotation = rotation_z(((360. / 12.) * (x as f32)).to_radians());
            rotation * Tuple::point(0., 1., 0.)
        })
        .map(|x| -> Tuple {
            let scale = scaling(
                canvas_size as f32 / 2. - 5.,
                canvas_size as f32 / 2. - 5.,
                0.,
            );
            scale * x
        })
        .map(|x| -> Tuple {
            let centering = translation(canvas_size as f32 / 2., canvas_size as f32 / 2., 0.);
            centering * x
        })
        .collect::<Vec<_>>();

    println!("{:?}", points);

    for point in points {
        println!(
            "Writing point at {:?} {:?}",
            point.x as usize, point.y as usize
        );
        write_square(&mut c, &Color::white(), point.x as usize, point.y as usize);
    }

    println!("Done");

    fs::write("output.ppm", c.to_ppm()).unwrap();
}
