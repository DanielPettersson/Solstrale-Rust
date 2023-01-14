use crate::geo::vec3::Vec3;

/// Defines a ray of light used by the ray tracer
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
    pub direction_inverted: Vec3,
    pub time: f64,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3, time: f64) -> Ray {
        let dir = direction.unit();
        let dir_inv = Vec3::new(1. / dir.x, 1. / dir.y, 1. / dir.z);

        Ray {
            origin,
            direction: dir,
            direction_inverted: dir_inv,
            time,
        }
    }

    /// returns the position at a given length of the ray
    pub fn at(&self, distance: f64) -> Vec3 {
        self.origin.add(self.direction.mul_s(distance))
    }
}

#[cfg(test)]
mod tests {
    use crate::geo::ray::Ray;
    use crate::geo::vec3::Vec3;
    use crate::random;

    #[test]
    fn test_at() {
        let origin = Vec3::new(1., 2., 3.);
        let direction = Vec3::new(4., 5., 6.);
        let l = direction.length();

        let r = Ray::new(origin, direction, random::random_normal_float());

        assert_eq!(r.at(0.), origin);
        assert!(r.at(l).sub(origin.add(direction)).near_zero());
        assert!(r.at(-l).sub(origin.sub(direction)).near_zero());
        assert!(r.at(l * 3.).sub(Vec3::new(13., 17., 21.)).near_zero());
    }
}
