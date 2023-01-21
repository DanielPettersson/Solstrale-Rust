//! Package camera provides a camera used by raytracer to shoot rays into the scene

use crate::geo::ray::Ray;
use crate::geo::vec3::{random_in_unit_disc, Vec3};
use crate::random::random_normal_float;
use crate::util::degrees_to_radians;

/// Contains all needed parameters for constructing a camera
pub struct CameraConfig {
    pub vertical_fov_degrees: f64,
    pub aperture_size: f64,
    pub focus_distance: f64,
    pub look_from: Vec3,
    pub look_at: Vec3,
}

/// Contains all data needed to describe a cameras position, field of view and
/// where it is pointing
pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    lens_radius: f64,
}

impl Camera {
    pub fn new(image_width: i32, image_height: i32, c: CameraConfig) -> Camera {
        let aspect_ratio = image_width as f64 / image_height as f64;
        let theta = degrees_to_radians(c.vertical_fov_degrees);
        let h = (theta / 2.).tan();
        let view_port_height = 2. * h;
        let view_port_width = aspect_ratio * view_port_height;

        let w = (c.look_from - c.look_at).unit();
        let u = Vec3::new(0., 1., 0.).cross(w).unit();
        let v = w.cross(u);

        let horizontal = (u * view_port_width) * c.focus_distance;
        let vertical = (v * view_port_height) * c.focus_distance;
        let lower_left_corner =
            c.look_from - (horizontal / 2.) - (vertical / 2.) - (w * c.focus_distance);

        Camera {
            origin: c.look_from,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            lens_radius: c.aperture_size / 2.,
        }
    }

    /// A function for generating a ray for a certain u/v for the raytraced image
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let rd = random_in_unit_disc() * self.lens_radius;
        let offset = self.u * rd.x + self.v * rd.y;

        let r_dir = self.lower_left_corner + (self.horizontal * u) + (self.vertical * v)
            - self.origin
            - offset;
        return Ray::new(self.origin + offset, r_dir, random_normal_float());
    }
}
