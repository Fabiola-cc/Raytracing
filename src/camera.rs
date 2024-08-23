use nalgebra_glm::{Vec3, normalize};

pub struct Camera {
    pub eye: Vec3,
    pub center: Vec3,
    pub up: Vec3,
}

impl Camera {
    pub fn new(eye:Vec3, center: Vec3, up: Vec3) -> Self {
        Camera {
            eye,
            center,
            up,
        }
    }

    pub fn basis_change(&self, vector: &Vec3) -> Vec3 {
        let forward = (self.center - self.eye).normalize();
        let right = forward.cross(&self.up).normalize();
        let up = right.cross(&forward).normalize();

        let rotated =
        vector.x * right +
        vector.y * up +
        - vector.z * forward;

        rotated.normalize()
    }
}