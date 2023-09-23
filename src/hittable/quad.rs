use crate::combine_aabbs;
use crate::geo::transformation::Transformer;
use crate::geo::vec3::{Vec3, ALMOST_ZERO};
use crate::geo::Aabb;
use crate::geo::Ray;
use crate::geo::Uv;
use crate::hittable::Hittables::QuadType;
use crate::hittable::{Hittable, Hittables};
use crate::material::{HitRecord, Material, Materials};
use crate::random::random_normal_float;
use crate::util::interval::{Interval, RAY_INTERVAL};

/// A rectangular flat hittable object
#[derive(Clone, Debug)]
pub struct Quad {
    q: Vec3,
    u: Vec3,
    v: Vec3,
    normal: Vec3,
    d: f64,
    w: Vec3,
    mat: Materials,
    b_box: Aabb,
    area: f64,
}

impl Quad {
    #![allow(clippy::new_ret_no_self)]
    /// Creates a new quad
    pub fn new(
        q: Vec3,
        u: Vec3,
        v: Vec3,
        mat: Materials,
        transformation: &dyn Transformer,
    ) -> Hittables {
        let q = transformation.transform(q, false);
        let u = transformation.transform(u, true);
        let v = transformation.transform(v, true);

        let b_box = combine_aabbs!(
            &Aabb::new_from_2_points(q, q + u),
            &Aabb::new_from_2_points(q, q + v),
            &Aabb::new_from_2_points(q, q + u + v)
        )
        .pad_if_needed();

        let n = u.cross(v);
        let normal = n.unit();

        QuadType(Quad {
            q,
            u,
            v,
            normal,
            d: normal.dot(q),
            w: n / n.dot(n),
            mat,
            b_box,
            area: n.length(),
        })
    }

    /// creates a new box shaped hittable object
    pub fn new_box(
        a: Vec3,
        b: Vec3,
        mat: Materials,
        transformation: &dyn Transformer,
    ) -> Vec<Hittables> {
        let mut sides = Vec::new();

        let min = Vec3::new(a.x.min(b.x), a.y.min(b.y), a.z.min(b.z));
        let max = Vec3::new(a.x.max(b.x), a.y.max(b.y), a.z.max(b.z));

        let dx = Vec3::new(max.x - min.x, 0., 0.);
        let dy = Vec3::new(0., max.y - min.y, 0.);
        let dz = Vec3::new(0., 0., max.z - min.z);

        sides.push(Quad::new(
            Vec3::new(min.x, min.y, max.z),
            dx,
            dy,
            mat.clone(),
            transformation,
        ));
        sides.push(Quad::new(
            Vec3::new(max.x, min.y, max.z),
            dz.neg(),
            dy,
            mat.clone(),
            transformation,
        ));
        sides.push(Quad::new(
            Vec3::new(max.x, min.y, min.z),
            dx.neg(),
            dy,
            mat.clone(),
            transformation,
        ));
        sides.push(Quad::new(
            Vec3::new(min.x, min.y, min.z),
            dz,
            dy,
            mat.clone(),
            transformation,
        ));
        sides.push(Quad::new(
            Vec3::new(min.x, max.y, max.z),
            dx,
            dz.neg(),
            mat.clone(),
            transformation,
        ));
        sides.push(Quad::new(
            Vec3::new(min.x, min.y, min.z),
            dx,
            dz,
            mat,
            transformation,
        ));

        sides
    }
}

impl Hittable for Quad {
    fn pdf_value(&self, origin: Vec3, direction: Vec3) -> f64 {
        let ray = Ray::new(origin, direction);

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
        let p = self.q + self.u * random_normal_float() + self.v * random_normal_float();
        p - origin
    }

    fn hit(&self, r: &Ray, ray_length: &Interval) -> Option<HitRecord> {
        let denom = self.normal.dot(r.direction);

        // No hit if the ray is parallel to the plane
        if denom.abs() < ALMOST_ZERO {
            return None;
        }

        // No hit if the hit point parameter t is outside the ray length interval.
        let t = (self.d - self.normal.dot(r.origin)) / denom;
        if !ray_length.contains(t) {
            return None;
        }

        // Determine the hit point lies within the planar shape using its plane coordinates.
        let hit_point = r.at(t);
        let planar_hit_point_vector = hit_point - self.q;
        let alpha = self.w.dot(planar_hit_point_vector.cross(self.v)) as f32;
        let beta = self.w.dot(self.u.cross(planar_hit_point_vector)) as f32;

        // Is hit point outside of primitive
        if !(0. ..=1.).contains(&alpha) || !(0. ..=1.).contains(&beta) {
            return None;
        }

        let mut normal = self.normal;
        let front_face = r.direction.dot(normal) < 0.;
        if !front_face {
            normal = normal.neg();
        }
        Some(HitRecord::new(
            hit_point,
            normal,
            &self.mat,
            t,
            Uv::new(alpha, beta),
            front_face,
        ))
    }

    fn bounding_box(&self) -> &Aabb {
        &self.b_box
    }

    fn get_lights(&self) -> Vec<Hittables> {
        if self.mat.is_light() {
            vec![QuadType(self.clone())]
        } else {
            vec![]
        }
    }
}
