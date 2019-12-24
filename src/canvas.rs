use crate::*;

use std::fs;
use std::io;
use std::path::Path;
use std::vec::Vec;

extern crate png;

type WriteResult = Result<(), Box<dyn std::error::Error>>;

pub trait Canvas {
    fn write_pixel(&mut self, c: &Color, x: usize, y: usize);
}

pub struct PpmCanvas {
    width: usize,
    height: usize,
    grid: Vec<Vec<Color>>,
}

impl PpmCanvas {
    fn to_ppm(&self) -> String {
        let mut pixel_data = vec![vec!["0".to_string(); self.width]; self.height];

        for y in 0..self.height {
            for x in 0..self.width {
                let p = self.grid[y][x].clamp().to_u8();
                pixel_data[y][x] = format!("{} {} {}", p.0, p.1, p.2)
            }
        }

        format!(
            "P3
{width} {height}
255
{data}
",
            width = self.width,
            height = self.height,
            data = pixel_data
                .iter()
                .map(|x| x.join(" "))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }

    pub fn new(width: usize, height: usize) -> Self {
        let grid = vec![vec![Color::black(); width]; height];
        PpmCanvas {
            width,
            height,
            grid,
        }
    }
    pub fn write_to_file(&self, filename: &str) -> WriteResult {
        Ok(fs::write(filename, self.to_ppm())?)
    }
}

impl Canvas for PpmCanvas {
    fn write_pixel(&mut self, c: &Color, x: usize, y: usize) {
        &self.grid[y][x].set(c);
    }
}

pub struct PngCanvas {
    width: usize,
    height: usize,
    grid: Vec<Vec<Color>>,
}
impl PngCanvas {
    pub fn new(width: usize, height: usize) -> Self {
        let grid = vec![vec![Color::black(); width]; height];
        PngCanvas {
            width,
            height,
            grid,
        }
    }
    pub fn write_to_file(&self, filename: &str) -> WriteResult {
        // based on code at https://docs.rs/png/0.15.2/png/index.html#using-the-encoder
        let path = Path::new(filename);
        let file = fs::File::create(path)?;
        let ref mut w = io::BufWriter::new(file);

        let mut encoder = png::Encoder::new(w, self.width as u32, self.height as u32);
        encoder.set_color(png::ColorType::RGB);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header()?;

        let mut data = Vec::<u8>::new();

        // todo: could simplify this if PngCanvas owns its own data
        for y in 0..self.height {
            for x in 0..self.width {
                let p = self.grid[y][x].clamp().to_u8();
                data.push(p.0);
                data.push(p.1);
                data.push(p.2);
            }
        }

        Ok(writer.write_image_data(&data)?)
    }
}

impl Canvas for PngCanvas {
    fn write_pixel(&mut self, c: &Color, x: usize, y: usize) {
        self.grid[y][x].set(c);
    }
}

#[cfg(test)]
mod tests {
    mod ppmcanvas {
        use super::super::*;

        #[test]
        fn can_create_basic_ppm_header() {
            let c = PpmCanvas::new(5, 3);
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
            let mut c = PpmCanvas::new(5, 3);
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
            let c = PpmCanvas::new(5, 3);
            let ppm = c.to_ppm();

            assert_eq!('\n', ppm.chars().last().unwrap());
        }
    }
}
