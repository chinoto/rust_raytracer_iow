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
    pub fn write_color(self) {
        println!(
            "{} {} {}",
            (255.999 * self.0).clamp(0., 255.).floor(),
            (255.999 * self.1).clamp(0., 255.).floor(),
            (255.999 * self.2).clamp(0., 255.).floor()
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
