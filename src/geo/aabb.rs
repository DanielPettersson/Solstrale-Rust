use crate::geo::ray::Ray;
use crate::geo::vec3::Vec3;
use crate::util::interval::{combine_intervals, Interval, EMPTY_INTERVAL};
use std::ops::Add;

const PAD_DELTA: f64 = 0.0001;

/// Axis Aligned Bounding Box
#[derive(Clone)]
pub struct Aabb {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl Aabb {
    pub fn new_with_empty_intervals() -> Aabb {
        Aabb {
            x: EMPTY_INTERVAL,
            y: EMPTY_INTERVAL,
            z: EMPTY_INTERVAL,
        }
    }

    pub fn new_from_2_points(a: Vec3, b: Vec3) -> Aabb {
        Aabb {
            x: Interval {
                min: a.x.min(b.x),
                max: a.x.max(b.x),
            },
            y: Interval {
                min: a.y.min(b.y),
                max: a.y.max(b.y),
            },
            z: Interval {
                min: a.z.min(b.z),
                max: a.z.max(b.z),
            },
        }
    }

    pub fn new_from_3_points(a: Vec3, b: Vec3, c: Vec3) -> Aabb {
        Aabb {
            x: Interval {
                min: a.x.min(b.x).min(c.x),
                max: a.x.max(b.x).max(c.x),
            },
            y: Interval {
                min: a.y.min(b.y).min(c.y),
                max: a.y.max(b.y).max(c.y),
            },
            z: Interval {
                min: a.z.min(b.z).min(c.z),
                max: a.z.max(b.z).max(c.z),
            },
        }
    }

    pub fn combine_aabbs(a: &Aabb, b: &Aabb) -> Aabb {
        Aabb {
            x: combine_intervals(a.x, b.x),
            y: combine_intervals(a.y, b.y),
            z: combine_intervals(a.z, b.z),
        }
    }

    pub fn pad_if_needed(&self) -> Aabb {
        let new_x = if self.x.size() >= PAD_DELTA {
            self.x
        } else {
            self.x.expand(PAD_DELTA)
        };
        let new_y = if self.y.size() >= PAD_DELTA {
            self.y
        } else {
            self.y.expand(PAD_DELTA)
        };
        let new_z = if self.z.size() >= PAD_DELTA {
            self.z
        } else {
            self.z.expand(PAD_DELTA)
        };

        return Aabb {
            x: new_x,
            y: new_y,
            z: new_z,
        };
    }

    pub fn hit(&self, r: Ray) -> bool {
        let mut t1 = (self.x.min - r.origin.x) * r.direction_inverted.x;
        let mut t2 = (self.x.max - r.origin.x) * r.direction_inverted.x;

        let mut tmin = t1.min(t2);
        let mut tmax = t1.max(t2);

        t1 = (self.y.min - r.origin.y) * r.direction_inverted.y;
        t2 = (self.y.max - r.origin.y) * r.direction_inverted.y;

        tmin = tmin.max(t1.min(t2));
        tmax = tmax.min(t1.max(t2));

        t1 = (self.z.min - r.origin.z) * r.direction_inverted.z;
        t2 = (self.z.max - r.origin.z) * r.direction_inverted.z;

        tmin = tmin.max(t1.min(t2));
        tmax = tmax.min(t1.max(t2));

        return tmax > tmin.max(0.);
    }

    pub fn center(&self) -> Vec3 {
        return Vec3::new(
            self.x.min + self.x.max * 0.5,
            self.y.min + self.y.max * 0.5,
            self.z.min + self.z.max * 0.5,
        );
    }
}

impl Add<Vec3> for &Aabb {
    type Output = Aabb;

    fn add(self, rhs: Vec3) -> Self::Output {
        Aabb {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}
