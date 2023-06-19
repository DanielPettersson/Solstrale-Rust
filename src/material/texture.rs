//! Contains textures to be used by [`Material`]
use std::error::Error;
use std::sync::Arc;

use crate::geo::Uv;
use enum_dispatch::enum_dispatch;
use image::RgbImage;
use simple_error::SimpleError;

use crate::geo::vec3::Vec3;
use crate::material::texture::Textures::{ImageTextureType, SolidColorType};
use crate::util::rgb_color::rgb_to_vec3;

/// Describes the color of a material.
/// The color can vary by the uv coordinates of the hittable
#[enum_dispatch]
pub trait Texture {
    /// Return the color of the texture at a given hit
    fn color(&self, uv: Uv) -> Vec3;
}

#[enum_dispatch(Texture)]
#[derive(Debug)]
/// An enum of textures
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
    #![allow(clippy::new_ret_no_self)]
    /// Create a new solid color texture
    pub fn new(r: f64, g: f64, b: f64) -> Textures {
        SolidColor::new_from_vec3(Vec3::new(r, g, b))
    }
    pub fn new_from_f32_array(c: [f32; 3]) -> Textures {
        SolidColor::new(c[0] as f64, c[1] as f64, c[2] as f64)
    }
    /// Create a new solid color texture from a [`Vec3`]
    pub fn new_from_vec3(color: Vec3) -> Textures {
        SolidColorType(SolidColor(color))
    }
}

impl Texture for SolidColor {
    fn color(&self, _: Uv) -> Vec3 {
        self.0
    }
}

/// Texture that uses image data for color by loading the image from the path
#[derive(Clone, Debug)]
pub struct ImageTexture {
    image: Arc<RgbImage>,
    max_x: f32,
    max_y: f32,
}

impl ImageTexture {
    #![allow(clippy::new_ret_no_self)]
    /// Creates a new image texture from a file path
    pub fn load(path: &str) -> Result<Textures, Box<dyn Error>> {
        let image = image::open(path)
            .map_err(|_| SimpleError::new(format!("Failed to load image texture {}", path)))?
            .into_rgb8();
        Ok(Self::new(Arc::new(image)))
    }

    /// Creates a texture that uses image data for color
    pub fn new(image: Arc<RgbImage>) -> Textures {
        let w = image.width();
        let h = image.height();
        ImageTextureType(ImageTexture {
            image,
            max_x: w as f32 - 1.,
            max_y: h as f32 - 1.,
        })
    }
}

impl Texture for ImageTexture {
    /// Returns the color in the image data that corresponds to the UV coordinate of the hittable
    /// If UV coordinates from hit record is <0 or >1 texture wraps
    fn color(&self, uv: Uv) -> Vec3 {
        let u = uv.u.abs() % 1.;
        let v = 1. - uv.v.abs() % 1.;

        let x = u * self.max_x;
        let y = v * self.max_y;

        let pixel = self.image.get_pixel(x as u32, y as u32);
        rgb_to_vec3(pixel)
    }
}
