use crate::random;
use std::f64::consts::PI;
use std::fmt;
use std::ops::{Add, Div, Mul, Sub};

/// Vec3 is a 3 dimensional vector
#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/// A value that is so small as to be almost zero
pub const ALMOST_ZERO: f64 = 1e-8;
pub const ZERO_VECTOR: Vec3 = Vec3 {
    x: 0.,
    y: 0.,
    z: 0.,
};

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ x: {}, y: {}, z: {} }}", self.x, self.y, self.z)
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    /// returns a Vec3 that has all values added with corresponding value in given Vec3
    /// # Examples:
    /// ```
    /// # use solstrale::geo::vec3::Vec3;
    /// let res = Vec3::new(1., 2., 3.) + Vec3::new(4., 5., 6.);
    /// assert_eq!(Vec3::new(5., 7., 9.), res)
    /// ```
    fn add(self, v: Self) -> Self::Output {
        Vec3 {
            x: self.x + v.x,
            y: self.y + v.y,
            z: self.z + v.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    /// returns a Vec3 that has all values subtracted by corresponding value in given Vec3
    /// # Examples:
    /// ```
    /// # use solstrale::geo::vec3::Vec3;
    /// let res = Vec3::new(1., 2., 3.) - Vec3::new(6., 5., 4.);
    /// assert_eq!(Vec3::new(-5., -3., -1.), res)
    /// ```
    fn sub(self, v: Self) -> Self::Output {
        Vec3 {
            x: self.x - v.x,
            y: self.y - v.y,
            z: self.z - v.z,
        }
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    /// returns a Vec3 that has all values multiplied with corresponding value in given Vec3
    /// # Examples:
    /// ```
    /// # use solstrale::geo::vec3::Vec3;
    /// let res = Vec3::new(1., 2., 3.) * Vec3::new(4., 5., 6.);
    /// assert_eq!(Vec3::new(4., 10., 18.), res)
    /// ```
    fn mul(self, v: Self) -> Self::Output {
        Vec3 {
            x: self.x * v.x,
            y: self.y * v.y,
            z: self.z * v.z,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    /// returns a Vec3 that has all values multiplied with given scalar
    /// # Examples:
    /// ```
    /// # use solstrale::geo::vec3::Vec3;
    /// let res = Vec3::new(1., 2., 3.) * 2.;
    /// assert_eq!(Vec3::new(2., 4., 6.), res)
    /// ```
    fn mul(self, t: f64) -> Self::Output {
        Vec3 {
            x: self.x * t,
            y: self.y * t,
            z: self.z * t,
        }
    }
}

impl Div<Vec3> for Vec3 {
    type Output = Vec3;

    /// returns a Vec3 that has all values multiplied with corresponding value in given Vec3
    /// # Examples:
    /// ```
    /// # use solstrale::geo::vec3::Vec3;
    /// let res = Vec3::new(1., 2., 3.) / Vec3::new(5., 4., 3.);
    /// assert_eq!(Vec3::new(0.2, 0.5, 1.), res)
    /// ```
    fn div(self, v: Self) -> Self::Output {
        Vec3 {
            x: self.x / v.x,
            y: self.y / v.y,
            z: self.z / v.z,
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    /// returns a Vec3 that has all values multiplied with given scalar
    /// # Examples:
    /// ```
    /// # use solstrale::geo::vec3::Vec3;
    /// let res = Vec3::new(1., 2., 3.) / 2.;
    /// assert_eq!(Vec3::new(0.5, 1., 1.5), res)
    /// ```
    fn div(self, t: f64) -> Self::Output {
        Vec3 {
            x: self.x / t,
            y: self.y / t,
            z: self.z / t,
        }
    }
}

impl Vec3 {
    /// Creates a new Vec3
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    /// returns a Vec3 that has all values negated
    /// # Examples:
    /// ```
    /// # use solstrale::geo::vec3::Vec3;
    /// let res = Vec3::new(1., 2., 3.).neg();
    /// assert_eq!(Vec3::new(-1., -2., -3.), res)
    /// ```
    pub fn neg(&self) -> Vec3 {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }

    /// returns the dot product with given Vec3
    /// # Examples:
    /// ```
    /// # use solstrale::geo::vec3::Vec3;
    /// let res = Vec3::new(1., 2., 3.).dot(Vec3::new(4., 5., 6.));
    /// assert_eq!(32., res)
    /// ```
    pub fn dot(&self, v: Vec3) -> f64 {
        self.x * v.x + self.y * v.y + self.z * v.z
    }

    /// returns the cross product with given Vec3
    /// # Examples:
    /// ```
    /// # use solstrale::geo::vec3::Vec3;
    /// let res = Vec3::new(2., 3., 4.).cross(Vec3::new(5., 6., 7.));
    /// assert_eq!(Vec3::new(-3., 6., -3.), res)
    /// ```
    pub fn cross(&self, v: Vec3) -> Vec3 {
        Vec3 {
            x: self.y * v.z - self.z * v.y,
            y: self.z * v.x - self.x * v.z,
            z: self.x * v.y - self.y * v.x,
        }
    }

    /// return the squared length of the vector
    /// # Examples:
    /// ```
    /// # use solstrale::geo::vec3::Vec3;
    /// let v = Vec3::new(1., 2., 3.);
    /// assert_eq!(14., v.length_squared())
    /// ```
    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    /// return the length of the vector
    /// # Examples:
    /// ```
    /// # use solstrale::geo::vec3::Vec3;
    /// let v = Vec3::new(0., 3., 4.);
    /// assert_eq!(5., v.length())
    /// ```
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    /// returns the vector but sized to a length of 1
    /// # Examples:
    /// ```
    /// # use solstrale::geo::vec3::{random_vec3, Vec3, ALMOST_ZERO};
    /// let v = random_vec3(-10., 10.);
    /// let unit_v = v.unit();
    /// assert!((unit_v.length() - 1.).abs() < ALMOST_ZERO);
    /// assert!(v.dot(unit_v) > 0.)
    /// ```
    pub fn unit(&self) -> Vec3 {
        *self / self.length()
    }

    /// Checks if the vectors length is near zero
    /// # Examples:
    /// ```
    /// # use solstrale::geo::vec3::{Vec3, ZERO_VECTOR};
    /// let v = Vec3::new(1., 2., 3.);
    /// assert!(!v.near_zero());
    /// assert!(ZERO_VECTOR.near_zero());
    /// ```
    pub fn near_zero(&self) -> bool {
        self.x.abs() < ALMOST_ZERO && self.y.abs() < ALMOST_ZERO && self.z.abs() < ALMOST_ZERO
    }

    /// returns the reflection vector given the normal n
    /// # Examples:
    /// ```
    /// # use solstrale::geo::vec3::Vec3;
    /// let v = Vec3::new(0., 3., 4.);
    /// assert_eq!(Vec3::new(0., -3., 4.), v.reflect(Vec3::new(0., 1., 0.)));
    /// assert_eq!(Vec3::new(0., 3., -4.), v.reflect(Vec3::new(0., 0., 1.)))
    /// ```
    pub fn reflect(&self, n: Vec3) -> Vec3 {
        self.sub(n * (self.dot(n) * 2.))
    }

    /// returns the refraction vector given the normal n and the index of refraction of the material
    /// # Examples:
    /// ```
    /// # use solstrale::geo::vec3::Vec3;
    /// let v = Vec3::new(-3., -3., 0.).unit();
    /// let ret = v.refract(Vec3::new(0., 1., 0.), 1.);
    /// assert!((ret - v).near_zero());
    /// ```
    pub fn refract(&self, n: Vec3, index_of_refraction: f64) -> Vec3 {
        let cos_theta = self.neg().dot(n).min(1.);
        let r_out_perpendicular = (n * cos_theta + *self) * index_of_refraction;
        let r_out_parallel = n * (-(1. - r_out_perpendicular.length_squared()).abs().sqrt());
        r_out_perpendicular + r_out_parallel
    }

    /// Returns value of a numbered axis, where x is 0, y is 1 and other is z
    /// # Examples:
    /// ```
    /// # use solstrale::geo::vec3::Vec3;
    /// let v = Vec3::new(1., 2., 3.);
    /// assert_eq!(1., v.axis(0));
    /// assert_eq!(2., v.axis(1));
    /// assert_eq!(3., v.axis(2));
    /// ```
    ///
    pub fn axis(&self, a: u8) -> f64 {
        if a == 0 {
            self.x
        } else if a == 1 {
            self.y
        } else {
            self.z
        }
    }
}

/// Creates a random Vec3 within the given interval
pub fn random_vec3(min: f64, max: f64) -> Vec3 {
    Vec3 {
        x: random::random_float(min, max),
        y: random::random_float(min, max),
        z: random::random_float(min, max),
    }
}

/// Creates a random Vec3 that is shorter than 1
pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = Vec3 {
            x: random::random_float(-1., 1.),
            y: random::random_float(-1., 1.),
            z: random::random_float(-1., 1.),
        };

        if p.length_squared() < 1. {
            return p;
        }
    }
}

/// Creates a random Vec3 that has the length of 1
pub fn random_unit_vector() -> Vec3 {
    random_in_unit_sphere().unit()
}

/// Creates a random Vec3 that is shorter than 1.
/// And in the same general direction as given normal.
pub fn random_in_hemisphere(normal: Vec3) -> Vec3 {
    let in_unit_sphere = random_in_unit_sphere();
    if in_unit_sphere.dot(normal) > 0. {
        return in_unit_sphere;
    }
    in_unit_sphere.neg()
}

/// Creates a random Vec3 that is shorter than 1 and that has a Z value of 0
pub fn random_in_unit_disc() -> Vec3 {
    loop {
        let p = Vec3 {
            x: random::random_float(-1., 1.),
            y: random::random_float(-1., 1.),
            z: 0.,
        };

        if p.length_squared() < 1. {
            return p;
        }
    }
}

/// Generates a random vector similar to RandomUnitVector
/// in that the length is always 1. But with a different distribution
/// as it is generated by two random angles.
pub fn random_cosine_direction() -> Vec3 {
    let r1 = random::random_normal_float();
    let r2 = random::random_normal_float();
    let r2_sqrt = r2.sqrt();

    let phi = 2. * PI * r1;
    let x = phi.cos() * r2_sqrt;
    let y = phi.sin() * r2_sqrt;
    let z = (1. - r2).sqrt();

    Vec3 { x, y, z }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::interval::Interval;

    #[test]
    fn test_random_vec3() {
        let interval = Interval { min: -2., max: 2. };

        for _ in 0..100 {
            let vec = random_vec3(interval.min, interval.max);

            assert!(interval.contains(vec.x), "x = {}", vec.x);
            assert!(interval.contains(vec.y));
            assert!(interval.contains(vec.z));
        }
    }

    #[test]
    fn test_random_in_unit_sphere() {
        for _ in 0..100 {
            let vec = random_in_unit_sphere();
            assert!(vec.length() <= 1.);
        }
    }

    #[test]
    fn test_random_unit_vector() {
        for _ in 0..100 {
            let vec = random_unit_vector();
            assert!((vec.length() - 1.) < ALMOST_ZERO);
        }
    }

    #[test]
    fn test_random_cosine_direction() {
        for _ in 0..100 {
            let vec = random_cosine_direction();
            assert!((vec.length() - 1.) < ALMOST_ZERO);
        }
    }

    #[test]
    fn test_random_in_hemisphere() {
        for _ in 0..100 {
            let normal = random_unit_vector();
            let vec = random_in_hemisphere(normal);
            assert!(
                vec.length() <= 1.,
                "vec {} is not in unit sphere as length is {}",
                vec,
                vec.length()
            );
            assert!(
                vec.dot(normal) > 0.,
                "vec {} is not is not pointing in same general direction as normal {}",
                vec,
                normal
            )
        }
    }

    #[test]
    fn test_random_in_unit_disc() {
        for _ in 0..100 {
            let vec = random_in_unit_disc();
            assert!(vec.length() <= 1.);
            assert_eq!(0., vec.z)
        }
    }
}
