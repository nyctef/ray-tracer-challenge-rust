use crate::*;

use std::fs;
use std::io;
use std::path::Path;
use std::vec::Vec;

extern crate png;

type WriteResult = Result<(), Box<dyn std::error::Error>>;

pub trait Canvas {
    fn new(width: usize, height: usize) -> Self;
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn pixel_at(&self, x: usize, y: usize) -> &Color;
    fn write_pixel(&mut self, c: &Color, x: usize, y: usize);
    // TODO: try out failure lib: https://stackoverflow.com/questions/48430836/rust-proper-error-handling-auto-convert-from-one-error-type-to-another-with-que
    fn write_to_file(&self, filename: &str) -> WriteResult;
}

pub struct PpmCanvas {
    width: usize,
    height: usize,
    grid: Vec<Vec<Color>>,
}

impl PpmCanvas {
    pub fn to_ppm(&self) -> String {
        let mut pixel_data = vec![vec!["0".to_string(); self.width()]; self.height()];

        for y in 0..self.height() {
            for x in 0..self.width() {
                let p = self.pixel_at(x, y).clamp().to_u8();
                pixel_data[y][x] = format!("{} {} {}", p.0, p.1, p.2)
            }
        }

        format!(
            "P3
{width} {height}
255
{data}
",
            width = self.width(),
            height = self.height(),
            data = pixel_data
                .iter()
                .map(|x| x.join(" "))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

impl Canvas for PpmCanvas {
    fn new(width: usize, height: usize) -> Self {
        let grid = vec![vec![Color::black(); width]; height];
        PpmCanvas {
            width,
            height,
            grid,
        }
    }
    fn width(&self) -> usize {
        self.width
    }
    fn height(&self) -> usize {
        self.height
    }
    fn pixel_at(&self, x: usize, y: usize) -> &Color {
        &self.grid[y][x]
    }
    fn write_pixel(&mut self, c: &Color, x: usize, y: usize) {
        &self.grid[y][x].set(c);
    }
    fn write_to_file(&self, filename: &str) -> WriteResult {
        Ok(fs::write(filename, self.to_ppm())?)
    }
}

#[allow(unused_macros)]
macro_rules! canvas_tests {
    ($canvasType: ty) => {
        #[test]
        fn canvas_creation() {
            let c: $canvasType = Canvas::new(30, 40);
            assert_eq!(30, c.width());
            assert_eq!(40, c.height());
        }

        #[test]
        fn uninitialized_pixel_is_black() {
            let c: $canvasType = Canvas::new(10, 10);
            assert_eq!(&Color::black(), c.pixel_at(3, 3));
        }

        #[test]
        fn can_write_pixel_to_canvas() {
            let mut c: $canvasType = Canvas::new(10, 10);
            c.write_pixel(&Color::red(), 2, 3);
            assert_eq!(&Color::red(), c.pixel_at(2, 3));
        }
    };
}

pub struct PngCanvas {
    // TODO: should probably pull out a common basic canvas rather than reusing this
    inner: PpmCanvas,
}

impl Canvas for PngCanvas {
    fn new(width: usize, height: usize) -> Self {
        PngCanvas {
            inner: PpmCanvas::new(width, height),
        }
    }
    fn width(&self) -> usize {
        self.inner.width()
    }
    fn height(&self) -> usize {
        self.inner.height()
    }
    fn pixel_at(&self, x: usize, y: usize) -> &Color {
        self.inner.pixel_at(x, y)
    }
    fn write_pixel(&mut self, c: &Color, x: usize, y: usize) {
        self.inner.write_pixel(c, x, y);
    }
    fn write_to_file(&self, filename: &str) -> WriteResult {
        // based on code at https://docs.rs/png/0.15.2/png/index.html#using-the-encoder
        let path = Path::new(filename);
        let file = fs::File::create(path)?;
        let ref mut w = io::BufWriter::new(file);

        let mut encoder = png::Encoder::new(w, self.width() as u32, self.height() as u32);
        encoder.set_color(png::ColorType::RGB);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header()?;

        let mut data = Vec::<u8>::new();

        // todo: could simplify this if PngCanvas owns its own data
        for y in 0..self.height() {
            for x in 0..self.width() {
                let p = self.pixel_at(x, y).clamp().to_u8();
                data.push(p.0);
                data.push(p.1);
                data.push(p.2);
            }
        }

        Ok(writer.write_image_data(&data)?)
    }
}

#[cfg(test)]
mod tests {
    mod ppmcanvas {
        use super::super::*;
        canvas_tests!(PpmCanvas);

        #[test]
        fn can_create_basic_ppm_header() {
            let c: PpmCanvas = Canvas::new(5, 3);
            let ppm = c.to_ppm();
            let header_lines = ppm.split("\n").take(3).collect::<Vec<_>>();

            // the first line specifies the type of netpbm file
            assert_eq!("P3", header_lines[0]);
            // the second line defines the size of the image
            assert_eq!("5 3", header_lines[1]);
            // the third line defines the maximum value of the pixel data
            assert_eq!("255", header_lines[2]);
        }

        #[test]
        fn can_write_ppm_pixel_data() {
            let mut c: PpmCanvas = Canvas::new(5, 3);
            let c1 = Color::new(1.5, 0.0, 0.0);
            let c2 = Color::new(0.0, 0.5, 0.0);
            let c3 = Color::new(-0.5, 0.0, 1.0);

            c.write_pixel(&c1, 0, 0);
            c.write_pixel(&c2, 2, 1);
            c.write_pixel(&c3, 4, 2);

            let ppm = c.to_ppm();
            let data_lines = ppm.split("\n").skip(3).take(3).collect::<Vec<_>>();

            assert_eq!("255 0 0 0 0 0 0 0 0 0 0 0 0 0 0", data_lines[0]);
            assert_eq!("0 0 0 0 0 0 0 128 0 0 0 0 0 0 0", data_lines[1]);
            assert_eq!("0 0 0 0 0 0 0 0 0 0 0 0 0 0 255", data_lines[2]);
        }

        #[test]
        #[ignore]
        fn ppm_pixel_data_wrapped_to_70_chars() {
            unimplemented!();
        }

        #[test]
        fn ppm_ends_with_newline_char() {
            let c: PpmCanvas = Canvas::new(5, 3);
            let ppm = c.to_ppm();

            assert_eq!('\n', ppm.chars().last().unwrap());
        }
    }

    mod pngcanvas {
        use super::super::*;
        canvas_tests!(PngCanvas);
    }
}
