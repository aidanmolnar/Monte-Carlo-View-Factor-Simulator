mod cone;
mod cylinder;
mod disk;
mod rectangle;
mod sphere;

use glam::Vec3;
use kiss3d::{
    nalgebra::{Quaternion, Translation3, UnitQuaternion, Vector3},
    window::Window,
};

use super::transform::Transform;
use super::{Hit, Ray, SurfaceSample};

#[derive(Clone)]
pub enum Collider {
    Sphere,
    Disk,
    Cylinder,
    Rectangle,
    Cone,
}

#[derive(Clone)]
pub enum Material {
    Diffuse,
    Specular,
}

#[derive(Clone)]
pub struct Surface {
    pub transform: Transform,
    collider: Collider,
    pub emissivity: f32,
    pub material: Material,
}

impl Surface {
    pub fn intersect(&self, ray: &Ray) -> Option<Hit> {
        let local_ray = self.transform.ray_world_to_local(ray.clone());

        self.collider
            .intersect(&local_ray)
            .map(|hit| self.transform.hit_local_to_world(hit))
    }

    pub fn sample(&self) -> SurfaceSample {
        let sample = self.collider.sample();

        self.transform.surface_sample_local_to_world(sample)
    }

    pub fn add_debug_mesh(&self, window: &mut Window) {
        let (scale, rotation, translation) = self.transform.m.to_scale_rotation_translation();

        let mut s = match self.collider {
            Collider::Sphere => window.add_sphere(scale.x),
            Collider::Disk => window.add_cylinder(scale.x, 0.0001),
            Collider::Cylinder => window.add_cylinder(scale.x, scale.z),
            Collider::Rectangle => window.add_quad(scale.x, scale.y, 1, 1),
            Collider::Cone => window.add_cone(scale.x, scale.z),
        };

        s.prepend_to_local_translation(&Translation3::new(
            translation.x,
            translation.y,
            translation.z,
        ));

        s.prepend_to_local_rotation(&UnitQuaternion::from_quaternion(Quaternion::new(
            rotation.w, rotation.x, rotation.y, rotation.z,
        )));

        match self.collider {
            Collider::Disk | Collider::Cylinder => {
                s.prepend_to_local_rotation(
                    &UnitQuaternion::rotation_between(&Vector3::y(), &Vector3::z()).unwrap(),
                );
            }
            Collider::Cone => {
                s.prepend_to_local_translation(&Translation3::new(0.0, 0.0, scale.z / 2.));
                s.prepend_to_local_rotation(
                    &UnitQuaternion::rotation_between(&Vector3::y(), &Vector3::z()).unwrap(),
                );
            }
            _ => {}
        };
    }

    pub fn new_sphere(position: Vec3, radius: f32) -> Self {
        let transform = Transform::new(position, Vec3::ONE * radius, Vec3::Z, Vec3::X);
        Surface {
            transform,
            collider: Collider::Sphere,
            emissivity: 1.0,
            material: Material::Diffuse,
        }
    }

    pub fn new_disk(position: Vec3, radius: f32, normal: Vec3) -> Self {
        let transform = Transform::new(
            position,
            Vec3::ONE * radius,
            normal,
            normal.any_orthonormal_vector(),
        );
        Surface {
            transform,
            collider: Collider::Disk,
            emissivity: 1.0,
            material: Material::Diffuse,
        }
    }

    pub fn new_cylinder(position: Vec3, radius: f32, height: f32, axis: Vec3) -> Self {
        let transform = Transform::new(
            position,
            Vec3 {
                x: radius,
                y: radius,
                z: height,
            },
            axis,
            axis.any_orthonormal_vector(),
        );
        Surface {
            transform,
            collider: Collider::Cylinder,
            emissivity: 1.0,
            material: Material::Diffuse,
        }
    }

    pub fn new_rectangle(
        position: Vec3,
        width: f32,
        height: f32,
        normal: Vec3,
        axis_x: Vec3,
    ) -> Self {
        let transform = Transform::new(
            position,
            Vec3 {
                x: width,
                y: height,
                z: 1.0,
            },
            normal,
            axis_x,
        );

        Surface {
            transform,
            collider: Collider::Rectangle,
            emissivity: 1.0,
            material: Material::Diffuse,
        }
    }

    pub fn new_cone(position: Vec3, radius: f32, height: f32, axis: Vec3) -> Self {
        let transform = Transform::new(
            position,
            Vec3 {
                x: radius,
                y: radius,
                z: height,
            },
            axis,
            axis.any_orthonormal_vector(),
        );
        Surface {
            transform,
            collider: Collider::Cone,
            emissivity: 1.0,
            material: Material::Diffuse,
        }
    }

    pub fn to_gray_body(mut self, emissivity: f32) -> Self {
        self.emissivity = emissivity;
        self
    }

    pub fn set_diffuse(mut self) -> Self {
        self.material = Material::Diffuse;
        self
    }

    pub fn set_specular(mut self) -> Self {
        self.material = Material::Specular;
        self
    }
}

// Not sure collider is the right phrase here
impl Collider {
    fn intersect(&self, ray: &Ray) -> Option<Hit> {
        match self {
            Collider::Sphere => sphere::intersect(ray),
            Collider::Disk => disk::intersect(ray),
            Collider::Cylinder => cylinder::intersect(ray),
            Collider::Rectangle => rectangle::intersect(ray),
            Collider::Cone => cone::intersect(ray),
        }
    }

    fn sample(&self) -> SurfaceSample {
        match self {
            Collider::Sphere => sphere::sample(),
            Collider::Disk => disk::sample(),
            Collider::Cylinder => cylinder::sample(),
            Collider::Rectangle => rectangle::sample(),
            Collider::Cone => cone::sample(),
        }
    }
}
