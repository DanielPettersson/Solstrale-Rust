use crate::geo::aabb::Aabb;
use crate::geo::ray::Ray;
use crate::geo::vec3::{random_unit_vector, Vec3};
use crate::geo::Uv;
use crate::hittable::Hittables::ConstantMediumType;
use crate::hittable::{Hittable, Hittables};
use crate::material::texture::SolidColor;
use crate::material::Materials;
use crate::material::{HitRecord, Isotropic};
use crate::random::random_normal_float;
use crate::util::interval::{Interval, UNIVERSE_INTERVAL};

#[derive(Clone, Debug)]
pub struct ConstantMedium {
    boundary: Box<Hittables>,
    negative_inverse_density: f64,
    phase_function: Materials,
}

impl ConstantMedium {
    /// Creates a fog type hittable object where rays not only scatter
    /// at the edge of the object, but at random points inside the object
    /// The material of the boundary hittable is ignored
    pub fn create(boundary: Hittables, density: f64, color: Vec3) -> Hittables {
        ConstantMediumType(ConstantMedium {
            boundary: Box::new(boundary),
            negative_inverse_density: -1. / density,
            phase_function: Isotropic::create(SolidColor::from_vec3(color)),
        })
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, r: &Ray, ray_length: &Interval) -> Option<HitRecord> {
        match self.boundary.hit(r, &UNIVERSE_INTERVAL) {
            None => None,
            Some(rec1) => {
                let interval = Interval::new(rec1.ray_length + 0.0001, f64::INFINITY);
                match self.boundary.hit(r, &interval) {
                    None => None,
                    Some(rec2) => {
                        let mut rec1_ray_length = rec1.ray_length.max(ray_length.min);
                        let rec2_ray_length = rec2.ray_length.min(ray_length.max);

                        if rec1_ray_length >= rec2_ray_length {
                            return None;
                        }

                        rec1_ray_length = rec1_ray_length.max(0.);
                        let r_length = r.direction.length();
                        let distance_inside_boundary =
                            (rec2_ray_length - rec1_ray_length) * r_length;
                        let hit_distance =
                            self.negative_inverse_density * random_normal_float().ln();

                        if hit_distance > distance_inside_boundary {
                            return None;
                        }

                        let t = rec1_ray_length + hit_distance / r_length;

                        Some(HitRecord {
                            hit_point: r.at(t),
                            normal: random_unit_vector(),
                            material: &self.phase_function,
                            ray_length: t,
                            uv: Uv::new(0.0, 0.0),
                            front_face: false,
                        })
                    }
                }
            }
        }
    }

    fn bounding_box(&self) -> &Aabb {
        self.boundary.bounding_box()
    }

    fn is_light(&self) -> bool {
        false
    }
}
