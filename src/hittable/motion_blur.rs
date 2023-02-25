use crate::geo::aabb::Aabb;
use crate::geo::ray::Ray;
use crate::geo::vec3::Vec3;
use crate::hittable::Hittables::MotionBlurType;
use crate::hittable::{Hittable, Hittables};
use crate::material::HitRecord;
use crate::util::interval::Interval;

#[derive(Clone, Debug)]
pub struct MotionBlur {
    blurred_hittable: Box<Hittables>,
    blur_direction: Vec3,
    b_box: Aabb,
}

impl MotionBlur {
    /// Creates a new hittable object that adds linear interpolated translation to
    /// its hittable based on the time of the ray. This gives the appearance of the object moving.
    pub fn new(blurred_hittable: Hittables, blur_direction: Vec3) -> Hittables {
        let b_box1 = blurred_hittable.bounding_box();
        let b_box2 = b_box1 + blur_direction;
        let b_box = Aabb::combine_aabbs(b_box1, &b_box2);

        MotionBlurType(MotionBlur {
            blurred_hittable: Box::new(blurred_hittable),
            blur_direction,
            b_box,
        })
    }
}

impl Hittable for MotionBlur {
    fn hit(&self, r: &Ray, ray_length: &Interval) -> Option<HitRecord> {
        let offset = self.blur_direction * r.time;
        let offset_ray = Ray::new(r.origin - offset, r.direction, r.time);

        self.blurred_hittable
            .hit(&offset_ray, ray_length)
            .map(|rec| HitRecord {
                hit_point: rec.hit_point + offset,
                normal: rec.normal,
                material: rec.material,
                ray_length: rec.ray_length,
                u: rec.u,
                v: rec.v,
                front_face: rec.front_face,
            })
    }

    fn bounding_box(&self) -> &Aabb {
        &self.b_box
    }

    fn is_light(&self) -> bool {
        self.blurred_hittable.is_light()
    }
}
