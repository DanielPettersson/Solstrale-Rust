use crate::geo::vec3::Vec3;

/// Onb is an Orthonormal Basis
#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct Onb {
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
}

impl Onb {
    // Creates a new Orthonormal Basis from the given vector
    pub fn new(w: Vec3) -> Onb {
        let unit_w = w.unit();

        let a = if unit_w.x.abs() > 0.9 {
            Vec3::new(0., 1., 0.)
        } else {
            Vec3::new(1., 0., 0.)
        };
        let v = unit_w.cross(a).unit();
        let u = unit_w.cross(v);

        Onb { u, v, w: unit_w }
    }

    /// Translates the given vector to the Orthonormal Basis
    pub fn local(&self, a: Vec3) -> Vec3 {
        self.u * a.x + self.v * a.y + self.w * a.z
    }
}
