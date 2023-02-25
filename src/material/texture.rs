use std::error::Error;
use std::sync::Arc;

use enum_dispatch::enum_dispatch;
use image::RgbImage;

use crate::geo::vec3::Vec3;
use crate::material::texture::Textures::{ImageTextureType, SolidColorType};
use crate::material::HitRecord;
use crate::util::rgb_color::rgb_to_vec3;

/// Describes the color of a material.
/// The color can vary by the uv coordinates of the hittable
#[enum_dispatch]
pub trait Texture {
    fn color(&self, rec: &HitRecord) -> Vec3;
}

#[enum_dispatch(Texture)]
#[derive(Debug)]
pub enum Textures {
    SolidColorType(SolidColor),
    ImageTextureType(ImageTexture),
}

impl Clone for Textures {
    fn clone(&self) -> Self {
        match self {
            SolidColorType(t) => SolidColorType(t.clone()),
            ImageTextureType(t) => ImageTextureType(t.clone()),
        }
    }
}

/// A texture with just the same color everywhere
#[derive(Clone, Debug)]
pub struct SolidColor(Vec3);

impl SolidColor {
    pub fn create(r: f64, g: f64, b: f64) -> Textures {
        SolidColor::from_vec3(Vec3::new(r, g, b))
    }
    pub fn from_vec3(color: Vec3) -> Textures {
        SolidColorType(SolidColor(color))
    }
}

impl Texture for SolidColor {
    fn color(&self, _: &HitRecord) -> Vec3 {
        self.0
    }
}

#[derive(Clone, Debug)]
pub struct ImageTexture {
    image: Arc<RgbImage>,
    mirror: bool,
    max_x: f64,
    max_y: f64,
}

impl ImageTexture {
    /// Creates a texture that uses image data for color by loading the image from the path
    pub fn load(path: &str) -> Result<Textures, Box<dyn Error>> {
        let image = image::open(path)
            .unwrap_or_else(|_| panic!("Failed to load image texture {}", path))
            .into_rgb8();
        Ok(Self::create(Arc::new(image), false))
    }

    /// Creates a texture that uses image data for color
    pub fn create(image: Arc<RgbImage>, mirror: bool) -> Textures {
        let w = image.width();
        let h = image.height();
        ImageTextureType(ImageTexture {
            image,
            mirror,
            max_x: w as f64 - 1.,
            max_y: h as f64 - 1.,
        })
    }
}

impl Texture for ImageTexture {
    /// Returns the color in the image data that corresponds to the UV coordinate of the hittable
    /// If UV coordinates from hit record is <0 or >1 texture wraps
    fn color(&self, rec: &HitRecord) -> Vec3 {
        let mut u = rec.u.abs() % 1.;
        if self.mirror {
            u = 1. - u
        }
        let v = 1. - rec.v.abs() % 1.;

        let x = u * self.max_x;
        let y = v * self.max_y;

        let pixel = self.image.get_pixel(x as u32, y as u32);
        rgb_to_vec3(pixel)
    }
}
