pub trait Canvas {
    fn new(width: i32, height: i32) -> Self;
    fn width(&self) -> i32;
    fn height(&self) -> i32;
}

pub struct TestCanvas {
    width: i32,
    height: i32,
}

impl Canvas for TestCanvas {
    fn new(width: i32, height: i32) -> Self {
        TestCanvas { width, height }
    }
    fn width(&self) -> i32 {
        self.width
    }
    fn height(&self) -> i32 {
        self.height
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn canvas_creation() {
        let c: TestCanvas = Canvas::new(30, 40);

        assert_eq!(30, c.width());
        assert_eq!(40, c.height());
    }
}
