use crate::geo::ray::Ray;
use crate::geo::vec3::{Vec3, ZERO_VECTOR};
use crate::pdf::Pdf;

/// A collection of all interesting properties from
/// when a ray hits a hittable object
pub struct HitRecord {
    pub hit_point: Vec3,
    pub normal: Vec3,
    pub material: Box<dyn Material>,
    pub ray_length: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
}

/// A collection of attributes from the scattering of a ray with a material
struct ScatterRecord {
    attenuation: Vec3,
    pdf: Box<dyn Pdf>,
    skip_pdf: bool,
    skip_pdf_ray: Ray,
}

/// The traut for types that describe how
/// a ray behaves when hitting an object.
pub trait Material {
    fn scattering_pdf(&self, _: HitRecord, _: Ray) -> f64 {
        0.
    }
    fn emitted(&self, _: HitRecord) -> Vec3 {
        ZERO_VECTOR
    }
    fn is_light(&self) -> bool {
        false
    }
    fn scatter(&self, ray_in: Ray, rec: HitRecord) -> Option<ScatterRecord>;
}
