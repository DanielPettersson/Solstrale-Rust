//! Package geo provides basic geometric constructs
pub mod aabb;
pub mod onb;
pub mod ray;
pub mod vec3;

#[derive(Copy, Clone, Debug)]
pub struct Uv {
    pub u: f32,
    pub v: f32,
}

impl Uv {
    pub fn new(u: f32, v: f32) -> Uv {
        Uv { u, v }
    }
}
