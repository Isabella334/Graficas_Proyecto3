use raylib::prelude::*;
use crate::matrix::create_view_matrix;
use std::f32::consts::PI;

pub struct Camera {
    pub eye: Vector3,
    pub target: Vector3,
    pub up: Vector3,
    pub yaw: f32,
    pub pitch: f32,
    pub distance: f32,
    pub rotation_speed: f32,
}

impl Camera {
    pub fn new(eye: Vector3, target: Vector3, up: Vector3) -> Self {
        let direction = eye - target;
        let distance = direction.length();

        if distance == 0.0 {
            panic!("Camera eye and target must not be the same point");
        }

        let pitch = (direction.y / distance).asin();
        let yaw = direction.z.atan2(direction.x);

        Self {
            eye,
            target,
            up,
            yaw,
            pitch,
            distance,
            rotation_speed: 0.05,
        }
    }

    fn update_eye_position(&mut self) {
        self.pitch = self.pitch.clamp(-PI / 2.0 + 0.01, PI / 2.0 - 0.01);
        let cos_pitch = self.pitch.cos();
        self.eye.x = self.target.x + self.distance * cos_pitch * self.yaw.cos();
        self.eye.y = self.target.y + self.distance * self.pitch.sin();
        self.eye.z = self.target.z + self.distance * cos_pitch * self.yaw.sin();
    }

    pub fn get_view_matrix(&self) -> Matrix {
        create_view_matrix(self.eye, self.target, self.up)
    }

    pub fn process_input(&mut self, window: &RaylibHandle) {
        let mut changed = false;

        if window.is_key_down(KeyboardKey::KEY_LEFT) {
            self.yaw += self.rotation_speed;
            changed = true;
        }
        if window.is_key_down(KeyboardKey::KEY_RIGHT) {
            self.yaw -= self.rotation_speed;
            changed = true;
        }
        if window.is_key_down(KeyboardKey::KEY_UP) {
            self.pitch += self.rotation_speed;
            changed = true;
        }
        if window.is_key_down(KeyboardKey::KEY_DOWN) {
            self.pitch -= self.rotation_speed;
            changed = true;
        }

        if changed {
            self.update_eye_position();
        }
    }
}