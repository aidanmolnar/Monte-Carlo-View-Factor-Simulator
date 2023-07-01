use glam::{Mat3, Mat4, Quat, Vec3};

use super::{Hit, Normal, Point, Ray, SurfaceSample};

#[derive(Clone, Debug)]
pub struct Transform {
    pub m: Mat4,           // local to world
    pub m_inv: Mat4,       // world to local
    pub m_inv_trans: Mat3, // transforms normals local to world
}

// Constructors
impl Transform {
    pub fn new(translation: Vec3, scale: Vec3, axis_z: Vec3, axis_x: Vec3) -> Self {
        let translation = Mat4::from_translation(translation);
        let scale = Mat4::from_scale(scale);

        let rotation_1 = Mat4::from_quat(Quat::from_rotation_arc(Vec3::Z, axis_z));
        let rotation_2 = Mat4::from_quat(Quat::from_rotation_arc(
            rotation_1.transform_vector3(Vec3::X),
            axis_x,
        ));
        let rotation = rotation_2 * rotation_1;

        let m = translation * rotation * scale;

        Self {
            m,
            m_inv: m.inverse(),
            m_inv_trans: Mat3::from_mat4(m).inverse().transpose(),
        }
    }
}

// Transformations
impl Transform {
    pub fn normal_local_to_world(&self, normal: Normal) -> Normal {
        Normal::new_from_unnormalized(self.m_inv_trans * normal.0)
    }

    pub fn point_world_to_local(&self, point: Point) -> Point {
        Point(self.m_inv.transform_point3(point.0))
    }

    pub fn point_local_to_world(&self, point: Point) -> Point {
        Point(self.m.transform_point3(point.0))
    }

    pub fn vec_world_to_local(&self, vector: Vec3) -> Vec3 {
        self.m_inv.transform_vector3(vector)
    }

    pub fn vec_local_to_world(&self, vector: Vec3) -> Vec3 {
        self.m.transform_vector3(vector)
    }

    pub fn ray_world_to_local(&self, ray: Ray) -> Ray {
        Ray {
            origin: self.point_world_to_local(ray.origin),
            direction: self.vec_world_to_local(ray.direction),
        }
    }

    pub fn ray_local_to_world(&self, ray: Ray) -> Ray {
        Ray {
            origin: self.point_local_to_world(ray.origin),
            direction: self.vec_local_to_world(ray.direction),
        }
    }

    pub fn hit_local_to_world(&self, hit: Hit) -> Hit {
        Hit {
            position: self.point_local_to_world(hit.position),
            normal: self.normal_local_to_world(hit.normal),
            t: hit.t,
        }
    }

    pub fn surface_sample_local_to_world(&self, sample: SurfaceSample) -> SurfaceSample {
        SurfaceSample {
            position: self.point_local_to_world(sample.position),
            normal: self.normal_local_to_world(sample.normal),
        }
    }
}
