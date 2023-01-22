//! Package pdf provides probability density functions

use crate::geo::onb::Onb;
use crate::geo::vec3::{random_cosine_direction, Vec3};
use crate::random::random_normal_float;
use std::f64::consts::PI;

pub trait Pdf {
    /// Returns the pdf value for a given vector
    fn value(&self, direction: Vec3) -> f64;
    /// Generate random direction for the pdf shape
    fn generate(&self) -> Vec3;
}

/// Returns the pdf value for a given vector for the pdfs.
/// Which is the average of the two base pdfs
pub fn mix_value(p0: &dyn Pdf, p1: &dyn Pdf, direction: Vec3) -> f64 {
    0.5 * p0.value(direction) + 0.5 * p1.value(direction)
}

/// Random direction for the pdfs shape.
/// Which is randomly chosen between the two base pdfs.
pub fn mix_generate(p0: &dyn Pdf, p1: &dyn Pdf) -> Vec3 {
    if random_normal_float() < 0.5 {
        p0.generate()
    } else {
        p1.generate()
    }
}

/// A probability density functions with a cosine distribution
pub struct CosinePdf {
    uvw: Onb,
}

impl CosinePdf {
    /// Creates a new instance of a CosinePdf
    pub fn new(w: Vec3) -> Box<dyn Pdf> {
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
