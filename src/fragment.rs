use raylib::math::{Vector2, Vector3};

pub struct Fragment {
    pub position: Vector2,
    pub world_position: Vector3,
    pub color: Vector3,
    pub depth: f32,
}

impl Fragment {
    pub fn new(x: f32, y: f32, world_pos: Vector3, color: Vector3, depth: f32) -> Self {
        Fragment {
            position: Vector2::new(x, y),
            world_position: world_pos,
            color,
            depth,
        }
    }
}