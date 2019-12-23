extern crate float_cmp;
pub use self::float_cmp::approx_eq;

// code crudely copied from https://doc.rust-lang.org/src/core/macros.rs.html#78-111
// ... do we need a macro-generating macro here?

#[cfg(test)]
macro_rules! assert_tuple_eq {
        ($left:expr, $right:expr $(, $set:ident = $val:expr)*) => {{
            match (&$left, &$right) {
                (left_val, right_val) => {
                    if !approx_eq!(Tuple, *left_val, *right_val $(, $set = $val)*) {
                        panic!(
                            r#"assertion failed: `(left approxEquals right)`
   left: `{:?}`
  right: `{:?}`"#,
                            &*left_val, &*right_val
                        );
                    }
                }
            }
        }};
    }

#[cfg(test)]
macro_rules! assert_color_eq {
        ($left:expr, $right:expr $(, $set:ident = $val:expr)*) => {{
            match (&$left, &$right) {
                (left_val, right_val) => {
                    if !approx_eq!(Color, *left_val, *right_val $(, $set = $val)*) {
                        panic!(
                            r#"assertion failed: `(left approxEquals right)`
   left: `{:?}`
  right: `{:?}`"#,
                            &*left_val, &*right_val
                        );
                    }
                }
            }
        }};
    }

#[cfg(test)]
macro_rules! assert_ray_eq {
        ($left:expr, $right:expr $(, $set:ident = $val:expr)*) => {{
            match (&$left, &$right) {
                (left_val, right_val) => {
                    if !approx_eq!(Ray, *left_val, *right_val $(, $set = $val)*) {
                        panic!(
                            r#"assertion failed: `(left approxEquals right)`
   left: `{:?}`
  right: `{:?}`"#,
                            &*left_val, &*right_val
                        );
                    }
                }
            }
        }};
    }
