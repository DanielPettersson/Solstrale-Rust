//! Contains transformations that can modify [`Vec3`]
//! Used to translate and rotate hittables
use crate::geo::vec3::Vec3;
use crate::util::degrees_to_radians;

/// A trait used for different transformations on [`Vec3`]
pub trait Transformer {
    /// Applies transformation
    fn transform(&self, _vec: Vec3, only_rotation: bool) -> Vec3;
}

/// A transformer that does nothing
pub struct NopTransformer();

impl Transformer for NopTransformer {
    fn transform(&self, vec: Vec3, _only_rotation: bool) -> Vec3 {
        vec
    }
}

/// A list of transformations to apply to a given [`Vec3`]
pub struct Transformations {
    transformations: Vec<Box<dyn Transformer>>,
}

impl Transformations {
    /// Creates a new list of transformations
    pub fn new(transformations: Vec<Box<dyn Transformer>>) -> Transformations {
        Transformations { transformations }
    }
}

impl Transformer for Transformations {
    fn transform(&self, vec: Vec3, only_rotation: bool) -> Vec3 {
        let mut v = vec;
        for t in &self.transformations {
            v = t.transform(v, only_rotation);
        }
        v
    }
}

/// Translates the position of the given [`Vec3`]
pub struct Translation {
    translation: Vec3,
}

impl Translation {
    /// Creates a new translation
    pub fn new(translation: Vec3) -> Translation {
        Translation { translation }
    }
}

impl Transformer for Translation {
    fn transform(&self, vec: Vec3, only_rotation: bool) -> Vec3 {
        if only_rotation {
            vec
        } else {
            vec + self.translation
        }
    }
}

/// Rotates the given [`Vec3`] around the global x-axis by angle degrees
pub struct RotationX {
    sin_theta: f64,
    cos_theta: f64,
}

impl RotationX {
    /// Creates a new y-rotation
    pub fn new(angle: f64) -> RotationX {
        let radians = degrees_to_radians(angle);
        RotationX {
            sin_theta: radians.sin(),
            cos_theta: radians.cos(),
        }
    }
}

impl Transformer for RotationX {
    fn transform(&self, vec: Vec3, _only_rotation: bool) -> Vec3 {
        Vec3::new(
            vec.x,
            self.cos_theta * vec.y + self.sin_theta * vec.z,
            -self.sin_theta * vec.y + self.cos_theta * vec.z,
        )
    }
}

/// Rotates the given [`Vec3`] around the global y-axis by angle degrees
pub struct RotationY {
    sin_theta: f64,
    cos_theta: f64,
}

impl RotationY {
    /// Creates a new y-rotation
    pub fn new(angle: f64) -> RotationY {
        let radians = degrees_to_radians(angle);
        RotationY {
            sin_theta: radians.sin(),
            cos_theta: radians.cos(),
        }
    }
}

impl Transformer for RotationY {
    fn transform(&self, vec: Vec3, _only_rotation: bool) -> Vec3 {
        Vec3::new(
            self.cos_theta * vec.x + self.sin_theta * vec.z,
            vec.y,
            -self.sin_theta * vec.x + self.cos_theta * vec.z,
        )
    }
}

/// Rotates the given [`Vec3`] around the global z-axis by angle degrees
pub struct RotationZ {
    sin_theta: f64,
    cos_theta: f64,
}

impl RotationZ {
    /// Creates a new y-rotation
    pub fn new(angle: f64) -> RotationZ {
        let radians = degrees_to_radians(angle);
        RotationZ {
            sin_theta: radians.sin(),
            cos_theta: radians.cos(),
        }
    }
}

impl Transformer for RotationZ {
    fn transform(&self, vec: Vec3, _only_rotation: bool) -> Vec3 {
        Vec3::new(
            self.cos_theta * vec.x + self.sin_theta * vec.y,
            -self.sin_theta * vec.x + self.cos_theta * vec.y,
            vec.z,
        )
    }
}

/// Scales the given [`Vec3`] by the given factor
pub struct Scale {
    scale: f64,
}

impl Scale {
    /// Creates a new scale transformer
    pub fn new(scale: f64) -> Scale {
        Scale { scale }
    }
}

impl Transformer for Scale {
    fn transform(&self, vec: Vec3, _only_rotation: bool) -> Vec3 {
        vec * self.scale
    }
}
