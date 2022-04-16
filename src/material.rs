use crate::{hit::HitRecord, ray::Ray, vec::Vec3};

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
