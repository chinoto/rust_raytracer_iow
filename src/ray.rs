use crate::vec::Vec3;

#[derive(Clone, Copy)]
pub struct Ray {
    pub origin: Vec3<f64>,
    pub direction: Vec3<f64>,
}

impl Ray {
    pub fn new(origin: Vec3<f64>, direction: Vec3<f64>) -> Ray {
        Ray { origin, direction }
    }
    pub fn at(self, magnitude: f64) -> Vec3<f64> {
        self.origin + self.direction * magnitude
    }
}
