use crate::material::AnyMaterial;
use crate::material::Lambertian;
use crate::material::Light;
use crate::material::Metal;
use rand::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use wasm_bindgen::JsCast;
extern crate web_sys;

mod camera;
mod hittable;
mod material;
mod ray;
mod sphere;
mod vec3;
use crate::camera::Camera;
use crate::hittable::hit_record;
use crate::hittable::hittable_list;
use crate::hittable::Hittable;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::dot;
use crate::vec3::random_unit_vector;
use crate::vec3::unit_vector;
use crate::vec3::Color;
use crate::vec3::Point3;
use crate::vec3::Vec3;

pub fn ray_color<T: Hittable>(ray: Ray, world: &T, depth: u32) -> Color {
    if depth <= 0 {
        return Color::origin();
    }
    let mut rec = hit_record {
        p: Point3::origin(),
        normal: Vec3::origin(),
        t: 0.0,
        front_face: false,
        material: AnyMaterial::Lambertian(Lambertian {
            albedo: Color::origin(),
        }),
    };
    if world.hit(ray, 0.001, 10000000.0, &mut rec) {
        let mut scattered = Ray {
            orig: Vec3::origin(),
            dir: Vec3::origin(),
        };
        let mut attenuation = Color::origin();
        if rec
            .material
            .scatter(ray, &rec, &mut attenuation, &mut scattered)
        {
            return attenuation * ray_color(scattered, world, depth - 1);
        }
        return Color::origin();
    }
    let unit_direction = unit_vector(ray.dir);
    let t = 0.5 * (unit_direction.y + 1.0);
    return (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0);
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();
    // Image
    let image_width = canvas.width();
    let image_height = canvas.height();

    // World
    let mut world = hittable_list { objects: vec![] };
    // Ground
    world.add(Box::new(Sphere {
        center: Point3::new(0.0, -100.5, -1.2),
        radius: 100.0,
        material: AnyMaterial::Lambertian(Lambertian {
            albedo: Color::new(0.8, 0.8, 0.8),
        }),
    }));
    world.add(Box::new(Sphere {
        center: Point3::new(-1.0, 0.0, -1.2),
        radius: 0.5,
        material: AnyMaterial::Lambertian(Lambertian {
            albedo: Color::new(0.8, 0.1, 0.1),
        }),
    }));
    world.add(Box::new(Sphere {
        center: Point3::new(0.0, 0.0, -1.2),
        radius: 0.5,
        material: AnyMaterial::Metal(Metal {
            albedo: Color::new(0.9, 0.9, 0.9),
        }),
    }));
    world.add(Box::new(Sphere {
        center: Point3::new(1.0, 0.0, -1.2),
        radius: 0.5,
        material: AnyMaterial::Metal(Metal {
            albedo: Color::new(0.2, 0.2, 0.9),
        }),
    }));

    let cam = Camera::new(image_width, image_height);

    // Render
    let samples_per_pixel = 100;
    let max_depth = 50;
    let data = &mut vec![255; (image_width * image_height * 4) as usize];
    let mut rng = thread_rng();
    for i in 0..image_width {
        for j in 0..image_height {
            let mut pixel_color = Color::origin();
            for s in 0..samples_per_pixel {
                let u = (i as f64 + rng.gen::<f64>()) / (image_width - 1) as f64;
                let v = (j as f64 + rng.gen::<f64>()) / (image_height - 1) as f64;
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(r, &world, max_depth);
            }

            let index = 4 * (image_width * (image_height - j - 1) + i);
            data[(index + 0) as usize] = (clamp(
                f64::sqrt(pixel_color[0] / samples_per_pixel as f64),
                0.0,
                0.999,
            ) * 256.0) as u8;
            data[(index + 1) as usize] = (clamp(
                f64::sqrt(pixel_color[1] / samples_per_pixel as f64),
                0.0,
                0.999,
            ) * 256.0) as u8;
            data[(index + 2) as usize] = (clamp(
                f64::sqrt(pixel_color[2] / samples_per_pixel as f64),
                0.0,
                0.999,
            ) * 256.0) as u8;
            data[(index + 3) as usize] = 255;
        }
    }
    let image_data =
        web_sys::ImageData::new_with_u8_clamped_array(Clamped(&mut data[..]), image_width).unwrap();
    context.put_image_data(&image_data, 0.0, 0.0).unwrap();
}
