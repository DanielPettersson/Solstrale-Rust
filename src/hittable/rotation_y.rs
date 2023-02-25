use crate::geo::aabb::Aabb;
use crate::geo::ray::Ray;
use crate::geo::vec3::Vec3;
use crate::hittable::Hittables::RotationYType;
use crate::hittable::{Hittable, Hittables};
use crate::material::HitRecord;
use crate::util::degrees_to_radians;
use crate::util::interval::Interval;

#[derive(Clone, Debug)]
pub struct RotationY {
    object: Box<Hittables>,
    sin_theta: f64,
    cos_theta: f64,
    b_box: Aabb,
}

impl RotationY {
    /// Creates a hittable object that rotates the given hittable
    /// around the Y axis with angle in degrees
    pub fn new(object: Hittables, angle: f64) -> Hittables {
        let radians = degrees_to_radians(angle);
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let object_b_box: &Aabb = object.bounding_box();

        let mut min = Vec3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
        let mut max = Vec3::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64 * object_b_box.x.max + (1. - i as f64) * object_b_box.x.min;
                    let y = j as f64 * object_b_box.y.max + (1. - j as f64) * object_b_box.y.min;
                    let z = k as f64 * object_b_box.z.max + (1. - k as f64) * object_b_box.z.min;

                    let new_x = cos_theta * x + sin_theta * z;
                    let new_z = -sin_theta * x + cos_theta * z;

                    let tester = Vec3::new(new_x, y, new_z);

                    min.x = min.x.min(tester.x);
                    min.y = min.y.min(tester.y);
                    min.z = min.z.min(tester.z);

                    max.x = max.x.max(tester.x);
                    max.y = max.y.max(tester.y);
                    max.z = max.z.max(tester.z);
                }
            }
        }

        RotationYType(RotationY {
            object: Box::new(object),
            sin_theta,
            cos_theta,
            b_box: Aabb::new_from_2_points(min, max),
        })
    }
}

impl Hittable for RotationY {
    fn pdf_value(&self, origin: Vec3, direction: Vec3) -> f64 {
        self.object.pdf_value(origin, direction)
    }

    fn random_direction(&self, origin: Vec3) -> Vec3 {
        self.object.random_direction(origin)
    }

    fn hit(&self, r: &Ray, ray_length: &Interval) -> Option<HitRecord> {
        let mut origin = r.origin;
        let mut direction = r.direction;

        origin.x = self.cos_theta * r.origin.x - self.sin_theta * r.origin.z;
        origin.z = self.sin_theta * r.origin.x + self.cos_theta * r.origin.z;

        direction.x = self.cos_theta * r.direction.x - self.sin_theta * r.direction.z;
        direction.z = self.sin_theta * r.direction.x + self.cos_theta * r.direction.z;

        let rotated_r = Ray::new(origin, direction, r.time);

        match self.object.hit(&rotated_r, ray_length) {
            None => None,
            Some(rec) => {
                let mut hit_point = rec.hit_point;
                hit_point.x = self.cos_theta * rec.hit_point.x + self.sin_theta * rec.hit_point.z;
                hit_point.z = -self.sin_theta * rec.hit_point.x + self.cos_theta * rec.hit_point.z;

                let mut normal = rec.normal;
                normal.x = self.cos_theta * rec.normal.x + self.sin_theta * rec.normal.z;
                normal.z = -self.sin_theta * rec.normal.x + self.cos_theta * rec.normal.z;

                Some(HitRecord {
                    hit_point,
                    normal,
                    material: rec.material,
                    ray_length: rec.ray_length,
                    u: rec.u,
                    v: rec.v,
                    front_face: rec.front_face,
                })
            }
        }
    }

    fn bounding_box(&self) -> &Aabb {
        &self.b_box
    }

    fn is_light(&self) -> bool {
        self.object.is_light()
    }
}
