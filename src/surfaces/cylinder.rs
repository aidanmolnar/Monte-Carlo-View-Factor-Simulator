use crate::{Hit, Normal, Point, Ray, SurfaceSample};
use glam::Vec3;
use rand::{thread_rng, Rng};

pub fn intersect(ray: &Ray) -> Option<Hit> {
    let a = ray.direction.x.powi(2) + ray.direction.y.powi(2);
    let b: f32 = 2.0 * (ray.origin.0.x * ray.direction.x + ray.origin.0.y * ray.direction.y);
    let c = ray.origin.0.x.powi(2) + ray.origin.0.y.powi(2) - 1.0;

    let discriminant = b.powi(2) - 4.0 * a * c;
    if discriminant < 0.0 {
        return None;
    }

    if a == 0.0 {
        return None;
    }

    let t1 = (-b + discriminant.sqrt()) / (2.0 * a);
    let t2 = (-b - discriminant.sqrt()) / (2.0 * a);

    // Get the smallest non-zero t
    let mut t = t1;
    if t < 0.0001 {
        t = t2;
    }
    if t < 0.0001 {
        return None;
    }

    let position = ray.origin.0 + t * ray.direction;
    if (position.z < -0.5) || (position.z > 0.5) {
        return None;
    }

    let normal = Normal::new_from_unnormalized(Vec3::new(position.x, position.y, 0.0));

    Some(Hit {
        normal,
        position: Point(position),
        t,
    })
}

pub fn sample() -> SurfaceSample {
    let mut rng = thread_rng();

    let theta = rng.gen_range(0.0..std::f32::consts::TAU);
    let x = theta.cos();
    let y = theta.sin();

    let z = rng.gen_range(-0.5..0.5);

    let position = Point(Vec3 { x, y, z });
    let normal = Normal::new(Vec3::new(x, y, 0.0));
    SurfaceSample { position, normal }
}
