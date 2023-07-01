pub mod primitives;
pub mod scene;
pub mod surfaces;
pub mod tests;
mod trace_record;
pub mod transform;

pub use scene::Scene;
pub use trace_record::TraceRecord;

use glam::f32::Vec3;
use kiss3d::window::Window;
use rand::{thread_rng, Rng};

#[derive(Clone, Debug)]
pub struct Point(pub Vec3);

#[derive(Clone, Debug)]
pub struct Normal(Vec3);

impl Normal {
    fn new_from_unnormalized(normal: Vec3) -> Self {
        Self(normal.normalize())
    }

    fn new(unit_normal: Vec3) -> Self {
        assert!(unit_normal.is_normalized());
        Self(unit_normal)
    }

    pub fn vec(&self) -> Vec3 {
        self.0
    }
}

#[derive(Debug)]
pub struct Hit {
    pub normal: Normal,
    pub position: Point,
    pub t: f32,
}

#[derive(Clone, Debug)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vec3,
}

impl Ray {
    pub fn at(&self, t: f32) -> Vec3 {
        self.origin.0 + t * self.direction
    }
}

impl Normal {
    pub fn to_diffuse_ray(&self) -> Vec3 {
        let mut rng = thread_rng();
        let g = rng.gen_range(0.0..1.0f32).sqrt().asin();
        let phi = rng.gen_range(0.0..std::f32::consts::TAU);

        let t1 = self.0.any_orthonormal_vector();
        let t2 = t1.cross(self.0).normalize(); // Generate a third vector perpindicular to both

        let x = self.0.x * g.cos() + t1.x * g.sin() * phi.cos() + t2.x * g.sin() * phi.sin();
        let y = self.0.y * g.cos() + t1.y * g.sin() * phi.cos() + t2.y * g.sin() * phi.sin();
        let z = self.0.z * g.cos() + t1.z * g.sin() * phi.cos() + t2.z * g.sin() * phi.sin();

        Vec3 { x, y, z }
    }
}

#[derive(Clone, Debug)]
pub struct SurfaceSample {
    pub position: Point,
    pub normal: Normal,
}

#[derive(Debug)]
pub struct HitRecord {
    pub hit: Hit,
    pub surface_id: usize,
}

pub trait DebugRender {
    fn add_to_window(&self, window: &mut Window);
}
