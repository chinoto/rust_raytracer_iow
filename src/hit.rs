use crate::{ray::Ray, vec::Vec3};
use std::ops::Range;

pub struct HitRecord {
    pub point: Vec3<f64>,
    pub normal: Vec3<f64>,
    pub dist: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(point: Vec3<f64>, normal: Vec3<f64>, dist: f64, ray: Ray) -> Self {
        let front_face = ray.direction.dot(normal) < 0.;
        HitRecord {
            point,
            normal: if front_face { normal } else { -normal },
            dist,
            front_face,
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: Ray, range: Range<f64>) -> Option<HitRecord>;
}

impl Hittable for [Box<dyn Hittable>] {
    fn hit(&self, ray: Ray, mut range: Range<f64>) -> Option<HitRecord> {
        self.iter().fold(None, |result, hittable| {
            if let Some(h) = hittable.hit(ray, range.clone()) {
                range.end = h.dist;
                return Some(h);
            }
            result
        })
    }
}
