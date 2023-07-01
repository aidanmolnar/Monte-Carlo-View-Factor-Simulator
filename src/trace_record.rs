use super::{HitRecord, Point, Ray};

use glam::Vec3;
use kiss3d::{nalgebra::Point3, window::Window};

pub struct TraceRecord {
    terminated_early: bool,
    origin: Vec3,
    last_ray: Ray,
    pub entries: Vec<TraceRecordEntry>,
}

impl TraceRecord {
    pub fn new(ray: Ray) -> Self {
        Self {
            terminated_early: false,
            origin: ray.origin.0,
            last_ray: ray,
            entries: Vec::new(),
        }
    }

    pub fn add_entry(&mut self, ray: Ray, hit_record: HitRecord, energy_absorbed: f32) {
        self.last_ray = ray;
        self.entries.push(TraceRecordEntry {
            surface_id: hit_record.surface_id,
            energy_absorbed,
            point: hit_record.hit.position,
        })
    }

    pub fn terminate_early(&mut self) {
        self.terminated_early = true;
    }
}

pub struct TraceRecordEntry {
    pub surface_id: usize,
    pub energy_absorbed: f32,
    point: Point,
}

// TODO: Very hacky, needs to be refactored to be more readable
impl TraceRecord {
    pub fn draw(&self, window: &mut Window) {
        let mut energy = 1.0;

        // Draw the first line segment
        let (point0, point1, color) = if self.entries.is_empty() {
            // If ray missed make it red
            (self.origin, self.last_ray.at(10.0), Point3::new(1., 0., 0.))
        } else {
            let entry = self.entries.first().unwrap();
            energy -= entry.energy_absorbed;
            (self.origin, entry.point.0, Point3::new(0., 1., 0.))
        };

        window.draw_line(
            &Point3::new(point0.x, point0.y, point0.z),
            &Point3::new(point1.x, point1.y, point1.z),
            &color,
        );

        for two_entries in self.entries.windows(2) {
            window.draw_line(
                &Point3::new(
                    two_entries[0].point.0.x,
                    two_entries[0].point.0.y,
                    two_entries[0].point.0.z,
                ),
                &Point3::new(
                    two_entries[1].point.0.x,
                    two_entries[1].point.0.y,
                    two_entries[1].point.0.z,
                ),
                &Point3::new(0., energy, 1. - energy),
            );
            energy -= two_entries[1].energy_absorbed;
        }

        if !self.terminated_early {
            let point0 = &self.last_ray.origin.0;
            let point1 = &self.last_ray.at(10.0);
            window.draw_line(
                &Point3::new(point0.x, point0.y, point0.z),
                &Point3::new(point1.x, point1.y, point1.z),
                &Point3::new(0., energy, 1. - energy),
            );
        }
    }
}
