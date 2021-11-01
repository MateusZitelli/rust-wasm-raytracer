use crate::dot;
use crate::hittable::hit_record;
use crate::hittable::Hittable;
use crate::material::AnyMaterial;
use crate::Point3;
use crate::Ray;

pub struct Sphere {
  pub center: Point3,
  pub radius: f64,
  pub material: AnyMaterial,
}

impl Hittable for Sphere {
  fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut hit_record) -> bool {
    let oc = r.orig - self.center;
    let a = r.dir.length_squared();
    let half_b = dot(oc, r.dir);
    let c = oc.length_squared() - self.radius * self.radius;
    let discriminant = half_b * half_b - a * c;
    if discriminant < 0.0 {
      return false;
    }
    let sqrtd = f64::sqrt(discriminant);

    let root = (-half_b - sqrtd) / a;
    if root < t_min || t_max < root {
      let root2 = (-half_b + sqrtd) / a;
      if root2 < t_min || t_max < root2 {
        return false;
      }
    }

    rec.t = root;
    rec.p = r.at(rec.t);
    let outward_normal = (rec.p - self.center) / self.radius;
    rec.set_face_normal(r, outward_normal);
    rec.material = self.material;

    return true;
  }
}
