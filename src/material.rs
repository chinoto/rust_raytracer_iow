use crate::{hit::HitRecord, ray::Ray, vec::Vec3};
use rand::random;

pub trait Material: Send + Sync {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Vec3<f64>, Ray)>;
}

pub struct Lambertian {
    pub albedo: Vec3<f64>,
}
impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<(Vec3<f64>, Ray)> {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        Some((self.albedo, Ray::new(rec.point, scatter_direction)))
    }
}

pub struct Metal {
    pub albedo: Vec3<f64>,
    pub fuzz: f64,
}
impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Vec3<f64>, Ray)> {
        let reflected =
            r_in.direction.unit().reflect(rec.normal) + Vec3::random_in_unit_sphere() * self.fuzz;
        (reflected.dot(rec.normal) > 0.).then(|| (self.albedo, Ray::new(rec.point, reflected)))
    }
}

pub struct Dielectric {
    pub ir: f64, // Index of refraction
}
impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Vec3<f64>, Ray)> {
        let refraction_ratio = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };
        let unit_direction = r_in.direction.unit();
        let cos_theta = (-unit_direction.dot(rec.normal)).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction =
            if cannot_refract || Dielectric::reflectance(cos_theta, refraction_ratio) > random() {
                unit_direction.reflect(rec.normal)
            } else {
                unit_direction.refract(rec.normal, refraction_ratio)
            };

        Some((Vec3(1., 1., 1.), Ray::new(rec.point, direction)))
    }
}
impl Dielectric {
    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let mut r0 = (1. - ref_idx) / (1. + ref_idx);
        r0 = r0 * r0;
        r0 + (1. - r0) * (1. - cosine).powi(5)
    }
}
