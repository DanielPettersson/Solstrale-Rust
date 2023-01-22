//! Package random provides a random number generator to be used by ray tracer.

/// returns a random float 0 to <1
pub fn random_normal_float() -> f64 {
    fastrand::f64()
}

/// returns a random float min to <max
pub fn random_float(min: f64, max: f64) -> f64 {
    fastrand::f64() * (max - min) + min
}

/// returns a random uint32 0 to <max
pub fn random_uint32(max: u32) -> u32 {
    fastrand::u32(0..max)
}

pub fn random_element_index<T>(v: &Vec<T>) -> usize {
    fastrand::usize(..v.len())
}
