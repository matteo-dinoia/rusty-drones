pub mod drone;
mod integration_testing;

pub use wg_2024::drone::Drone;

#[macro_export]
macro_rules! extract {
    ($e:expr, $p:path) => {
        match &$e {
            $p(ref value) => Some(value),
            _ => None,
        }
    };
}
