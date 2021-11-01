use crate::vec3::Point3;
use crate::Vec3;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Ray {
  pub orig: Point3,
  pub dir: Vec3,
}

impl Ray {
  pub fn at(self, t: f64) -> Vec3 {
    return self.orig + self.dir * t;
  }
}
