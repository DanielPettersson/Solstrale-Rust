//! Materials to be applied to hittable objects

use std::f64::consts::PI;

use enum_dispatch::enum_dispatch;

use crate::geo::{Onb, Ray};
use crate::geo::Uv;
use crate::geo::vec3::{random_in_unit_sphere, Vec3, ZERO_VECTOR};
use crate::hittable::Hittables;
use crate::material::Materials::{BlendType, DielectricType, DiffuseLightType, IsotropicType, LambertianType, MetalType};
use crate::material::texture::{SolidColor, Texture};
use crate::material::texture::Textures;
use crate::pdf::{ContainerPdf, CosinePdf, mix_generate, mix_value, SpherePdf};
use crate::random::random_normal_float;

pub mod texture;

/// A collection of all interesting properties from
/// when a ray hits a hittable object
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct RayHit<'a> {
    /// Hit point for the ray on a hittable
    pub hit_point: Vec3,
    /// Normal vector of the hittable at the hit point
    pub normal: Vec3,
    /// Material of the hittable that the ray hit
    pub material: &'a Materials,
    /// The length of the ray from origin to hit point
    pub ray_length: f64,
    /// Texture coordinate at the hit point
    pub uv: Uv,
    /// Whether the hit point is inside or outside the hittable
    pub front_face: bool,
}

impl<'a> RayHit<'a> {
    /// Creates a new HitRecord
    pub fn new(
        hit_point: Vec3,
        normal: Vec3,
        material: &'a Materials,
        ray_length: f64,
        uv: Uv,
        front_face: bool,
    ) -> RayHit<'a> {
        RayHit {
            hit_point,
            normal: material.get_transformed_normal(normal, uv),
            material,
            ray_length,
            uv,
            front_face,
        }
    }
}

/// Scattering of a ray against a pdf material
pub struct ScatterPdf {
    /// The attenuation color from the ray hit
    pub color: Vec3,
    /// The scattered ray
    pub ray: Ray,
    /// The probability factor for the scattered ray
    pub probability: f64
}

/// Scattering of a ray against a basic material
pub struct ScatterBasic {
    /// The attenuation color from the ray hit
    pub color: Vec3,
    /// The scattered ray
    pub ray: Ray,
}

/// Scattering of a ray against a light emitting material
pub struct ScatterEmission {
    /// The emitted color from the ray hit
    pub color: Vec3,
    /// The attenuation factor of the light source
    pub attenuation_factor: Option<f64>,
}

/// An enum of scatter types
pub enum RayScatter {
    /// Scatters using [`Pdfs`] to determine the ray
    ScatterPdf(ScatterPdf),
    /// A basic scattering without the use of [`Pdfs`]
    ScatterBasic(ScatterBasic),
    /// No scattering of light, only emission.
    ScatterEmission(ScatterEmission),
}

/// The trait for types that describe how
/// a ray behaves when hitting an object.
#[enum_dispatch]
pub trait Material {
    /// Is the material emitting light
    fn is_light(&self) -> bool {
        false
    }

    /// Calculate scattering of the ray
    fn scatter(&self, _ray: &Ray, _rec: &RayHit, _lights: &[Hittables]) -> RayScatter;

    /// Get normal transformed by the material, implementations typically uses a normal texture map
    fn get_transformed_normal(&self, normal: Vec3, _uv: Uv) -> Vec3 {
        normal
    }
}

#[derive(Default)]
/// An color along with along with attenuation information
pub struct AttenuatedColor {
    /// Color value before attenuation
    pub color: Vec3,
    /// Factor for calculating amount of attenuation
    pub attenuation_factor: Option<f64>,
    /// Distance the light has travelled
    pub accumulated_ray_length: f64,
}

impl AttenuatedColor {
    /// Calculate the actual color based on the original color
    /// and the attenuation information
    pub fn get_attenuated_color(&self) -> Vec3 {
        self.attenuation_factor.map_or(self.color, |af| {
            self.color * 1. / (1. + af * self.accumulated_ray_length)
        })
    }
}

#[enum_dispatch(Material)]
#[derive(Debug)]
/// An enum of available materials
pub enum Materials {
    /// [`Material`] of type [`Lambertian`]
    LambertianType(Lambertian),
    /// [`Material`] of type [`Metal`]
    MetalType(Metal),
    /// [`Material`] of type [`Dielectric`]
    DielectricType(Dielectric),
    /// [`Material`] of type [`DiffuseLight`]
    DiffuseLightType(DiffuseLight),
    /// [`Material`] of type [`Isotropic`]
    IsotropicType(Isotropic),
    /// [`Material`] of type [`Blend`]
    BlendType(Blend),
}

