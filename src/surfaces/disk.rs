use crate::{Hit, Normal, Point, Ray, SurfaceSample};

use glam::Vec3;
use rand::{thread_rng, Rng};

pub fn sample() -> SurfaceSample {
    let mut rng = thread_rng();
    let theta = rng.gen_range(0.0..std::f32::consts::TAU);
    let r = (rng.gen_range(0.0..1.0) as f32).sqrt();

    let x = r * theta.cos();
    let y = r * theta.sin();

    SurfaceSample {
        position: Point(Vec3 { x, y, z: 0.0 }),
        normal: Normal::new(Vec3::Z),
    }
}

pub fn intersect(ray: &Ray) -> Option<Hit> {
    // Ensure ray intersects plane
    if ray.direction.z == 0.0 {
        return None;
    }

    // Calculate the intersection point by solving for t in the equation:
    // origin + t * direction = intersection
    // where intersection is a point on the xy plane (z = 0).
    let t = -ray.origin.0.z / ray.direction.z;

    if t < 0.0001 {
        return None;
    }

    let pos = ray.at(t);

    if pos.dot(pos) < 1. {
        Some(Hit {
            normal: Normal::new(Vec3::Z),
            position: Point(pos),
            t,
        })
    } else {
        None
    }
}
