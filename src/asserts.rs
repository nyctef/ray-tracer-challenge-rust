extern crate float_cmp;
pub use self::float_cmp::approx_eq;

// TODO: learn more about how this assert is put together
// (code crudely copied from https://doc.rust-lang.org/src/core/macros.rs.html#78-111)
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
