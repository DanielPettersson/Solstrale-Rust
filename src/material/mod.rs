//! Materials to be applied to [`hittable::Hittable`] objects

use std::f64::consts::PI;

use enum_dispatch::enum_dispatch;

use crate::geo::{Onb, Ray};
use crate::geo::Uv;
use crate::geo::vec3::{random_in_unit_sphere, Vec3, ZERO_VECTOR};
use crate::material::Materials::{
    DielectricType, DiffuseLightType, IsotropicType, LambertianType, MetalType,
};
use crate::material::texture::{SolidColor, Texture};
use crate::material::texture::Textures;
use crate::pdf::{CosinePdf, Pdfs, SpherePdf};
use crate::random::random_normal_float;

pub mod texture;

/// A collection of all interesting properties from
/// when a ray hits a hittable object
#[derive(Clone, Debug)]
pub struct HitRecord<'a> {
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

/// A collection of attributes from the scattering of a ray with a material
pub struct ScatterRecord<'a> {
    /// The attenuation color from the ray hit
    pub attenuation: Vec3,
    /// The type of scattering to do for the ray, depends on the material
    pub scatter_type: ScatterType<'a>,
}

/// An enum of scatter types
pub enum ScatterType<'a> {
    ScatterPdf(Pdfs<'a>),
    ScatterRay(Ray),
}

/// The trait for types that describe how
/// a ray behaves when hitting an object.
#[enum_dispatch]
pub trait Material {
    /// Return the pdf to use for ray scattering
    fn scattering_pdf(&self, _rec: &HitRecord, _scattered: &Ray) -> f64 {
        0.
    }

    /// Color emitted from the material
    fn emitted(&self, _rec: &HitRecord) -> Vec3 {
        ZERO_VECTOR
    }

    /// Is the material emitting light
    fn is_light(&self) -> bool {
        false
    }

    /// Calculate scattering of the ray
    fn scatter(&self, _ray: &Ray, _rec: &HitRecord) -> Option<ScatterRecord> {
        None
    }

    /// Get normal transformed by the material, implementations typically uses a normal texture map
    fn get_transformed_normal(&self, rec: &HitRecord) -> Vec3 {
        rec.normal
    }
}

#[enum_dispatch(Material)]
#[derive(Debug)]
/// An enum of materials
pub enum Materials {
    LambertianType(Lambertian),
    MetalType(Metal),
    DielectricType(Dielectric),
    DiffuseLightType(DiffuseLight),
    IsotropicType(Isotropic),
}

impl Clone for Materials {
    fn clone(&self) -> Self {
        match self {
            LambertianType(m) => LambertianType(m.clone()),
            MetalType(m) => MetalType(m.clone()),
            DielectricType(m) => DielectricType(m.clone()),
            DiffuseLightType(m) => DiffuseLightType(m.clone()),
            IsotropicType(m) => IsotropicType(m.clone()),
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
        LambertianType(Lambertian { albedo, normal })
    }
}

impl Material for Lambertian {
    fn scattering_pdf(&self, rec: &HitRecord, scattered: &Ray) -> f64 {
        let cos_theta = rec.normal.dot(scattered.direction.unit());
        if cos_theta < 0. {
            0.
        } else {
            cos_theta / PI
        }
    }

    fn scatter(&self, _: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let attenuation = self.albedo.color(rec);
        let normal = self.get_transformed_normal(rec);
        let pdf = CosinePdf::new(normal);

        return Some(ScatterRecord {
            attenuation,
            scatter_type: ScatterType::ScatterPdf(pdf),
        });
    }

    fn get_transformed_normal(&self, rec: &HitRecord) -> Vec3 {
        self.normal.as_ref().map_or(
            rec.normal,
            |n| transform_normal_by_map(n, rec)
        )
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
        MetalType(Metal { albedo, normal, fuzz })
    }
}

impl Material for Metal {
    /// Returns a reflected scattered ray for the metal material
    /// The Fuzz property of the metal defines the randomness applied to the reflection
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let normal = self.get_transformed_normal(rec);
        let reflected = ray.direction.unit().reflect(normal);

