use std::f64::consts::PI;

use enum_dispatch::enum_dispatch;

use crate::geo::ray::Ray;
use crate::geo::vec3::{Vec3, ZERO_VECTOR};
use crate::material::texture::Textures;
use crate::material::texture::{SolidColor, Texture};
use crate::pdf::{CosinePdf, Pdfs};

pub mod texture;

/// A collection of all interesting properties from
/// when a ray hits a hittable object
pub struct HitRecord<'a> {
    pub hit_point: Vec3,
    pub normal: Vec3,
    pub material: &'a Materials,
    pub ray_length: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
}

/// A collection of attributes from the scattering of a ray with a material
pub struct ScatterRecord<'a> {
    pub attenuation: Vec3,
    pub pdf: Pdfs<'a>,
    pub skip_pdf_ray: Option<Ray>,
}

/// The traut for types that describe how
/// a ray behaves when hitting an object.
#[enum_dispatch]
pub trait Material {
    fn scattering_pdf(&self, _rec: &HitRecord, _scattered: &Ray) -> f64 {
        0.
    }
    fn emitted(&self, _rec: &HitRecord) -> Vec3 {
        ZERO_VECTOR
    }
    fn is_light(&self) -> bool {
        false
    }
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<ScatterRecord>;
}

#[enum_dispatch(Material)]
pub enum Materials {
    Lambertian(Lambertian),
    DiffuseLight(DiffuseLight),
}

impl Clone for Materials {
    fn clone(&self) -> Self {
        match self {
            Materials::Lambertian(m) => Materials::Lambertian(m.clone()),
            Materials::DiffuseLight(m) => Materials::DiffuseLight(m.clone()),
        }
    }
}

/// A typical matte material
pub struct Lambertian {
    tex: Textures,
}

impl Lambertian {
    pub fn new(tex: Textures) -> Materials {
        Materials::Lambertian(Lambertian { tex })
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
        let attenuation = self.tex.color(&rec);
        let pdf = CosinePdf::new(rec.normal);

        return Some(ScatterRecord {
            attenuation,
            pdf,
            skip_pdf_ray: None,
        });
    }
}

impl Clone for Lambertian {
    fn clone(&self) -> Self {
        Lambertian {
            tex: self.tex.clone(),
        }
    }
}

/// A material used for emitting light
pub struct DiffuseLight {
    tex: Textures,
}

impl DiffuseLight {
    /// Creates a new diffuse light material
    pub fn new(r: f64, g: f64, b: f64) -> Materials {
        Materials::DiffuseLight(DiffuseLight {
            tex: SolidColor::new(r, g, b),
        })
    }
}

impl Material for DiffuseLight {
    fn emitted(&self, rec: &HitRecord) -> Vec3 {
        if !rec.front_face {
            return ZERO_VECTOR;
        }
        return self.tex.color(rec);
    }

    fn is_light(&self) -> bool {
        true
    }

    fn scatter(&self, _: &Ray, _: &HitRecord) -> Option<ScatterRecord> {
        None
    }
}

impl Clone for DiffuseLight {
    fn clone(&self) -> Self {
        DiffuseLight {
            tex: self.tex.clone(),
        }
    }
}
