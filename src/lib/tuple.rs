type Tuple = (f32, f32, f32, f32);

fn tuple(x: f32, y: f32, z: f32, w: f32) -> Tuple {
    (x, y, z, w)
}

fn point(x: f32, y: f32, z: f32) -> Tuple {
    tuple(x, y, z, 1.0)
}

fn vec(x: f32, y: f32, z: f32) -> Tuple {
    tuple(x, y, z, 0.0)
}

// TODO: does w need an approximate comparison?
fn is_point(x: Tuple) -> bool {
    x.3 == 1.0
}

fn is_vec(x: Tuple) -> bool {
    x.3 == 0.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tuple_creation() {
        let a = point(4.3, -4.2, 3.1);
        assert!(is_point(a));
        assert!(!is_vec(a));

        let b = vec(4.3, -4.2, 3.1);
        assert!(!is_point(b));
        assert!(is_vec(b));
    }
}
