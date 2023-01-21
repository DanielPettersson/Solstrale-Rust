//! Package pdf provides probability density functions

use crate::geo::vec3::Vec3;

pub trait Pdf {
    fn value(&self, direction: Vec3) -> f64;
    fn generate(&self) -> Vec3;
}
