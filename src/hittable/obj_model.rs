use crate::geo::vec3::Vec3;
use crate::geo::Uv;
use crate::hittable::bvh::Bvh;
use crate::hittable::Hittables;
use crate::hittable::Hittables::TriangleType;
use crate::hittable::Triangle;
use crate::material::texture::{ImageTexture, SolidColor};
use crate::material::{Lambertian, Materials};
use simple_error::SimpleError;
use std::collections::HashMap;
use std::error::Error;
use tobj::LoadOptions;

/// Reads a Wavefront .obj file and creates a bvh containing
/// all triangles. It also read materials from the referred .mat file.
/// Support for colored and textured lambertian materials.
pub fn load_obj_model(
    path: &str,
    filename: &str,
    scale: f64,
    pos: Vec3,
) -> Result<Hittables, Box<dyn Error>> {
    load_obj_model_with_default_material(
        path,
        filename,
        scale,
        pos,
        Lambertian::new(SolidColor::new(1., 1., 1.), None),
    )
}

/// Reads a Wavefront .obj file and creates a bvh containing
/// all triangles. It also read materials from the referred .mat file.
/// Support for colored and textured lambertian materials.
/// Applies supplied default material if none in model
pub fn load_obj_model_with_default_material(
    path: &str,
    filename: &str,
    scale: f64,
    pos: Vec3,
    default_material: Materials,
) -> Result<Hittables, Box<dyn Error>> {
    let load_options = LoadOptions {
        triangulate: true,
        ..Default::default()
    };

    let filepath = format!("{}{}", path, filename);
    let (models, materials) = tobj::load_obj(&filepath, &load_options)
        .map_err(|_| SimpleError::new(format!("failed to load obj model from {}", &filepath)))?;
    let materials = materials.map_err(|_| format!("failed to load MTL file for {}", &filepath))?;

    let mut mat_map = HashMap::from([(-1, default_material.clone())]);
    for (i, m) in materials.iter().enumerate() {
        match &m.diffuse_texture {
            None => {
                let color = match m.diffuse {
                    None => SolidColor::new(1., 1., 1.),
                    Some(c) => SolidColor::new_from_f32_array(c),
                };
                mat_map.insert(i as i8, Lambertian::new(color, None));
            }
            Some(diffuse_texture_filename) => {
                let albedo_texture =
                    ImageTexture::load(&format!("{}{}", path, diffuse_texture_filename))?;
                let normal_texture = match &m.normal_texture {
                    None => None,
                    Some(normal_texture_filename) => Some(ImageTexture::load(&format!(
                        "{}{}",
                        path, normal_texture_filename
                    ))?),
                };

                mat_map.insert(i as i8, Lambertian::new(albedo_texture, normal_texture));
            }
        }
    }

    let mut triangles = Vec::new();

    for m in models {
        let mesh = &m.mesh;
        for i in (0..mesh.indices.len()).step_by(3) {
            let mut pos_offset = (mesh.indices[i] * 3) as usize;
            let v0 = Vec3::new(
                mesh.positions[pos_offset] as f64,
                mesh.positions[pos_offset + 1] as f64,
                mesh.positions[pos_offset + 2] as f64,
            ) * scale
                + pos;

            pos_offset = (mesh.indices[i + 1] * 3) as usize;
            let v1 = Vec3::new(
                mesh.positions[pos_offset] as f64,
                mesh.positions[pos_offset + 1] as f64,
                mesh.positions[pos_offset + 2] as f64,
            ) * scale
                + pos;

            pos_offset = (mesh.indices[i + 2] * 3) as usize;
            let v2 = Vec3::new(
                mesh.positions[pos_offset] as f64,
                mesh.positions[pos_offset + 1] as f64,
                mesh.positions[pos_offset + 2] as f64,
            ) * scale
                + pos;

            let (uv0, uv1, uv2) = if mesh.texcoords.is_empty() {
                (
                    Uv { u: 0.0, v: 0.0 },
                    Uv { u: 0.0, v: 0.0 },
                    Uv { u: 0.0, v: 0.0 },
                )
            } else {
                let tex_offset1 = (mesh.texcoord_indices[i] * 2) as usize;
                let tex_offset2 = (mesh.texcoord_indices[i + 1] * 2) as usize;
                let tex_offset3 = (mesh.texcoord_indices[i + 2] * 2) as usize;
                (
                    Uv {
                        u: mesh.texcoords[tex_offset1],
                        v: mesh.texcoords[tex_offset1 + 1],
                    },
                    Uv {
                        u: mesh.texcoords[tex_offset2],
                        v: mesh.texcoords[tex_offset2 + 1],
                    },
                    Uv {
                        u: mesh.texcoords[tex_offset3],
                        v: mesh.texcoords[tex_offset3 + 1],
                    },
                )
            };

            let material_id = match mesh.material_id {
                None => -1,
                Some(id) => id as i8,
            };
            let material = match mat_map.get(&material_id) {
                None => default_material.to_owned(),
                Some(m) => m.to_owned(),
            };

            if let TriangleType(t) =
                Triangle::new_with_tex_coords(v0, v1, v2, uv0, uv1, uv2, material)
            {
                triangles.push(t);
            }
        }
    }

    Ok(Bvh::new(triangles))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geo::vec3::ZERO_VECTOR;

    #[test]
    fn missing_file() {
        let res = load_obj_model("resources/obj/", "missing.obj", 1., ZERO_VECTOR);
        assert_eq!(
            "failed to load obj model from resources/obj/missing.obj",
            format!("{}", res.err().unwrap())
        );
    }

    #[test]
    fn missing_material_file() {
        let res = load_obj_model("resources/obj/", "missingMaterialLib.obj", 1., ZERO_VECTOR);
        assert_eq!(
            "failed to load MTL file for resources/obj/missingMaterialLib.obj",
            format!("{}", res.err().unwrap())
        );
    }

    #[test]
    fn missing_image_file() {
        let res = load_obj_model("resources/obj/", "missingImage.obj", 1., ZERO_VECTOR);
        assert!(format!("{}", res.err().unwrap())
            .contains("Failed to load image texture resources/obj/missing.jpg"));
    }

    #[test]
    fn invalid_image_file() {
        let res = load_obj_model("resources/obj/", "invalidImage.obj", 1., ZERO_VECTOR);
        assert!(format!("{}", res.err().unwrap())
            .contains("Failed to load image texture resources/obj/invalidImage.mtl"));
    }
}
