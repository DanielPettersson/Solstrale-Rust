//! Package hittable provides objects that are hittable by rays shot by the ray tracer.
//! Some of these hittable objects are containers for other objects
//! Some others are used to translate or rotate other objects

pub mod constant_medium;
pub mod hittable_list;
pub mod motion_blur;
pub mod quad;
pub mod rotation_y;
pub mod sphere;
pub mod translation;
pub mod triangle;

use crate::geo::aabb::Aabb;
use crate::geo::ray::Ray;
use crate::geo::vec3::Vec3;
use crate::hittable::constant_medium::ConstantMedium;
use crate::hittable::hittable_list::HittableList;
use crate::hittable::motion_blur::MotionBlur;
use crate::hittable::quad::Quad;
use crate::hittable::rotation_y::RotationY;
use crate::hittable::sphere::Sphere;
use crate::hittable::translation::Translation;
use crate::hittable::triangle::Triangle;
use crate::material::HitRecord;
use crate::util::interval::Interval;
use enum_dispatch::enum_dispatch;
use std::slice::Iter;

/// The common trait for all objects in the ray tracing scene
/// that can be hit by rays
#[enum_dispatch]
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
    fn children(&self) -> Option<Iter<Hittables>> {
        None
    }
    fn add(&mut self, _hittable: Hittables) {
        panic!("Can only add child to HittableList")
    }
}

#[enum_dispatch(Hittable)]
pub enum Hittables {
    HittableList(HittableList),
    Sphere(Sphere),
    ConstantMedium(ConstantMedium),
    MotionBlur(MotionBlur),
    Quad(Quad),
    RotationY(RotationY),
    Translation(Translation),
    Triangle(Triangle),
}

impl Clone for Hittables {
    fn clone(&self) -> Self {
        match self {
            Hittables::HittableList(_) => panic!("Should not clone HittableList"),
            Hittables::Sphere(h) => Hittables::Sphere(h.clone()),
            Hittables::ConstantMedium(h) => Hittables::ConstantMedium(h.clone()),
            Hittables::MotionBlur(h) => Hittables::MotionBlur(h.clone()),
            Hittables::Quad(h) => Hittables::Quad(h.clone()),
            Hittables::RotationY(h) => Hittables::RotationY(h.clone()),
            Hittables::Translation(h) => Hittables::Translation(h.clone()),
            Hittables::Triangle(h) => Hittables::Triangle(h.clone()),
        }
    }
}
