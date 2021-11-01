use crate::random;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Div;
use std::ops::DivAssign;
use std::ops::Index;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Neg;
use std::ops::Sub;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3 {
  pub x: f64,
  pub y: f64,
  pub z: f64,
}

impl Sub for Vec3 {
  type Output = Self;

  fn sub(self, other: Self) -> Self::Output {
    Self {
      x: self.x - other.x,
      y: self.y - other.y,
      z: self.z - other.z,
    }
  }
}
impl Neg for Vec3 {
  type Output = Self;

  fn neg(self) -> Self::Output {
    Self {
      x: -self.x,
      y: -self.y,
      z: -self.z,
    }
  }
}

impl Add for Vec3 {
  type Output = Self;

  fn add(self, other: Self) -> Self::Output {
    Self {
      x: self.x + other.x,
      y: self.y + other.y,
      z: self.z + other.z,
    }
  }
}

impl Mul<f64> for Vec3 {
  type Output = Self;

  fn mul(self, other: f64) -> Self::Output {
    Self {
      x: self.x * other,
      y: self.y * other,
      z: self.z * other,
    }
  }
}

impl Mul<Vec3> for Vec3 {
  type Output = Self;

  fn mul(self, other: Vec3) -> Self::Output {
    Self {
      x: self.x * other.x,
      y: self.y * other.y,
      z: self.z * other.z,
    }
  }
}

impl Div<f64> for Vec3 {
  type Output = Self;

  fn div(self, other: f64) -> Self::Output {
    Self {
      x: self.x / other,
      y: self.y / other,
      z: self.z / other,
    }
  }
}

impl AddAssign for Vec3 {
  fn add_assign(&mut self, other: Self) {
    self.x += other.x;
    self.y += other.y;
    self.z += other.z;
  }
}

impl MulAssign<f64> for Vec3 {
  fn mul_assign(&mut self, other: f64) {
    self.x *= other;
    self.y *= other;
    self.z *= other;
  }
}

impl Mul<Vec3> for f64 {
  type Output = Vec3;

  fn mul(self, other: Vec3) -> Vec3 {
    Vec3 {
      x: other.x * self,
      y: other.y * self,
      z: other.z * self,
    }
  }
}

impl DivAssign<f64> for Vec3 {
  fn div_assign(&mut self, other: f64) {
    self.x /= other;
    self.y /= other;
    self.z /= other;
  }
}

impl Vec3 {
  pub fn new(x: f64, y: f64, z: f64) -> Self {
    Self { x: x, y: y, z: z }
  }
  pub fn origin() -> Self {
    Self {
      x: 0.0,
      y: 0.0,
      z: 0.0,
    }
  }
  pub fn length(self) -> f64 {
    f64::sqrt(self.length_squared())
  }
  pub fn length_squared(self) -> f64 {
    self.x * self.x + self.y * self.y + self.z * self.z
  }

  pub fn random() -> Vec3 {
    Vec3 {
      x: random::<f64>(),
      y: random::<f64>(),
      z: random::<f64>(),
    }
  }

  pub fn random_clamp(min: f64, max: f64) -> Vec3 {
    Vec3 {
      x: random::<f64>() * (max - min) + min,
      y: random::<f64>() * (max - min) + min,
      z: random::<f64>() * (max - min) + min,
    }
  }

  pub fn near_zero(self) -> bool {
    let s = 1e-8;
    return self.length_squared() < s * s;
  }
}

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
  return v - 2.0 * dot(v, n) * n;
}
pub fn random_unit_vector() -> Vec3 {
  return unit_vector(random_in_unit_sphere());
}
pub fn random_in_unit_sphere() -> Vec3 {
  let mut p = Vec3::random_clamp(-1.0, 1.0);
  loop {
    if p.length_squared() < 1.0 {
      break;
    }
    p = Vec3::random_clamp(-1.0, 1.0);
  }
  return p;
}
pub fn cross(u: Vec3, v: Vec3) -> Vec3 {
  Vec3 {
    x: u.y * v.z - u.z * v.y,
    y: u.x * v.z - u.z * v.x,
    z: u.x * v.y - u.y * v.x,
  }
}

pub fn dot(u: Vec3, v: Vec3) -> f64 {
  u.x * v.x + u.y * v.y + u.z * v.z
}

pub fn unit_vector(v: Vec3) -> Vec3 {
  v / v.length()
}

pub use Vec3 as Point3;
pub use Vec3 as Color;

impl Index<usize> for Vec3 {
  type Output = f64;
  fn index(&self, index: usize) -> &Self::Output {
    match index {
      0 => &self.x,
      1 => &self.y,
      2 => &self.z,
      _ => panic!("Accessing wrong vector dimension"),
    }
  }
}
