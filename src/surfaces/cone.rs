use crate::{Hit, Normal, Point, Ray, SurfaceSample};
use glam::Vec3;
use rand::{thread_rng, Rng};

pub fn intersect(ray: &Ray) -> Option<Hit> {
    let x0 = ray.origin.0.x;
    let y0 = ray.origin.0.y;
    let z0 = ray.origin.0.z;
    let v_x = ray.direction.x;
    let v_y = ray.direction.y;
    let v_z = ray.direction.z;

    let a = v_z.powi(2) - v_x.powi(2) - v_y.powi(2);
    let b = 2.0 * (z0 * v_z - v_z - x0 * v_x - y0 * v_y);
    let c = z0.powi(2) - 2. * z0 + 1. - x0.powi(2) - y0.powi(2);

    let discriminant = b.powi(2) - 4.0 * a * c;
    if discriminant < 0.0 {
        return None;
    }

    if a == 0.0 {
        return None;
    }

    let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
    let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

    if t1 > 0.0001 {
        let position = Point(ray.at(t1));
        if position.0.z > 0.0 && position.0.z < 1.0 {
            let normal =
                Normal::new(Vec3::new(position.0.x, position.0.y, 1. - position.0.z).normalize());
            return Some(Hit {
                normal,
                position,
                t: t1,
            });
        }
    }

    if t2 > 0.0001 {
        let position = Point(ray.at(t2));
        if position.0.z > 0.0 && position.0.z < 1.0 {
            let normal =
                Normal::new(Vec3::new(position.0.x, position.0.y, 1. - position.0.z).normalize());
            return Some(Hit {
                normal,
                position,
                t: t2,
            });
        }
    }

    None
}

pub fn sample() -> SurfaceSample {
    let mut rng = thread_rng();

    // Generate a random angle in radians
    let theta = rng.gen_range(0.0..std::f32::consts::TAU);

    // Generate a random value from a uniform distribution
    let r = (rng.gen_range(0.0..1.0) as f32).sqrt();

    // Calculate the x and y coordinates of the point on the surface of the cone
    let x = r * theta.cos();
    let y = r * theta.sin();

    // Calculate the height of the point on the cone
    let z = 1. - r;

    let normal = Vec3 { x, y, z: 1. - z }.normalize();

    SurfaceSample {
        position: Point(Vec3 { x, y, z }),
        normal: Normal::new(normal),
    }
}
