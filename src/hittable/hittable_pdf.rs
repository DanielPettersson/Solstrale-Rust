use crate::geo::vec3::Vec3;
use crate::hittable::hittable_list::HittableList;
use crate::hittable::Hittable;
use crate::pdf::Pdf;

/// A wrapper for generating pdfs for a list of hittable objects
pub struct HittablePdf<'a> {
    objects: &'a HittableList,
    origin: Vec3,
}

impl<'a> HittablePdf<'a> {
    /// Creates a new instance of HittablePdf
    pub fn new(objects: &'a HittableList, origin: Vec3) -> HittablePdf {
        HittablePdf { objects, origin }
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
