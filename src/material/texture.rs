use crate::geo::vec3::Vec3;
use crate::material::HitRecord;
use enum_dispatch::enum_dispatch;

/// Describes the color of a material.
/// The color can vary by the uv coordinates of the hittable
#[enum_dispatch]
pub trait Texture {
    fn color(&self, rec: &HitRecord) -> Vec3;
}

#[enum_dispatch(Texture)]
pub enum Textures {
    SolidColor(SolidColor),
}

/// A texture with just the same color everywhere
pub struct SolidColor(Vec3);

impl SolidColor {
    pub fn new(r: f64, g: f64, b: f64) -> Textures {
        Textures::SolidColor(SolidColor(Vec3::new(r, g, b)))
    }
}

impl Texture for SolidColor {
    fn color(&self, _: &HitRecord) -> Vec3 {
        self.0
    }
}
