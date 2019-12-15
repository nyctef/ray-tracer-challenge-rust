use color::Color;
use std::vec::Vec;

pub trait Canvas {
    fn new(width: usize, height: usize) -> Self;
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn pixel_at(&self, x: usize, y: usize) -> &Color;
}

pub struct TestCanvas {
    width: usize,
    height: usize,
    grid: Vec<Vec<Color>>,
}

impl Canvas for TestCanvas {
    fn new(width: usize, height: usize) -> Self {
        let grid = vec![vec![Color::black(); width]; height];
        TestCanvas {
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
        &self.grid[x][y]
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
    };
}

#[cfg(test)]
mod tests {
    mod test_canvas {
        use super::super::*;
        canvas_tests!(TestCanvas);
    }
}
