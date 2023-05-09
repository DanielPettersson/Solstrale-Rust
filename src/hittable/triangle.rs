use crate::geo::vec3::{Vec3, ALMOST_ZERO};
use crate::geo::Aabb;
use crate::geo::Ray;
use crate::geo::Uv;
use crate::hittable::Hittables::TriangleType;
use crate::hittable::{Hittable, Hittables};
use crate::material::{HitRecord, Material, Materials};
use crate::random::random_normal_float;
use crate::util::interval::{Interval, RAY_INTERVAL};

/// A triangle shaped hittable object
#[derive(Clone, Debug)]
pub struct Triangle {
    v0: Vec3,
    v0v1: Vec3,
    v0v2: Vec3,
    uv0: Uv,
    uv1: Uv,
    uv2: Uv,
    normal: Vec3,
    mat: Materials,
    b_box: Aabb,
    area: f64,
    /// Center point of the triangle
    pub center: Vec3,
}

impl Triangle {
    #![allow(clippy::new_ret_no_self)]
    /// Creates a new triangle hittable object with no texture coordinates
    pub fn new(v0: Vec3, v1: Vec3, v2: Vec3, mat: Materials) -> Hittables {
        Triangle::new_with_tex_coords(
            v0,
            v1,
            v2,
            Uv { u: 0.0, v: 0.0 },
            Uv { u: 0.0, v: 0.0 },
            Uv { u: 0.0, v: 0.0 },
            mat,
        )
    }

    /// Creates a new triangle flat hittable object. A counter clockwise winding is expected
    pub fn new_with_tex_coords(
        v0: Vec3,
        v1: Vec3,
        v2: Vec3,
        uv0: Uv,
        uv1: Uv,
        uv2: Uv,
        mat: Materials,
    ) -> Hittables {
        let b_box = Aabb::new_from_3_points(v0, v1, v2).pad_if_needed();
        let v0v1 = v1 - v0;
        let v0v2 = v2 - v0;
        let n = v0v1.cross(v0v2);
        let normal = n.unit();
        let area = n.length() / 2.;
        let center = (v0 + v1 + v2) * 0.33333;

        TriangleType(Triangle {
            v0,
            v0v1,
            v0v2,
            uv0,
            uv1,
            uv2,
            normal,
            mat,
            b_box,
            area,
            center,
        })
    }

    /// returns the center point for the triangle on the given axis
    pub fn center(&self, axis: u8) -> f64 {
        self.center.axis(axis)
    }
}

impl Hittable for Triangle {
    fn pdf_value(&self, origin: Vec3, direction: Vec3) -> f64 {
        let ray = Ray::new(origin, direction, 0.);

        match self.hit(&ray, &RAY_INTERVAL) {
            None => 0.,
            Some(rec) => {
                let distance_squared = rec.ray_length * rec.ray_length * direction.length_squared();
                let cosine = (direction.dot(rec.normal) / direction.length()).abs();

                distance_squared / (cosine * self.area)
            }
        }
    }

    fn random_direction(&self, origin: Vec3) -> Vec3 {
        let p = self.v0 + self.v0v1 * random_normal_float() + self.v0v2 * random_normal_float();
        p - origin
    }

    fn hit(&self, r: &Ray, ray_length: &Interval) -> Option<HitRecord> {
        let p_vec = r.direction.cross(self.v0v2);
        let det = self.v0v1.dot(p_vec);

        // No hit if the ray is parallel to the plane
        if det.abs() < ALMOST_ZERO {
            return None;
        }

        let inv_det = 1. / det;
        let t_vec = r.origin - self.v0;
        let q_vec = t_vec.cross(self.v0v1);

        // Is hit point outside of primitive
        let u = (t_vec.dot(p_vec) * inv_det) as f32;
        if !(0. ..=1.).contains(&u) {
            return None;
        }
        let v = (r.direction.dot(q_vec) * inv_det) as f32;
        if v < 0. || u + v > 1. {
            return None;
        }

        let tt = self.v0v2.dot(q_vec) * inv_det;
        let intersection = r.at(tt);

        // Return false if the hit point parameter t is outside the ray length interval.
        if !ray_length.contains(tt) {
            return None;
        }

        let uv0 = 1. - u - v;
        let uv = Uv::new(
            uv0 * self.uv0.u + u * self.uv1.u + v * self.uv2.u,
            uv0 * self.uv0.v + u * self.uv1.v + v * self.uv2.v,
        );

        let mut normal = self.normal;
        let front_face = r.direction.dot(normal) < 0.;
        if !front_face {
            normal = normal.neg()
        }
        Some(HitRecord {
            hit_point: intersection,
            normal,
            material: &self.mat,
            ray_length: tt,
            uv,
            front_face,
        })
    }

    fn bounding_box(&self) -> &Aabb {
        &self.b_box
    }

    fn is_light(&self) -> bool {
        self.mat.is_light()
    }
}
