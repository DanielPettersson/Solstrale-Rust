//! Package pdf provides probability density functions

use std::f64::consts::PI;

use enum_dispatch::enum_dispatch;

use crate::geo::onb::Onb;
use crate::geo::vec3::{random_cosine_direction, random_unit_vector, Vec3};
use crate::hittable::{Hittable, Hittables};
use crate::random::random_normal_float;

pub const SPHERE_PDF_VALUE: f64 = 1. / (4. * PI);

#[enum_dispatch]
pub trait Pdf {
    /// Returns the pdf value for a given vector
    fn value(&self, direction: Vec3) -> f64;
    /// Generate random direction for the pdf shape
    fn generate(&self) -> Vec3;
}

#[enum_dispatch(Pdf)]
pub enum Pdfs<'a> {
    CosinePdf(CosinePdf),
    HittablePdf(HittablePdf<'a>),
    SpherePdf(SpherePdf),
}

/// Returns the pdf value for a given vector for the pdfs.
/// Which is the average of the two base pdfs
pub fn mix_value(p0: &Pdfs, p1: &Pdfs, direction: Vec3) -> f64 {
    0.5 * p0.value(direction) + 0.5 * p1.value(direction)
}

/// Random direction for the pdfs shape.
/// Which is randomly chosen between the two base pdfs.
pub fn mix_generate(p0: &Pdfs, p1: &Pdfs) -> Vec3 {
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

impl<'a> CosinePdf {
    /// Creates a new instance of a CosinePdf
    pub fn new(w: Vec3) -> Pdfs<'a> {
        Pdfs::CosinePdf(CosinePdf { uvw: Onb::new(w) })
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

/// A wrapper for generating pdfs for a list of hittable objects
pub struct HittablePdf<'a> {
    objects: &'a Hittables,
    origin: Vec3,
}

impl<'a> HittablePdf<'a> {
    /// Creates a new instance of HittablePdf
    pub fn new(objects: &'a Hittables, origin: Vec3) -> Pdfs {
        Pdfs::HittablePdf(HittablePdf { objects, origin })
    }
}

impl<'a> Pdf for HittablePdf<'a> {
    fn value(&self, direction: Vec3) -> f64 {
        self.objects.pdf_value(self.origin, direction)
    }

    fn generate(&self) -> Vec3 {
        self.objects.random_direction(self.origin)
    }
}

/// A probability density functions with a sphere distribution
pub struct SpherePdf();

impl<'a> SpherePdf {
    /// Creates a new instance of SpherePdf
    pub fn new() -> Pdfs<'a> {
        Pdfs::SpherePdf(SpherePdf {})
    }
}

impl Pdf for SpherePdf {
    /// returns the pdf value for a given vector for the SpherePdf
    fn value(&self, _: Vec3) -> f64 {
        SPHERE_PDF_VALUE
    }

    /// Generate random direction for the SpherePdf shape
    fn generate(&self) -> Vec3 {
        random_unit_vector()
    }
}
