use crate::dot;
use crate::hit_record;
use crate::random_unit_vector;
use crate::unit_vector;
use crate::vec3::reflect;
use crate::Color;
use crate::Ray;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Lambertian {
  pub albedo: Color,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Metal {
  pub albedo: Color,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Light {
  pub albedo: Color,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum AnyMaterial {
  Metal(Metal),
  Lambertian(Lambertian),
  Light(Light),
}

impl AnyMaterial {
  pub fn scatter(
    &self,
    r_in: Ray,
    rec: &hit_record,
    attenuation: &mut Color,
    scattered: &mut Ray,
  ) -> bool {
    match self {
      Self::Lambertian(lambertian) => {
        let mut scatter_direction = rec.normal + random_unit_vector();

        if scatter_direction.near_zero() {
          scatter_direction = rec.normal;
        }
        *scattered = Ray {
          orig: rec.p,
          dir: scatter_direction,
        };
        *attenuation = lambertian.albedo;
        return true;
      }
      Self::Metal(metal) => {
        let reflected = reflect(unit_vector(r_in.dir), rec.normal);

        *scattered = Ray {
          orig: rec.p,
          dir: reflected,
        };
        *attenuation = metal.albedo;
        return dot(scattered.dir, rec.normal) > 0.0;
      }
      Self::Light(light) => {
        let mut scatter_direction = rec.normal + random_unit_vector();

        if scatter_direction.near_zero() {
          scatter_direction = rec.normal;
        }
        *scattered = Ray {
          orig: rec.p,
          dir: scatter_direction,
        };
        *attenuation = light.albedo;
        return true;
      }
    }
  }
}
