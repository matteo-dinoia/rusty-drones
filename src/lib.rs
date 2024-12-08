mod checkers;
mod drone;
mod testing_utils;

#[macro_export]
macro_rules! extract {
    ($e:expr, $p:path) => {
        match $e {
            $p(ref value) => Some(value),
            _ => None,
        }
    };
}

#[macro_export]
macro_rules! extract_mut {
    ($e:expr, $p:path) => {
        match $e {
            $p(ref mut value) => Some(value),
            _ => None,
        }
    };
}

pub use drone::RustyDrone;