impl Clone for Materials {
    fn clone(&self) -> Self {
        match self {
            LambertianType(m) => LambertianType(m.clone()),
            MetalType(m) => MetalType(m.clone()),
            DielectricType(m) => DielectricType(m.clone()),
            DiffuseLightType(m) => DiffuseLightType(m.clone()),
            IsotropicType(m) => IsotropicType(m.clone()),
            BlendType(m) => BlendType(m.clone())
        }
    }
}

/// A typical matte material
#[derive(Clone, Debug)]
pub struct Lambertian {
    albedo: Textures,
    normal: Option<Textures>,
}

impl Lambertian {
    #![allow(clippy::new_ret_no_self)]
    /// Create a new lambertian material
    pub fn new(albedo: Textures, normal: Option<Textures>) -> Materials {
        Materials::from(Lambertian { albedo, normal })
    }

    fn scattering_pdf_value(normal: Vec3, scatter_direction: Vec3) -> f64 {
        let cos_theta = normal.dot(scatter_direction);
        if cos_theta < 0. {
            0.
        } else {
            cos_theta / PI
        }
    }
}

impl Material for Lambertian {

    fn scatter(&self, _: &Ray, rec: &RayHit, lights: &[Hittables]) -> RayScatter {
        let color = self.albedo.color(rec.uv);
        let pdf = CosinePdf::new(rec.normal);

        let light_pdf = ContainerPdf::new(lights, rec.hit_point);

        let pdf_direction = mix_generate(&light_pdf, &pdf);
        let scattered = Ray::new(rec.hit_point, pdf_direction);
        let light_pdf_value = mix_value(&light_pdf, &pdf, scattered.direction);
        let scattering_pdf_value = Lambertian::scattering_pdf_value(rec.normal, scattered.direction.unit());

        RayScatter::ScatterPdf(ScatterPdf {
            color,
            ray: scattered,
            probability: scattering_pdf_value / light_pdf_value,
        })
    }

    fn get_transformed_normal(&self, normal: Vec3, uv: Uv) -> Vec3 {
        self.normal
            .as_ref()
            .map_or(normal, |n| transform_normal_by_map(n, normal, uv))
    }
}

/// Metal is a material that is reflective
#[derive(Clone, Debug)]
pub struct Metal {
    albedo: Textures,
    normal: Option<Textures>,
    fuzz: f64,
}

impl Metal {
    #![allow(clippy::new_ret_no_self)]
    /// Creates a metal material
    pub fn new(albedo: Textures, normal: Option<Textures>, fuzz: f64) -> Materials {
        Materials::from(Metal {
            albedo,
            normal,
            fuzz,
        })
    }
}

impl Material for Metal {
    /// Returns a reflected scattered ray for the metal material
    /// The Fuzz property of the metal defines the randomness applied to the reflection
    fn scatter(&self, ray: &Ray, rec: &RayHit, _lights: &[Hittables]) -> RayScatter {
        let reflected = ray.direction.unit().reflect(rec.normal);

        RayScatter::ScatterBasic(ScatterBasic {
            color: self.albedo.color(rec.uv),
            ray: Ray::new(
                rec.hit_point,
                reflected + random_in_unit_sphere() * self.fuzz,
            ),
        })
    }

    fn get_transformed_normal(&self, normal: Vec3, uv: Uv) -> Vec3 {
        self.normal
            .as_ref()
            .map_or(normal, |n| transform_normal_by_map(n, normal, uv))
    }
}

/// A glass type material with an index of refraction
#[derive(Clone, Debug)]
pub struct Dielectric {
    albedo: Textures,
    normal: Option<Textures>,
    index_of_refraction: f64,
}

impl Dielectric {
    #![allow(clippy::new_ret_no_self)]
    /// Creates a new dielectric material
    pub fn new(albedo: Textures, normal: Option<Textures>, index_of_refraction: f64) -> Materials {
        Materials::from(Dielectric {
            albedo,
            normal,
            index_of_refraction,
        })
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, rec: &RayHit, _lights: &[Hittables]) -> RayScatter {
        let refraction_ratio = if rec.front_face {
            1. / self.index_of_refraction
        } else {
            self.index_of_refraction
        };

        let unit_direction = ray.direction.unit();
        let cos_theta = unit_direction.neg().dot(rec.normal).min(1.);
        let sin_theta = (1. - cos_theta * cos_theta).sqrt();
        let cannot_refract = refraction_ratio * sin_theta > 1.;

        let direction =
            if cannot_refract || reflectance(cos_theta, refraction_ratio) > random_normal_float() {
                unit_direction.reflect(rec.normal)
            } else {
                unit_direction.refract(rec.normal, refraction_ratio)
            };

        RayScatter::ScatterBasic(ScatterBasic {
            color: self.albedo.color(rec.uv),
            ray: Ray::new(rec.hit_point, direction),
        })
    }

    fn get_transformed_normal(&self, normal: Vec3, uv: Uv) -> Vec3 {
        self.normal
            .as_ref()
            .map_or(normal, |n| transform_normal_by_map(n, normal, uv))
    }
}

