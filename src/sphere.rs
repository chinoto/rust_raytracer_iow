use crate::{
    hit::{HitRecord, Hittable},
    material::Material,
    vec::Vec3,
};
use std::sync::Arc;

pub struct Sphere {
    pub center: Vec3<f64>,
    pub radius: f64,
    pub mat: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vec3<f64>, radius: f64, mat: Arc<dyn Material>) -> Self {
        Sphere {
            center,
            radius,
            mat,
        }
    }
}

impl Hittable for Sphere {
    fn hit(
        &self,
        r: crate::ray::Ray,
        range: std::ops::Range<f64>,
    ) -> Option<crate::hit::HitRecord> {
        let oc = r.origin - self.center;
        let a = r.direction.length_squared();
        let half_b = oc.dot(r.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0. {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        let dist = Some((-half_b - sqrtd) / a)
            .filter(|root| range.contains(root))
            .or_else(|| Some((-half_b + sqrtd) / a))
            .filter(|root| range.contains(root))?;
        let point = r.at(dist);
        let normal = (point - self.center) / self.radius;
        Some(HitRecord::new(point, normal, self.mat.clone(), dist, r))
    }
}
