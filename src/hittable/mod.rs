//! Objects that are hittable by rays shot by the ray tracer.
//! Some of these hittable objects are containers for other objects
//! Some others are used to translate or rotate other objects

mod bvh;
mod constant_medium;
mod hittable_list;
mod obj_model;
mod quad;
mod rotation_y;
mod sphere;
mod translation;
mod triangle;

use crate::geo::vec3::Vec3;
use crate::geo::Aabb;
use crate::geo::Ray;
pub use crate::hittable::bvh::Bvh;
pub use crate::hittable::constant_medium::ConstantMedium;
pub use crate::hittable::hittable_list::HittableList;
pub use crate::hittable::obj_model::load_obj_model;
pub use crate::hittable::obj_model::load_obj_model_with_default_material;
pub use crate::hittable::quad::Quad;
pub use crate::hittable::rotation_y::RotationY;
pub use crate::hittable::sphere::Sphere;
pub use crate::hittable::translation::Translation;
pub use crate::hittable::triangle::Triangle;
use crate::hittable::Hittables::{
    BvhType, ConstantMediumType, HittableListType, QuadType, RotationYType, SphereType,
    TranslationType, TriangleType,
};
use crate::material::HitRecord;
use crate::util::interval::Interval;
use enum_dispatch::enum_dispatch;
use std::slice::Iter;

/// The common trait for all objects in the ray tracing scene
/// that can be hit by rays
#[enum_dispatch]
pub trait Hittable {
    /// Return the pdf value for the hittable given the origin and direction of the ray that hits
    fn pdf_value(&self, _origin: Vec3, _direction: Vec3) -> f64 {
        panic!("Should not be used for materials that can not be lights")
    }

    /// Generate a random direction from the given point on the hittable
    fn random_direction(&self, _origin: Vec3) -> Vec3 {
        panic!("Should not be used for materials that can not be lights")
    }

    /// Check if the given ray hits the hittable within the interval
    fn hit(&self, r: &Ray, ray_length: &Interval) -> Option<HitRecord>;

    /// Create a bounding box that contains the hittable
    fn bounding_box(&self) -> &Aabb;

    /// Is the hittable a light?
    fn is_light(&self) -> bool;

    /// Return the children of the hittable, if any
    fn children(&self) -> Option<Iter<Hittables>> {
        None
    }

    /// Add a new child to the hittable
    fn add(&mut self, _hittable: Hittables) {
        panic!("Can only add child to HittableList")
    }
}

#[enum_dispatch(Hittable)]
#[derive(Debug)]
/// Enum of the available hittable types
pub enum Hittables {
    /// [`Hittable`] of the type [`HittableList`]
    HittableListType(HittableList),
    /// [`Hittable`] of the type [`Sphere`]
    SphereType(Sphere),
    /// [`Hittable`] of the type [`ConstantMedium`]
    ConstantMediumType(ConstantMedium),
    /// [`Hittable`] of the type [`Quad`]
    QuadType(Quad),
    /// [`Hittable`] of the type [`RotationY`]
    RotationYType(RotationY),
    /// [`Hittable`] of the type [`Translation`]
    TranslationType(Translation),
    /// [`Hittable`] of the type [`Triangle`]
    TriangleType(Triangle),
    /// [`Hittable`] of the type [`Bvh`]
    BvhType(Bvh),
}

impl Clone for Hittables {
    fn clone(&self) -> Self {
        match self {
            HittableListType(_) => panic!("Should not clone HittableList"),
            SphereType(h) => SphereType(h.clone()),
            ConstantMediumType(h) => ConstantMediumType(h.clone()),
            QuadType(h) => QuadType(h.clone()),
            RotationYType(h) => RotationYType(h.clone()),
            TranslationType(h) => TranslationType(h.clone()),
            TriangleType(h) => TriangleType(h.clone()),
            BvhType(h) => BvhType(h.clone()),
        }
    }
}
