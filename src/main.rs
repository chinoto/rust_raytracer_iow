pub mod hit;
pub mod ray;
pub mod sphere;
pub mod vec;
use hit::Hittable;
use ray::Ray;
use sphere::Sphere;
use std::f64::INFINITY;
use vec::Vec3;

fn ray_color(r: Ray, world: &[Box<dyn Hittable>]) -> Vec3<f64> {
    if let Some(h) = world.hit(r, 0.0..INFINITY) {
        return (h.normal + Vec3(1., 1., 1.)) * 0.5;
    }
    let unit_direction = r.direction.unit();
    let t = 0.5 * (unit_direction.1 + 1.0);
    Vec3(1.0, 1.0, 1.0) * (1.0 - t) + Vec3(0.5, 0.7, 1.0) * t
}

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 16. / 9.;
    const IMAGE_WIDTH: u32 = 400;
    const IMAGE_HEIGHT: u32 = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as u32;

    // World
    let world: Vec<Box<dyn Hittable>> = vec![
        Box::new(Sphere::new(Vec3(0., 0., -1.), 0.5)),
        Box::new(Sphere::new(Vec3(0., -100.5, -1.), 100.)),
    ];

    // Camera
    let viewport_height = 2.;
    let viewport_width = viewport_height as f64 * ASPECT_RATIO;
    let focal_length = 1.;

    let origin = Vec3(0., 0., 0.);
    let horizontal = Vec3(viewport_width, 0., 0.);
    let vertical = Vec3(0., viewport_height, 0.);
    let lower_left_corner = origin - (horizontal + vertical) / 2. - Vec3(0., 0., focal_length);

    // Render
    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);
    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {}", j);
        for i in 0..IMAGE_WIDTH {
            let u = i as f64 / ((IMAGE_WIDTH - 1) as f64);
            let v = j as f64 / ((IMAGE_HEIGHT - 1) as f64);
            let r = Ray::new(
                origin,
                lower_left_corner + horizontal * u + vertical * v - origin,
            );
            ray_color(r, &world).write_color();
        }
    }
}