        Some(ScatterRecord {
            attenuation: self.albedo.color(rec),
            scatter_type: ScatterType::ScatterRay(Ray::new(
                rec.hit_point,
                reflected + random_in_unit_sphere() * self.fuzz,
            )),
        })
    }

    fn get_transformed_normal(&self, rec: &HitRecord) -> Vec3 {
        self.normal.as_ref().map_or(
            rec.normal,
            |n| transform_normal_by_map(n, rec)
        )
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
        DielectricType(Dielectric {
            albedo,
            normal,
            index_of_refraction,
        })
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let refraction_ratio = if rec.front_face {
            1. / self.index_of_refraction
        } else {
            self.index_of_refraction
        };
        let normal = self.get_transformed_normal(rec);

        let unit_direction = ray.direction.unit();
        let cos_theta = unit_direction.neg().dot(normal).min(1.);
        let sin_theta = (1. - cos_theta * cos_theta).sqrt();
        let cannot_refract = refraction_ratio * sin_theta > 1.;

        let direction =
            if cannot_refract || reflectance(cos_theta, refraction_ratio) > random_normal_float() {
                unit_direction.reflect(normal)
            } else {
                unit_direction.refract(normal, refraction_ratio)
            };

        Some(ScatterRecord {
            attenuation: self.albedo.color(rec),
            scatter_type: ScatterType::ScatterRay(Ray::new(rec.hit_point, direction)),
        })
    }

    fn get_transformed_normal(&self, rec: &HitRecord) -> Vec3 {
        self.normal.as_ref().map_or(
            rec.normal,
            |n| transform_normal_by_map(n, rec)
        )
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
}

impl DiffuseLight {
    #![allow(clippy::new_ret_no_self)]
    /// Creates a new diffuse light material
    pub fn new(r: f64, g: f64, b: f64) -> Materials {
        DiffuseLightType(DiffuseLight {
            tex: SolidColor::new(r, g, b),
        })
    }

    /// Creates a new diffuse light material from a [´Vec3´] color
    pub fn new_from_vec3(v: Vec3) -> Materials {
        DiffuseLightType(DiffuseLight {
            tex: SolidColor::new_from_vec3(v),
        })
    }
}

impl Material for DiffuseLight {
    fn emitted(&self, rec: &HitRecord) -> Vec3 {
        if !rec.front_face {
            return ZERO_VECTOR;
        }
        self.tex.color(rec)
    }

    fn is_light(&self) -> bool {
        true
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
        IsotropicType(Isotropic { tex })
    }
}

fn transform_normal_by_map(normal_map: &Textures, rec: &HitRecord) -> Vec3 {
    let map_normal = (normal_map.color(rec) - 0.5) * 2.;
    Onb::new(rec.normal).local(map_normal)
}

const SPHERE_PDF_VALUE: f64 = 1. / (4. * PI);

impl Material for Isotropic {
    /// returns the pdf value for a given rays for the isotropic material
    fn scattering_pdf(&self, _: &HitRecord, _: &Ray) -> f64 {
        SPHERE_PDF_VALUE
    }

    /// Returns a randomly scattered ray in any direction
    fn scatter(&self, _: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let attenuation = self.tex.color(rec);

        let pdf = SpherePdf::new();

        Some(ScatterRecord {
            attenuation,
            scatter_type: ScatterType::ScatterPdf(pdf),
        })
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Sub;

    use crate::geo::Uv;
    use crate::geo::vec3::Vec3;
    use crate::material::{HitRecord, Lambertian, Materials, transform_normal_by_map};
    use crate::material::texture::SolidColor;

    #[test]
    fn test_transform_normal_by_map() {
        let n = transform_normal_by_map(
            &SolidColor::new(1., 0.5, 0.5),
            &hit_record(Vec3::new(1., 0., 0.), &dummy_material()),
        );

        assert!(Vec3::new(0., -1., 0.).sub(n).near_zero(), "n was {}", n);
    }

    fn dummy_material() -> Materials {
        Lambertian::new(SolidColor::new(1., 1., 1.), None)
    }

    fn hit_record(n: Vec3, m: &Materials) -> HitRecord {
        HitRecord {
            hit_point: Default::default(),
            normal: n,
            material: m,
            ray_length: 0.0,
            uv: Uv::new(0., 0.),
            front_face: false,
        }
    }

}
