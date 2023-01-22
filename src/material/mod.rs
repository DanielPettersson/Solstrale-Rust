pub mod texture;

use crate::geo::ray::Ray;
use crate::geo::vec3::{Vec3, ZERO_VECTOR};
use crate::material::texture::Texture;
use crate::pdf::{CosinePdf, Pdf};
use std::f64::consts::PI;

/// A collection of all interesting properties from
/// when a ray hits a hittable object
pub struct HitRecord<'a> {
    pub hit_point: Vec3,
    pub normal: Vec3,
    pub material: &'a dyn Material,
    pub ray_length: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
}

/// A collection of attributes from the scattering of a ray with a material
pub struct ScatterRecord {
    pub attenuation: Vec3,
    pub pdf: Box<dyn Pdf>,
    pub skip_pdf_ray: Option<Ray>,
}

/// The traut for types that describe how
/// a ray behaves when hitting an object.
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

/// A typical matte material
pub struct Lambertian {
    tex: Box<dyn Texture>,
}

impl Lambertian {
    pub fn new(tex: Box<dyn Texture>) -> Box<dyn Material> {
        Box::new(Lambertian { tex })
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
