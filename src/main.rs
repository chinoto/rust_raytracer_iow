pub mod camera;
pub mod hit;
pub mod material;
pub mod ray;
pub mod sphere;
pub mod vec;

use crate::{
    camera::Camera,
    material::{Dielectric, Lambertian, Material, Metal},
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

fn random_scene() -> Vec<Box<dyn Hittable>> {
    let mut world: Vec<Box<dyn Hittable>> = Vec::new();

    let ground_material = Arc::new(Lambertian {
        albedo: Vec3(0.5, 0.5, 0.5),
    });
    world.push(Box::new(Sphere::new(
        Vec3(0., -1000., 0.),
        1000.,
        ground_material,
    )));

    Arc::new(Dielectric { ir: 1.5 });
    Arc::new(Metal {
        albedo: Vec3(0.8, 0.6, 0.2),
        fuzz: 0.,
    });

    for a in -11..11 {
        for b in -11..11 {
            let center = Vec3(
                (a as f64) + 0.9 * random::<f64>(),
                0.2,
                (b as f64) + 0.9 * random::<f64>(),
            );
            if (center - Vec3(4., 0.2, 0.)).length() <= 0.9 {
                continue;
            }

            let choose_mat: f32 = random();
            let mat: Arc<dyn Material> = if choose_mat < 0.8 {
                Arc::new(Lambertian {
                    albedo: Vec3::random(0. ..1.) * Vec3::random(0. ..1.),
                })
            } else if choose_mat < 0.95 {
                Arc::new(Metal {
                    albedo: Vec3::random(0.5..1.),
                    fuzz: random::<f64>() * 0.5,
                })
            } else {
                Arc::new(Dielectric { ir: 1.5 })
            };
            world.push(Box::new(Sphere::new(center, 0.2, mat)));
        }
    }

    let material1 = Arc::new(Dielectric { ir: 1.5 });
    world.push(Box::new(Sphere::new(Vec3(0., 1., 0.), 1.0, material1)));

    let material2 = Arc::new(Lambertian {
        albedo: Vec3(0.4, 0.2, 0.1),
    });
    world.push(Box::new(Sphere::new(Vec3(-4., 1., 0.), 1.0, material2)));

    let material3 = Arc::new(Metal {
        albedo: Vec3(0.7, 0.6, 0.5),
        fuzz: 0.0,
    });
    world.push(Box::new(Sphere::new(Vec3(4., 1., 0.), 1.0, material3)));
    world
}
fn main() {
    // Image
    const ASPECT_RATIO: f64 = 3. / 2.;
    const IMAGE_WIDTH: u32 = 1200;
    const IMAGE_HEIGHT: u32 = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as u32;
    const SAMPLES_PER_PIXEL: usize = 500;
    const MAX_DEPTH: u32 = 50;

    // World
    let world = random_scene();

    // Camera
    let look_from = Vec3(13., 2., 3.);
    let look_at = Vec3(0., 0., 0.);
    let vup = Vec3(0., 1., 0.);
    let cam = Camera::new(look_from, look_at, vup, 20., ASPECT_RATIO, 0.1, 10.);

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
