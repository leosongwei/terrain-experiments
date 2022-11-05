use glam::{Quat, EulerRot, Vec3, Mat4};

pub struct SimpleCamera {
    position: Vec3,
    angle_rad: Vec3
}

impl SimpleCamera {
    pub fn new(position: Vec3, angle_rad: Vec3) -> Self {
        SimpleCamera { position, angle_rad }
    }

    pub fn get_view_mat(&self) -> Mat4 {
        return Mat4::from_rotation_translation(
            Quat::from_euler(EulerRot::XYZ, self.angle_rad.x, self.angle_rad.y, self.angle_rad.z),
            self.position)
    }
}