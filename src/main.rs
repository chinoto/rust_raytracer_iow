pub mod camera;
pub mod hit;
pub mod material;
pub mod ray;
pub mod sphere;
pub mod vec;

use crate::{
    camera::Camera,
    material::{Dielectric, Lambertian, Metal},
};
use hit::Hittable;
use rand::random;
use ray::Ray;
use rayon::prelude::*;
use sphere::Sphere;
use std::{f64::INFINITY, sync::Arc};
use vec::Vec3;

fn ray_color(r: Ray, world: &[Box<dyn Hittable>], depth: u32) -> Vec3<f64> {
    if depth == 0 {
        return Vec3(0., 0., 0.);
    }
    if let Some(rec) = world.hit(r, 0.001..INFINITY) {
        if let Some((attenuation, scattered)) = rec.mat.scatter(&r, &rec) {
            return attenuation * ray_color(scattered, world, depth - 1);
        }
        return Vec3(0., 0., 0.);
    }
    let unit_direction = r.direction.unit();
    let t = 0.5 * (unit_direction.1 + 1.0);
    Vec3(1.0, 1.0, 1.0) * (1.0 - t) + Vec3(0.5, 0.7, 1.0) * t
}

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 16. / 9.;
    const IMAGE_WIDTH: u32 = 800;
    const IMAGE_HEIGHT: u32 = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as u32;
    const SAMPLES_PER_PIXEL: usize = 100;
    const MAX_DEPTH: u32 = 50;

    // World
    let material_ground = Arc::new(Lambertian {
        albedo: Vec3(0.8, 0.8, 0.),
    });
    let material_center = Arc::new(Lambertian {
        albedo: Vec3(0.1, 0.2, 0.5),
    });
    let material_left = Arc::new(Dielectric { ir: 1.5 });
    let material_left2 = material_left.clone();
    let material_right = Arc::new(Metal {
        albedo: Vec3(0.8, 0.6, 0.2),
        fuzz: 0.,
    });

    let world: Vec<Box<dyn Hittable>> = vec![
        Box::new(Sphere::new(Vec3(0.0, -100.5, -1.0), 100.0, material_ground)),
        Box::new(Sphere::new(Vec3(0.0, 0.0, -1.0), 0.5, material_center)),
        Box::new(Sphere::new(Vec3(-1.0, 0.0, -1.0), 0.5, material_left)),
        Box::new(Sphere::new(Vec3(-1.0, 0.0, -1.0), -0.45, material_left2)),
        Box::new(Sphere::new(Vec3(1.0, 0.0, -1.0), 0.5, material_right)),
    ];

    // Camera
    let cam = Camera::new(
        Vec3(-2., 2., 1.),
        Vec3(0., 0., -1.),
        Vec3(0., 1., 0.),
        20.,
        ASPECT_RATIO,
    );

    // Render
    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);
    let mut scratch = Vec::new();
    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {}", j);
        (0..IMAGE_WIDTH)
            .into_par_iter()
            .map(|i| {
                (0..SAMPLES_PER_PIXEL).fold(Vec3(0., 0., 0.), |acc, _| {
                    let u = (i as f64 + random::<f64>()) / ((IMAGE_WIDTH - 1) as f64);
                    let v = (j as f64 + random::<f64>()) / ((IMAGE_HEIGHT - 1) as f64);
                    let r = cam.get_ray(u, v);
                    ray_color(r, &world, MAX_DEPTH) + acc
                }) / (SAMPLES_PER_PIXEL as f64)
            })
            .collect_into_vec(&mut scratch);
        scratch.drain(..).for_each(Vec3::write_color);
    }
}
