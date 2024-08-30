use std::ops::*;
use cgmath::Point3;
use lerp::Lerp;
use crate::utils;

#[derive(Copy, Clone, Default)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x: x, y: y, z: z }
    }

    pub fn rad(&self) -> Vector3 {
        return Vector3 { x: utils::degrees_to_radians(self.x), y: utils::degrees_to_radians(self.y), z: utils::degrees_to_radians(self.z) };
    }

    pub fn invert_xy(&self) -> Vector3 {
        return Vector3 { x: self.y, y: self.x, z: self.z };
    }
    
    pub fn lerp_vec(&self, other: Vector3, t: f32) -> Vector3 {
        return Vector3 { x: self.x.lerp(other.x, t), y: self.y.lerp(other.y, t), z: self.z.lerp(other.z, t) };
    }
}

impl Add<Vector3> for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: Vector3) -> Vector3 {
        return Vector3 { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z };
    }
}

impl Add<f32> for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: f32) -> Vector3 {
        return Vector3 { x: self.x + rhs, y: self.y + rhs, z: self.z + rhs };
    }
}

impl AddAssign<Vector3> for Vector3 {
    fn add_assign(&mut self, rhs: Vector3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl AddAssign<f32> for Vector3 {
    fn add_assign(&mut self, rhs: f32) {
        self.x += rhs;
        self.y += rhs;
        self.z += rhs;
    }
}

impl Sub<Vector3> for Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: Vector3) -> Vector3 {
        return Vector3 { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z };
    }
}

impl Sub<f32> for Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: f32) -> Vector3 {
        return Vector3 { x: self.x - rhs, y: self.y - rhs, z: self.z - rhs };
    }
}

impl SubAssign<Vector3> for Vector3 {
    fn sub_assign(&mut self, rhs: Vector3) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl SubAssign<f32> for Vector3 {
    fn sub_assign(&mut self, rhs: f32) {
        self.x -= rhs;
        self.y -= rhs;
        self.z -= rhs;
    }
}

impl Mul<Vector3> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: Vector3) -> Vector3 {
        return Vector3 { x: self.x * rhs.x, y: self.y * rhs.y, z: self.z * rhs.z };
    }
}

impl Mul<f32> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: f32) -> Vector3 {
        return Vector3 { x: self.x * rhs, y: self.y * rhs, z: self.z * rhs };
    }
}

impl MulAssign<Vector3> for Vector3 {
    fn mul_assign(&mut self, rhs: Vector3) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl MulAssign<f32> for Vector3 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl Div<Vector3> for Vector3 {
    type Output = Vector3;

    fn div(self, rhs: Vector3) -> Vector3 {
        return Vector3 { x: self.x / rhs.x, y: self.y / rhs.y, z: self.z / rhs.z };
    }
}

impl Div<f32> for Vector3 {
    type Output = Vector3;

    fn div(self, rhs: f32) -> Vector3 {
        return Vector3 { x: self.x / rhs, y: self.y / rhs, z: self.z / rhs };
    }
}

impl DivAssign<Vector3> for Vector3 {
    fn div_assign(&mut self, rhs: Vector3) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
    }
}

impl DivAssign<f32> for Vector3 {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl Neg for Vector3 {
    type Output = Vector3;

    fn neg(self) -> Self::Output {
        return Vector3 { x: -self.x, y: -self.y, z: -self.z };
    }
}

impl From<Vector3> for [f32; 3] {
    fn from(value: Vector3) -> [f32; 3] {
        return [value.x, value.y, value.z]
    }
}

impl From<Vector3> for Point3<f32> {
    fn from(value: Vector3) -> Self {
        return Point3::new(value.x, value.y, value.z);
    }
}

impl std::fmt::Display for Vector3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}, {}]", self.x, self.y, self.z)
    }
}