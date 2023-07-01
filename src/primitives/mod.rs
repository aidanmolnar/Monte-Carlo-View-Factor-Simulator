use glam::Vec3;

use super::{surfaces::Surface, Scene};

pub trait Primitive {
    fn add_surfaces(&self, scene: &mut Scene);
}

pub struct Cube {
    pub translation: Vec3,
    pub axis_y: Vec3,
    pub axis_x: Vec3,
    pub scale: Vec3,
}

pub struct Cylinder {
    pub translation: Vec3,
    pub axis: Vec3,
    pub height: f32,
    pub radius: f32,
}

pub struct Cone {
    pub translation: Vec3,
    pub axis: Vec3,
    pub height: f32,
    pub radius: f32,
}

impl Primitive for Cube {
    fn add_surfaces(&self, scene: &mut Scene) {
        let i = self.axis_x;
        let j = self.axis_y;
        let k = i.cross(j).normalize();

        // Add +x face
        scene.add_surface(Surface::new_rectangle(
            self.translation + i * self.scale.x / 2.,
            self.scale.y,
            self.scale.z,
            i,
            j,
        ));

        // Add -x face
        scene.add_surface(Surface::new_rectangle(
            self.translation - i * self.scale.x / 2.,
            self.scale.y,
            self.scale.z,
            -i,
            j,
        ));

        // Add +y face
        scene.add_surface(Surface::new_rectangle(
            self.translation + j * self.scale.y / 2.,
            self.scale.x,
            self.scale.z,
            j,
            i,
        ));

        // Add -y face
        scene.add_surface(Surface::new_rectangle(
            self.translation - j * self.scale.y / 2.,
            self.scale.x,
            self.scale.z,
            -j,
            i,
        ));

        // Add +z face
        scene.add_surface(Surface::new_rectangle(
            self.translation + k * self.scale.z / 2.,
            self.scale.x,
            self.scale.y,
            k,
            i,
        ));

        // Add -z face
        scene.add_surface(Surface::new_rectangle(
            self.translation - k * self.scale.z / 2.,
            self.scale.x,
            self.scale.y,
            -k,
            -i,
        ));
    }
}

impl Primitive for Cylinder {
    fn add_surfaces(&self, scene: &mut Scene) {
        let axis = self.axis.normalize();

        // Add top face
        scene.add_surface(Surface::new_disk(
            self.translation + axis * self.height / 2.,
            self.radius,
            axis,
        ));

        // Add bottom face
        scene.add_surface(Surface::new_disk(
            self.translation - axis * self.height / 2.,
            self.radius,
            -axis,
        ));

        // Add side face
        scene.add_surface(Surface::new_cylinder(
            self.translation,
            self.radius,
            self.height,
            axis,
        ));
    }
}

impl Primitive for Cone {
    fn add_surfaces(&self, scene: &mut Scene) {
        let axis = self.axis.normalize();

        // Add base face
        scene.add_surface(Surface::new_disk(self.translation, self.radius, -axis));

        // Add side face
        scene.add_surface(Surface::new_cone(
            self.translation,
            self.radius,
            self.height,
            axis,
        ));
    }
}
