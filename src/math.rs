#[cfg(feature = "libm")]
pub use libm::{ceil, ceilf, exp10, exp10f, floor, floorf, log10, log10f};

#[cfg(all(feature = "std", not(feature = "libm")))]
pub fn ceil(x: f64) -> f64 {
    x.ceil()
}

#[cfg(all(feature = "std", not(feature = "libm")))]
pub fn ceilf(x: f32) -> f32 {
    x.ceil()
}

#[cfg(all(feature = "std", not(feature = "libm")))]
pub fn exp10(x: f64) -> f64 {
    10f64.powf(x)
}

#[cfg(all(feature = "std", not(feature = "libm")))]
pub fn exp10f(x: f32) -> f32 {
    10f32.powf(x)
}

#[cfg(all(feature = "std", not(feature = "libm")))]
pub fn floor(x: f64) -> f64 {
    x.floor()
}

#[cfg(all(feature = "std", not(feature = "libm")))]
pub fn floorf(x: f32) -> f32 {
    x.floor()
}

#[cfg(all(feature = "std", not(feature = "libm")))]
pub fn log10(x: f64) -> f64 {
    x.log10()
}

#[cfg(all(feature = "std", not(feature = "libm")))]
pub fn log10f(x: f32) -> f32 {
    x.log10()
}
