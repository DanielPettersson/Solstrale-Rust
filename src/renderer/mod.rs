use crate::geo::ray::Ray;
use crate::geo::vec3::{Vec3, ZERO_VECTOR};
use crate::hittable::hittable_list::HittableList;

pub mod shader;

pub struct Renderer {
    pub lights: HittableList,
}

impl Renderer {
    pub fn ray_color(&self, _: Ray, _: i32) -> (Vec3, Vec3, Vec3) {
        (ZERO_VECTOR, ZERO_VECTOR, ZERO_VECTOR)
    }
}