/// Calculate reflectance using Schlick's approximation
fn reflectance(cosine: f64, index_of_refraction: f64) -> f64 {
    let mut r0 = (1. - index_of_refraction) / (1. + index_of_refraction);
    r0 = r0 * r0;
    r0 + (1. - r0) * (1. - cosine).powi(5)
}

/// A material used for emitting light
#[derive(Clone, Debug)]
pub struct DiffuseLight {
    tex: Textures,
    attenuation_factor: Option<f64>,
}

impl DiffuseLight {
    #![allow(clippy::new_ret_no_self)]

    /// Creates a new diffuse light material
    ///
    /// # Arguments
    /// * `r` - The red component of the light
    /// * `g` - The green component of the light
    /// * `b` - The blue component of the light
    /// * `attenuation_half_length` - The distance at which the light is attenuated to half its strength
    pub fn new(r: f64, g: f64, b: f64, attenuation_half_length: Option<f64>) -> Materials {
        Materials::from(DiffuseLight {
            tex: SolidColor::new(r, g, b),
            attenuation_factor: attenuation_half_length.map(|a| 1. / a),
        })
    }

    /// Creates a new diffuse light material
    ///
    /// # Arguments
    /// * `v` - The [`Vec3`] representation of the light color
    pub fn new_from_vec3(v: Vec3) -> Materials {
        DiffuseLightType(DiffuseLight {
            tex: SolidColor::new_from_vec3(v),
            attenuation_factor: None,
        })
    }
}

impl Material for DiffuseLight {
    fn is_light(&self) -> bool {
        true
    }

    fn scatter(&self, _ray: &Ray, rec: &RayHit, _lights: &[Hittables]) -> RayScatter {
        RayScatter::ScatterEmission(ScatterEmission {
            color: if rec.front_face {
                self.tex.color(rec.uv)
            } else {
                ZERO_VECTOR
            },
            attenuation_factor: self.attenuation_factor,
        })
    }
}

/// Isotropic is a fog type material
/// Should not be used directly, but is used internally by ConstantMedium hittable
#[derive(Clone, Debug)]
pub struct Isotropic {
    tex: Textures,
}

impl Isotropic {
    #![allow(clippy::new_ret_no_self)]
    /// Create a new isotropic material
    pub(crate) fn new(tex: Textures) -> Materials {
        Materials::from(Isotropic { tex })
    }
}

fn transform_normal_by_map(normal_map: &Textures, normal: Vec3, uv: Uv) -> Vec3 {
    let map_normal = (normal_map.color(uv) - 0.5) * 2.;
    Onb::new(map_normal).local(normal)
}

const SPHERE_PDF_VALUE: f64 = 1. / (4. * PI);

impl Material for Isotropic {

    /// Returns a randomly scattered ray in any direction
    fn scatter(&self, _: &Ray, rec: &RayHit, lights: &[Hittables]) -> RayScatter {
        let color = self.tex.color(rec.uv);

        let pdf = SpherePdf::new();
        let light_pdf = ContainerPdf::new(lights, rec.hit_point);
        let pdf_direction = mix_generate(&light_pdf, &pdf);
        let scattered = Ray::new(rec.hit_point, pdf_direction);
        let light_pdf_value = mix_value(&light_pdf, &pdf, scattered.direction);

        RayScatter::ScatterPdf(ScatterPdf {
            color,
            ray: scattered, 
            probability: SPHERE_PDF_VALUE / light_pdf_value,
        })
  }
}

/// A blend of two underlying materials
#[derive(Clone, Debug)]
pub struct Blend {
    material_1: Box<Materials>,
    material_2: Box<Materials>,
    blend_factor: f64,
}

impl Blend {
    #![allow(clippy::new_ret_no_self)]
    /// Create a new blend material from two underlying material and a blend factor [0..1]
    pub fn new(material_1: Materials, material_2: Materials, blend_factor: f64) -> Materials {
        Materials::from(Blend { material_1: Box::new(material_1), material_2: Box::new(material_2), blend_factor })
    }
}

impl Material for Blend {
    fn scatter(&self, ray: &Ray, rec: &RayHit, lights: &[Hittables]) -> RayScatter {
        if random_normal_float() > self.blend_factor {
            self.material_1.scatter(ray, rec, lights)
        } else {
            self.material_2.scatter(ray, rec, lights)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Sub;

    use crate::geo::Uv;
    use crate::geo::vec3::Vec3;
    use crate::material::texture::SolidColor;
    use crate::material::transform_normal_by_map;

    #[test]
    fn test_transform_normal_by_map() {
        let n = transform_normal_by_map(
            &SolidColor::new(1., 0.5, 0.5),
            Vec3::new(1., 0., 0.),
            Uv::default(),
        );

        assert!(Vec3::new(0., -1., 0.).sub(n).near_zero(), "n was {}", n);
    }
}
