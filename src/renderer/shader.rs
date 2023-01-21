use crate::geo::ray::Ray;
use crate::geo::vec3::{Vec3, ZERO_VECTOR};
use crate::material::HitRecord;
use crate::renderer::Renderer;

/// Shader calculates the color from a ray hitting a hittable object
pub trait Shader {
    fn shade(&self, renderer: &Renderer, rec: HitRecord, ray: Ray, depth: i32) -> Vec3;
}

/// A full raytracing shader
pub struct PathTracingShader {
    max_depth: i32,
}

impl Shader for PathTracingShader {
    /// Calculates the color using path tracing
    fn shade(&self, renderer: &Renderer, rec: HitRecord, ray: Ray, depth: i32) -> Vec3 {
        if depth >= self.max_depth {
            return ZERO_VECTOR
        }

        let emitted_color = rec.material.emitted(&rec);
        let scatter_res = rec.material.scatter(ray, &rec);

        match scatter_res {
            None => emitted_color,
            Some(scatter_record) => {

                match scatter_record.skip_pdf_ray {
                    Some(skip_pdf_ray) => {
                        let (rc, _, _) = renderer.ray_color(skip_pdf_ray, depth+1);
                        return scatter_record.attenuation * rc
                    }
                    None => {
                        let lightPdf = hittable.NewHittablePdf(renderer.lights, rec.HitPoint)
                        mixturePdf := pdf.NewMixturePdf(lightPdf, scatter_record.Pdf)

                        scattered := geo.NewRay(
                            rec.HitPoint,
                            mixturePdf.Generate(),
                            ray.Time,
                        )
                        pdfVal := mixturePdf.Value(scattered.Direction)
                        scatteringPdf := rec.Material.ScatteringPdf(rec, scattered)
                        rc, _, _ := renderer.rayColor(scattered, depth+1)
                        scatterColor := scatter_record.Attenuation.MulS(scatteringPdf).Mul(rc).DivS(pdfVal)

                        return filterInvalidColorValues(emitted_color.Add(scatterColor))
                    }
                }
            }
        }
    }
}
