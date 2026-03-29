use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }
}

// -Vec3
impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self {
        Self::new(-self.x, -self.y, -self.z)
    }
}

// Vec3 + Vec3
impl Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

// Vec3 - Vec3
impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

// Element-wise multiply (Vec3 * Vec3)
impl Mul for Vec3 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

// Vec3 * scalar (float64)
impl Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, t: f64) -> Self {
        Self::new(self.x * t, self.y * t, self.z * t)
    }
}

// Vec3 * scalar (int32)
impl Mul<i32> for Vec3 {
    type Output = Self;
    fn mul(self, t: i32) -> Self {
        Self::new(self.x * t as f64, self.y * t as f64, self.z * t as f64)
    }
}

// scalar (float64) * Vec3
impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, v: Vec3) -> Vec3 {
        v * self
    }
}

// scalar (int32) * Vec3
impl Mul<Vec3> for i32 {
    type Output = Vec3;
    fn mul(self, v: Vec3) -> Vec3 {
        v * self
    }
}

// Vec3 / scalar (float64)
impl Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, t: f64) -> Self {
        self * (1.0 / t)
    }
}

// Free functions
pub fn dot(u: Vec3, v: Vec3) -> f64 {
    u.x * v.x + u.y * v.y + u.z * v.z
}

#[allow(dead_code)]
pub fn cross(u: Vec3, v: Vec3) -> Vec3 {
    Vec3::new(
        u.y * v.z - u.z * v.y,
        u.z * v.x - u.x * v.z,
        u.x * v.y - u.y * v.x,
    )
}

pub fn unit_vector(v: Vec3) -> Vec3 {
    v / v.length()
}

// Type alias for 3D Point
pub type Point3 = Vec3;
