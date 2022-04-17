use crate::{ray::Ray, vec::Vec3};

pub struct Camera {
    pub origin: Vec3<f64>,
    pub horizontal: Vec3<f64>,
    pub vertical: Vec3<f64>,
    pub lower_left_corner: Vec3<f64>,
    pub u: Vec3<f64>,
    pub v: Vec3<f64>,
    pub w: Vec3<f64>,
    pub lens_radius: f64,
}
impl Camera {
    pub fn new(
        look_from: Vec3<f64>,
        look_at: Vec3<f64>,
        vup: Vec3<f64>,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Self {
        let h = (vfov / 2.).to_radians().tan();
        let viewport_height = 2.0 * h;
        let viewport_width = viewport_height * aspect_ratio;

        let w = (look_from - look_at).unit();
        let u = vup.cross(w).unit();
        let v = w.cross(u);

        let origin = look_from;
        let horizontal = u * viewport_width * focus_dist;
        let vertical = v * viewport_height * focus_dist;
        let lower_left_corner = origin - (horizontal + vertical) / 2. - w * focus_dist;

        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
            u,
            v,
            w,
            lens_radius: aperture / 2.,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = Vec3::random_unit_disk() * self.lens_radius;
        let offset = self.u * rd.0 + self.v * rd.1;
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + self.horizontal * s + self.vertical * t - self.origin - offset,
        )
    }
}
