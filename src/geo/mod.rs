//! Package geo provides basic geometric constructs
pub mod aabb;
pub mod onb;
pub mod ray;
pub mod vec3;

#[derive(Copy, Clone, Debug)]
pub struct Uv {
    pub u: f64,
    pub v: f64,
}

impl Uv {
    pub fn new(u: f64, v: f64) -> Uv {
        Uv { u, v }
    }
}
