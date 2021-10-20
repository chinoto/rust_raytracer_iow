use crate::{ray::Ray, vec::Vec3};

pub struct Camera {
    pub origin: Vec3<f64>,
    pub horizontal: Vec3<f64>,
    pub vertical: Vec3<f64>,
    pub lower_left_corner: Vec3<f64>,
}
impl Camera {
    pub fn new(viewport_height: f64, aspect_ratio: f64, focal_length: f64) -> Self {
        let viewport_width = viewport_height * aspect_ratio;

        let origin = Vec3(0., 0., 0.);
        let horizontal = Vec3(viewport_width, 0., 0.);
        let vertical = Vec3(0., viewport_height, 0.);
        let lower_left_corner = origin - (horizontal + vertical) / 2. - Vec3(0., 0., focal_length);

        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin,
        )
    }
}
