//! Contains the different shader used by the renderer
use crate::geo::vec3::Vec3;
use crate::geo::Ray;
use crate::material::Material;
use crate::material::ScatterType::{ScatterPdf, ScatterRay};
use crate::material::{AttenuatedColor, HitRecord};
use crate::pdf::{mix_generate, mix_value, ContainerPdf};
use crate::renderer::shader::Shaders::{
    AlbedoShaderType, NormalShaderType, PathTracingShaderType, SimpleShaderType,
};
use crate::renderer::Renderer;
use enum_dispatch::enum_dispatch;
use std::default::Default;

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

    /// Does the shader need a hittable with a light emitting  material
    fn needs_light(&self) -> bool;
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
        PathTracingShaderType(PathTracingShader { max_depth })
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
        let attenuated_color = rec.material.emitted(rec, total_ray_length);
        let scatter_res = rec.material.scatter(ray, rec);

        match scatter_res {
            None => attenuated_color,
            Some(scatter_record) => match scatter_record.scatter_type {
                ScatterRay(scatter_ray) => {
                    let ray_color_res =
                        renderer.ray_color(&scatter_ray, depth + 1, total_ray_length);
                    AttenuatedColor {
                        color: scatter_record.color * ray_color_res.pixel_color.color,
                        attenuation_factor: ray_color_res.pixel_color.attenuation_factor,
                        accumulated_ray_length: ray_color_res.pixel_color.accumulated_ray_length,
                    }
                }
                ScatterPdf(pdf) => {
                    let light_pdf = ContainerPdf::new(&renderer.lights, rec.hit_point);

                    let pdf_direction = mix_generate(&light_pdf, &pdf);
                    let scattered = Ray::new(rec.hit_point, pdf_direction);
                    let pdf_val = mix_value(&light_pdf, &pdf, scattered.direction);
                    let scattering_pdf = rec.material.scattering_pdf(rec, &scattered);
                    let ray_color_res = renderer.ray_color(&scattered, depth + 1, total_ray_length);
                    let scatter_color =
                        scatter_record.color * scattering_pdf * ray_color_res.pixel_color.color
                            / pdf_val;

                    AttenuatedColor {
                        color: filter_invalid_color_values(attenuated_color.color + scatter_color),
                        attenuation_factor: ray_color_res.pixel_color.attenuation_factor,
                        accumulated_ray_length: ray_color_res.pixel_color.accumulated_ray_length,
                    }
                }
            },
        }
    }

    fn needs_light(&self) -> bool {
        true
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
        AlbedoShaderType(AlbedoShader {})
    }
}

impl Shader for AlbedoShader {
    /// Calculates the color only attenuation color
    fn shade(&self, _: &Renderer, rec: &HitRecord, ray: &Ray, _: u32, _: f64) -> AttenuatedColor {
        AttenuatedColor {
            color: match rec.material.scatter(ray, rec) {
                None => rec.material.emitted(rec, 0.).color,
                Some(scatter_record) => scatter_record.color,
            },
            ..AttenuatedColor::default()
        }
    }

    fn needs_light(&self) -> bool {
        false
    }
}

#[derive(Clone)]
/// Outputs the normals of the ray hit point
pub struct NormalShader {}

impl NormalShader {
    #![allow(clippy::new_ret_no_self)]
    /// Create a new normal shader
    pub fn new() -> Shaders {
        NormalShaderType(NormalShader {})
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

    fn needs_light(&self) -> bool {
        false
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
        SimpleShaderType(SimpleShader {
            light_dir: Vec3::new(1., 1., -1.),
        })
    }
}

impl Shader for SimpleShader {
    /// Calculates the color only using normal and attenuation color
    fn shade(&self, _: &Renderer, rec: &HitRecord, ray: &Ray, _: u32, _: f64) -> AttenuatedColor {
        AttenuatedColor {
            color: match rec.material.scatter(ray, rec) {
                None => rec.material.emitted(rec, 0.).color,
                Some(scatter_record) => {
                    // Get a factor to multiply attenuation color, range between .25 -> 1.25
                    // To get some decent flat shading
                    let normal_factor = rec.normal.dot(self.light_dir) * 0.5 + 0.75;

                    scatter_record.color * normal_factor
                }
            },
            ..AttenuatedColor::default()
        }
    }

    fn needs_light(&self) -> bool {
        false
    }
}
