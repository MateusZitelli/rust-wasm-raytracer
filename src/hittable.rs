use crate::dot;
use crate::AnyMaterial;
use crate::Color;
use crate::Lambertian;
use crate::Point3;
use crate::Ray;
use crate::Vec3;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct hit_record {
  pub p: Point3,
  pub normal: Vec3,
  pub t: f64,
  pub front_face: bool,
  pub material: AnyMaterial,
}

impl hit_record {
  pub fn set_face_normal(&mut self, r: Ray, outward_normal: Vec3) {
    let front_face = dot(r.dir, outward_normal) < 0.0;
    self.normal = if front_face {
      outward_normal
    } else {
      -outward_normal
    };
  }
}

pub trait Hittable {
  fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut hit_record) -> bool {
    false
  }
}

pub struct hittable_list {
  pub objects: Vec<Box<dyn Hittable>>,
}

impl hittable_list {
  pub fn add(&mut self, object: Box<dyn Hittable>) {
    self.objects.push(object)
  }
}

impl Hittable for hittable_list {
  fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut hit_record) -> bool {
    let mut temp_rec = hit_record {
      p: Point3::origin(),
      normal: Vec3::origin(),
      t: t_max,
      front_face: false,
      material: AnyMaterial::Lambertian(Lambertian {
        albedo: Color::origin(),
      }),
    };
    let mut hit_anything = false;
    let mut closest_so_far = t_max;
    for object in &self.objects {
      if object.hit(r, t_min, closest_so_far, &mut temp_rec) {
        hit_anything = true;
        closest_so_far = temp_rec.t;
        *rec = temp_rec;
      }
    }

    return hit_anything;
  }
}
