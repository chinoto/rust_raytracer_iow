use rand::random;
use std::ops::*;

#[derive(Clone, Copy, Debug)]
pub struct Vec3<T>(pub T, pub T, pub T);

impl<T: Copy + Add<Output = T> + Mul<Output = T>> Vec3<T> {}

impl Vec3<f64> {
    pub fn dot(self, rhs: Vec3<f64>) -> f64 {
        self.0 * rhs.0 + self.1 * rhs.1 + self.2 * rhs.2
    }
    pub fn cross(self, rhs: Vec3<f64>) -> Vec3<f64> {
        Vec3(
            self.1 * rhs.2 - self.2 * rhs.1,
            self.2 * rhs.0 - self.0 * rhs.2,
            self.0 * rhs.1 - self.1 * rhs.0,
        )
    }
    pub fn length_squared(self) -> f64 {
        self.dot(self)
    }
    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }
    pub fn unit(self) -> Vec3<f64> {
        self / self.length()
    }
    pub fn random(range: Range<f64>) -> Vec3<f64> {
        Vec3(random::<f64>(), random::<f64>(), random::<f64>()) * (range.end - range.start)
            + Vec3(1., 1., 1.) * range.start
    }
    pub fn random_in_unit_sphere() -> Vec3<f64> {
        loop {
            let vec = Vec3::random(-1. ..1.);
            if vec.length_squared() < 1. {
                return vec;
            }
        }
    }
    pub fn random_unit_vector() -> Vec3<f64> {
        Vec3::random_in_unit_sphere().unit()
    }
    pub fn random_unit_disk() -> Vec3<f64> {
        loop {
            let vec = Vec3(random(), random(), 0.) * 2. - Vec3(1., 1., 0.);
            if vec.length_squared() < 1. {
                return vec;
            }
        }
    }
    pub fn near_zero(self) -> bool {
        let threshold = 1e-8;
        [self.0, self.1, self.2].iter().all(|d| d.abs() < threshold)
    }
    pub fn reflect(self, normal: Vec3<f64>) -> Vec3<f64> {
        self - normal * 2.0 * self.dot(normal)
    }
    pub fn refract(self, normal: Vec3<f64>, etai_over_etat: f64) -> Vec3<f64> {
        let cos_theta = (-self.dot(normal)).min(1.0);
        let r_out_perp = (self + normal * cos_theta) * etai_over_etat;
        let r_out_parallel = normal * -(1.0 - r_out_perp.length_squared()).abs().sqrt();
        r_out_perp + r_out_parallel
    }
    pub fn write_color(self) {
        println!(
            "{} {} {}",
            (255.999 * self.0.sqrt()).clamp(0., 255.).floor(),
            (255.999 * self.1.sqrt()).clamp(0., 255.).floor(),
            (255.999 * self.2.sqrt()).clamp(0., 255.).floor()
        );
    }
}

impl<T: Add<Output = T>> Add for Vec3<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl<T: Sub<Output = T>> Sub for Vec3<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl<T: Neg<Output = T>> Neg for Vec3<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec3(-self.0, -self.1, -self.2)
    }
}

impl<T: Mul<Output = T>> Mul for Vec3<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec3(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

impl<T: Copy + Mul<Output = T>> Mul<T> for Vec3<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Vec3(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl<T: Div<Output = T>> Div for Vec3<T> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Vec3(self.0 / rhs.0, self.1 / rhs.1, self.2 / rhs.2)
    }
}

impl<T: Copy + Div<Output = T>> Div<T> for Vec3<T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Vec3(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}
