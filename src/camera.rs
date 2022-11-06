use glam::{EulerRot, Mat4, Quat, Vec3, Vec4};

pub struct SimpleCamera {
    pub position: Vec3,
    pub angle_rad: Vec3,
    pub fov_y_rad: f32,
    pub aspect_ratio_w_over_h: f32,
    pub z_near: f32,
}

impl SimpleCamera {
    pub fn new(
        position: Vec3,
        angle_rad: Vec3,
        fov_y_rad: f32,
        aspect_ratio_w_over_h: f32,
        z_near: f32,
    ) -> Self {
        SimpleCamera {
            position,
            angle_rad,
            fov_y_rad,
            aspect_ratio_w_over_h,
            z_near,
        }
    }

    pub fn get_view_mat(&self) -> Mat4 {
        return Mat4::perspective_infinite_lh(
            self.fov_y_rad,
            self.aspect_ratio_w_over_h,
            self.z_near,
        ) * Mat4::from_rotation_translation(
            Quat::from_euler(
                EulerRot::XYZ,
                self.angle_rad.x,
                self.angle_rad.y,
                self.angle_rad.z,
            ),
            Vec3::new(0.0, 0.0, 0.0),
        ) * Mat4::from_translation(Vec3::new(
            -self.position.x,
            -self.position.y,
            self.position.z,
        ));
    }

    pub fn move_camera(&mut self, movement: Vec3) {
        let rotated_movement = Mat4::from_rotation_y(self.angle_rad.y)
            * Vec4::new(movement.x, movement.y, movement.z, 1.0);
        self.position += Vec3::new(rotated_movement.x, rotated_movement.y, rotated_movement.z);
    }

    pub fn rotate_camera(&mut self, rotation: Vec3) {
        self.angle_rad += Vec3::new(-rotation.x, -rotation.y, 0f32);
        if self.angle_rad.x > 3.14 / 2.0 {
            self.angle_rad.x = 3.14 / 2.0
        }
        if self.angle_rad.x < -3.14 / 2.0 {
            self.angle_rad.x = -3.14 / 2.0
        }
    }
}
