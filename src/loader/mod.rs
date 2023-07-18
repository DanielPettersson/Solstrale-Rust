//! Module containing object model loaders

use crate::geo::transformation::Transformer;
use crate::hittable::Hittables;
use crate::material::Materials;
use std::error::Error;

pub mod obj;

/// Common trait for loading object models of different formats
pub trait Loader {
    /// Loads a model
    fn load(
        &self,
        transformation: &dyn Transformer,
        default_material: Option<Materials>,
    ) -> Result<Hittables, Box<dyn Error>>;
}
