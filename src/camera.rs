use crate::dot;
use crate::Point3;
use crate::Ray;
use crate::Vec3;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Camera {
  pub origin: Point3,
  pub lower_left_corner: Point3,
  pub horizontal: Vec3,
  pub vertical: Vec3,
}

impl Camera {
  pub fn new(image_width: u32, image_height: u32) -> Camera {
    let viewport_height = 2.0;
    let viewport_width = image_width as f64 / image_height as f64 * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
      origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    Camera {
      origin,
      horizontal,
      vertical,
      lower_left_corner,
    }
  }

  pub fn get_ray(self, u: f64, v: f64) -> Ray {
    Ray {
      orig: self.origin,
      dir: self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
    }
  }
}
