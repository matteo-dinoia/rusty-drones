#[macro_export]
macro_rules! extract {
    ($e:expr, $p:path) => {
        match &$e {
            $p(ref value) => Some(value),
            _ => None,
        }
    };
}
