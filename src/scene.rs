use kiss3d::{light::Light, window::Window};
use rayon::prelude::*;

use super::{primitives::Primitive, TraceRecord};

use super::{
    surfaces::{Material, Surface},
    HitRecord, Ray,
};

#[derive(Default)]
pub struct Scene {
    pub surfaces: Vec<Surface>,
}

impl Scene {
    // Cast a ray until it hits a surfacem then return the hit
    pub fn cast_ray(&self, ray: &Ray) -> Option<HitRecord> {
        let mut closest_hit_opt = None;

        // Brute force -> run through all the objects naively
        for (surface_id, object) in self.surfaces.iter().enumerate() {
            if let Some(hit) = object.intersect(ray) {
                if closest_hit_opt.is_none() {
                    closest_hit_opt = Some(HitRecord { surface_id, hit });
                } else if let Some(closest_hit) = &mut closest_hit_opt {
                    // Check if this is closer than previous hit
                    if hit.t < closest_hit.hit.t {
                        *closest_hit = HitRecord { surface_id, hit };
                    }
                }
            }
        }
        closest_hit_opt
    }

    const MAX_REFLECTIONS: u32 = 100;
    const MIN_ENERGY: f32 = 1e-3;

    // Trace ray through reflections etc.
    pub fn trace_ray(&self, mut ray: Ray) -> TraceRecord {
        let mut energy = 1.0;
        let mut reflections = 0;

        let mut record = TraceRecord::new(ray.clone());

        while let Some(hit_record) = self.cast_ray(&ray) {
            let surface = &self.surfaces[hit_record.surface_id];

            let energy_absorbed = energy * surface.emissivity;

            // Update loop variables
            reflections += 1;
            energy -= energy_absorbed;

            // Calculate relfected ray direction
            let reflected_direction = match surface.material {
                Material::Diffuse => hit_record.hit.normal.to_diffuse_ray(),
                Material::Specular => {
                    ray.direction
                        - 2. * ray.direction.dot(hit_record.hit.normal.0) * hit_record.hit.normal.0
                }
            };

            // Create reflected ray
            ray = Ray {
                origin: hit_record.hit.position.clone(),
                direction: reflected_direction,
            };

            // Record reflection
            record.add_entry(ray.clone(), hit_record, energy_absorbed);

            if reflections > Scene::MAX_REFLECTIONS || energy < Scene::MIN_ENERGY {
                record.terminate_early();
                break;
            }
        }

        record
    }

    pub fn view_factors_for_surface(&self, surface: usize, num_rays: usize) -> Vec<f32> {
        // Check the surface id is valid
        assert!(surface < self.surfaces.len());

        // Make vec to hold view factors
        let mut view_factors = vec![0.0f32; self.surfaces.len()];

        // Count energy that hits each surface
        for _ in 0..num_rays {
            // Sample from last surface added to scene
            let sample = self.surfaces[surface].sample();

            let ray = Ray {
                origin: sample.position,
                direction: sample.normal.to_diffuse_ray(),
            };

            let record = self.trace_ray(ray);

            for entry in record.entries {
                view_factors[entry.surface_id] += entry.energy_absorbed;
            }
        }

        // Divide view factors by count
        for view_factor in view_factors.iter_mut() {
            *view_factor /= num_rays as f32;
        }

        view_factors
    }

    pub fn view_factors_for_surface_parallel(&self, surface: usize, num_rays: usize) -> Vec<f32> {
        let num_threads = rayon::current_num_threads();

        let rays_per_thread = num_rays / num_threads;

        let mut view_factors = (0..num_threads)
            .into_par_iter()
            .map(|_| self.view_factors_for_surface(surface, rays_per_thread))
            .reduce(
                || vec![0.0f32; self.surfaces.len()],
                |mut state, value| {
                    for (new, old) in value.iter().zip(state.iter_mut()) {
                        *old += new;
                    }
                    state
                },
            );

        // Divide view factors by thread_count
        for view_factor in view_factors.iter_mut() {
            *view_factor /= num_threads as f32;
        }

        view_factors
    }

    pub fn debug_rays_from_surface(&self, surface: usize, num_rays: usize) {
        // Check the surface id is valid
        assert!(surface < self.surfaces.len());

        let mut records = Vec::new();

        for _ in 0..num_rays {
            // Sample from last surface added to scene
            let sample = self.surfaces[surface].sample();

            let ray = Ray {
                origin: sample.position,
                direction: sample.normal.to_diffuse_ray(),
            };

            let record = self.trace_ray(ray);

            records.push(record);
        }

        // Create debug window
        let mut window = Window::new("Tracer Debug Render");
        window.set_light(Light::StickToCamera);

        // Add mesh surfaces to renderer debug representation
        self.add_debug_meshes(&mut window);

        // Draw ray cast records
        while window.render() {
            for record in &records {
                record.draw(&mut window);
            }
        }
    }

    pub fn add_debug_meshes(&self, window: &mut Window) {
        for surface in &self.surfaces {
            surface.add_debug_mesh(window);
        }
    }

    pub fn add_surface(&mut self, surface: Surface) {
        self.surfaces.push(surface);
    }

    pub fn add_primitive(&mut self, primitive: impl Primitive) {
        primitive.add_surfaces(self);
    }
}
