use crate::geo::aabb::Aabb;
use crate::geo::ray::Ray;
use crate::geo::vec3::Vec3;
use crate::hittable::{Hittable, Hittables};
use crate::material::HitRecord;
use crate::random::random_element_index;
use crate::util::interval::Interval;
use std::slice::Iter;

/// A special type of hittable that is a container
/// for a list of other hittable objects. Used to be able to have many
/// objects in a scene
pub struct HittableList {
    pub list: Vec<Hittables>,
    b_box: Aabb,
}

impl HittableList {
    /// Creates new empty HittableList
    pub fn new() -> HittableList {
        HittableList {
            list: Vec::new(),
            b_box: Aabb::new_with_empty_intervals(),
        }
    }

    /// Adds a new hittable object to this HittableList
    pub fn add(&mut self, h: Hittables) {
        self.b_box = Aabb::combine_aabbs(&self.b_box, h.bounding_box());
        self.list.push(h);
    }
}

impl Hittable for HittableList {
    fn pdf_value(&self, origin: Vec3, direction: Vec3) -> f64 {
        let sum: f64 = self
            .list
            .iter()
            .map(|i| i.pdf_value(origin, direction))
            .sum();
        sum / self.list.len() as f64
    }

    fn random_direction(&self, origin: Vec3) -> Vec3 {
        let idx = random_element_index(&self.list);
        self.list[idx].random_direction(origin)
    }

    fn hit(&self, r: &Ray, ray_length: &Interval) -> Option<HitRecord> {
        let mut closest_hit_record: Option<HitRecord> = None;
        let mut closest_interval = Interval::new(ray_length.min, ray_length.max);

        for h in &self.list {
            let hit_record_opt = h.hit(r, &closest_interval);
            if let Some(hit_record) = hit_record_opt {
                closest_interval = Interval::new(ray_length.min, hit_record.ray_length);
                closest_hit_record = Some(hit_record);
            }
        }
        closest_hit_record
    }

    fn bounding_box(&self) -> &Aabb {
        &self.b_box
    }

    fn is_light(&self) -> bool {
        false
    }

    fn children(&self) -> Option<Iter<Hittables>> {
        Some(self.list.iter())
    }
}
