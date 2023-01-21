//! Package pdf provides probability density functions

use crate::geo::onb::Onb;
use crate::geo::vec3::{random_cosine_direction, Vec3};
use std::f64::consts::PI;

pub trait Pdf {
    /// Returns the pdf value for a given vector
    fn value(&self, direction: Vec3) -> f64;
    /// Generate random direction for the pdf shape
    fn generate(&self) -> Vec3;
}

/// A probability density functions with a cosine distribution
pub struct CosinePdf {
    uvw: Onb,
}

impl CosinePdf {
    /// Creates a new instance of a CosinePdf
    pub(crate) fn new(w: Vec3) -> Box<dyn Pdf> {
        Box::new(CosinePdf { uvw: Onb::new(w) })
    }
}

impl Pdf for CosinePdf {
    fn value(&self, direction: Vec3) -> f64 {
        let cosine_theta = direction.unit().dot(self.uvw.w);
        (cosine_theta / PI).max(0.)
    }

    fn generate(&self) -> Vec3 {
        self.uvw.local(random_cosine_direction())
    }
}
