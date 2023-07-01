use crate::{Hit, Normal, Point, Ray, SurfaceSample};

use glam::f32::Vec3;
use rand::{thread_rng, Rng};

pub fn sample() -> SurfaceSample {
    let mut rng = thread_rng();
    let phi = rng.gen_range(0.0..std::f32::consts::TAU);
    let u = rng.gen_range(-1.0..1.0);

    let x = phi.cos() * (1.0f32 - u * u).sqrt();
    let y = phi.sin() * (1.0f32 - u * u).sqrt();
    let z = u;

    let n = Vec3 { x, y, z };

    SurfaceSample {
        normal: Normal::new(n),
        position: Point(n),
    }
}

// https://misterdanb.github.io/raytracinginrust/
pub fn intersect(ray: &Ray) -> Option<Hit> {
    let oc = ray.origin.0;
    let a = ray.direction.length().powi(2);
    let half_b = oc.dot(ray.direction);
    let c = oc.length().powi(2) - 1.0;
    let discriminant = half_b * half_b - a * c;

    if discriminant < 0.0 {
        return None;
    }

    let t_min = 0.0001 * 1.0;
    let t_max = 100.0;

    // Find the nearest root that lies in the acceptable range
    let sqrtd = discriminant.sqrt();
    let mut root = (-half_b - sqrtd) / a;

    if root < t_min || t_max < root {
        root = (-half_b + sqrtd) / a;
        if root < t_min || t_max < root {
            return None;
        }
    }

    let p = ray.at(root);
    let hit = Hit {
        t: root,
        position: Point(p),
        normal: Normal::new_from_unnormalized(p),
    };

    Some(hit)
}
