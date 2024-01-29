//! Contains the different shader used by the renderer
use std::default::Default;

use enum_dispatch::enum_dispatch;

use crate::geo::Ray;
use crate::geo::vec3::Vec3;
use crate::material::{AttenuatedColor, HitRecord};
use crate::material::Material;
use crate::material::ScatterType::{Emission, ScatterPdf, ScatterRay};
use crate::renderer::Renderer;

/// Calculates the color from a ray hitting a hittable object
#[enum_dispatch]
pub trait Shader {
    /// Calculate the color of the pixel
    ///
    /// # Arguments
    /// * `renderer` - A reference to the [`Renderer`]
    /// * `rec` - [`HitRecord`] for the current ray hit
    /// * `ray` - The [`Ray`] for the current hit
    /// * `depth` - The recursive depth of the rendering
    /// * `accumulated_ray_length` - Sum of ray length so far including all bounces
    fn shade(
        &self,
        renderer: &Renderer,
        rec: &HitRecord,
        ray: &Ray,
        depth: u32,
        accumulated_ray_length: f64,
    ) -> AttenuatedColor;
}

#[enum_dispatch(Shader)]
#[derive(Clone)]
/// An enum of available shaders
pub enum Shaders {
    /// [`Shader`] of type [`PathTracingShader`]
    PathTracingShaderType(PathTracingShader),
    /// [`Shader`] of type [`AlbedoShader`]
    AlbedoShaderType(AlbedoShader),
    /// [`Shader`] of type [`NormalShader`]
    NormalShaderType(NormalShader),
    /// [`Shader`] of type [`SimpleShader`]
    SimpleShaderType(SimpleShader),
}

#[derive(Clone)]
/// A full raytracing shader
pub struct PathTracingShader {
    max_depth: u32,
}

impl PathTracingShader {
    #![allow(clippy::new_ret_no_self)]
    /// Create a new path tracing shader
    pub fn new(max_depth: u32) -> Shaders {
        Shaders::from(PathTracingShader { max_depth })
    }
}

impl Shader for PathTracingShader {
    /// Calculates the color using path tracing
    fn shade(
        &self,
        renderer: &Renderer,
        rec: &HitRecord,
        ray: &Ray,
        depth: u32,
        accumulated_ray_length: f64,
    ) -> AttenuatedColor {
        if depth >= self.max_depth {
            return AttenuatedColor::default();
        }

        let total_ray_length = rec.ray_length + accumulated_ray_length;
        let scatter_record = rec.material.scatter(ray, rec, &renderer.lights);

        match scatter_record.scatter_type {
            Emission(attenuation_factor) => {
                AttenuatedColor {
                    color: scatter_record.color,
                    attenuation_factor,
                    accumulated_ray_length: total_ray_length,
                }
            }
            ScatterRay(scatter_ray) => {
                let ray_color_res =
                    renderer.ray_color(&scatter_ray, depth + 1, total_ray_length);

                AttenuatedColor {
                    color: scatter_record.color * ray_color_res.pixel_color.color,
                    attenuation_factor: ray_color_res.pixel_color.attenuation_factor,
                    accumulated_ray_length: ray_color_res.pixel_color.accumulated_ray_length,
                }
            }
            ScatterPdf(scatter_ray, scatter_probability) => {
                let ray_color_res = renderer.ray_color(&scatter_ray, depth + 1, total_ray_length);
                let scatter_color = scatter_record.color * scatter_probability * ray_color_res.pixel_color.color;

                AttenuatedColor {
                    color: filter_invalid_color_values(scatter_color),
                    attenuation_factor: ray_color_res.pixel_color.attenuation_factor,
                    accumulated_ray_length: ray_color_res.pixel_color.accumulated_ray_length,
                }
            }
        }
    }
}

fn filter_invalid_color_values(col: Vec3) -> Vec3 {
    Vec3::new(
        filter_color_value(col.x),
        filter_color_value(col.y),
        filter_color_value(col.z),
    )
}

fn filter_color_value(val: f64) -> f64 {
    if val.is_nan() {
        0.
    } else {
        // A subjectively chosen value that is a trade off between
        // color acne and suppressing intensity
        val.min(3.)
    }
}

#[derive(Clone)]
/// Outputs flat color
pub struct AlbedoShader {}

impl AlbedoShader {
    #![allow(clippy::new_ret_no_self)]
    /// Create a new albedo shader
    pub fn new() -> Shaders {
        Shaders::from(AlbedoShader {})
    }
}

impl Shader for AlbedoShader {
    /// Calculates the color only attenuation color
    fn shade(&self, renderer: &Renderer, rec: &HitRecord, ray: &Ray, _: u32, _: f64) -> AttenuatedColor {
        AttenuatedColor {
            color: rec.material.scatter(ray, rec, &renderer.lights).color,
            ..AttenuatedColor::default()
        }
    }
}

#[derive(Clone)]
/// Outputs the normals of the ray hit point
pub struct NormalShader {}

impl NormalShader {
    #![allow(clippy::new_ret_no_self)]
    /// Create a new normal shader
    pub fn new() -> Shaders {
        Shaders::from(NormalShader {})
    }
}

impl Shader for NormalShader {
    /// Calculates the color only using normal
    fn shade(&self, _: &Renderer, rec: &HitRecord, _: &Ray, _: u32, _: f64) -> AttenuatedColor {
        AttenuatedColor {
            color: rec.normal,
            ..AttenuatedColor::default()
        }
    }
}

#[derive(Clone)]
/// A simple shader for quick rendering
pub struct SimpleShader {
    light_dir: Vec3,
}

impl SimpleShader {
    #![allow(clippy::new_ret_no_self)]
    /// Create a new simple shader
    pub fn new() -> Shaders {
        Shaders::from(SimpleShader {
            light_dir: Vec3::new(1., 1., -1.),
        })
    }
}

impl Shader for SimpleShader {
    /// Calculates the color only using normal and attenuation color
    fn shade(&self, renderer: &Renderer, rec: &HitRecord, ray: &Ray, _: u32, _: f64) -> AttenuatedColor {
        let scatter_record = rec.material.scatter(ray, rec, &renderer.lights);
        AttenuatedColor {
            color: match scatter_record.scatter_type {
                Emission(_) => scatter_record.color,
                _ => {
                    // Get a factor to multiply attenuation color, range between .25 -> 1.25
                    // To get some decent flat shading
                    let normal_factor = rec.normal.dot(self.light_dir) * 0.5 + 0.75;

                    scatter_record.color * normal_factor
                }
            },
            ..AttenuatedColor::default()
        }
    }
}
