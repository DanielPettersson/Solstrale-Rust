//! Package hittable provides objects that are hittable by rays shot by the ray tracer.
//! Some of these hittable objects are containers for other objects
//! Some others are used to translate or rotate other objects

pub mod hittable_list;
pub mod hittable_pdf;
pub mod sphere;

use crate::geo::aabb::Aabb;
use crate::geo::ray::Ray;
use crate::geo::vec3::Vec3;
use crate::material::HitRecord;
use crate::util::interval::Interval;

/// The common trait for all objects in the ray tracing scene
/// that can be hit by rays
pub trait Hittable {
    fn pdf_value(&self, _origin: Vec3, _direction: Vec3) -> f64 {
        panic!("Should not be used for materials that can not be lights")
    }
    fn random_direction(&self, _origin: Vec3) -> Vec3 {
        panic!("Should not be used for materials that can not be lights")
    }
    fn hit(&self, r: &Ray, ray_length: &Interval) -> Option<HitRecord>;
    fn bounding_box(&self) -> &Aabb;
    fn is_light(&self) -> bool;
}
