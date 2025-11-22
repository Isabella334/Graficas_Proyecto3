use raylib::math::{Vector2, Vector3};

#[derive(Clone)]
pub struct Fragment {
    pub position: Vector2,
    pub world_position: Vector3,
    pub color: Vector3,           // r = iluminación, g = u, b = v
    pub depth: f32,
    pub normal: Vector3,          // ✅ nueva: normal interpolada y normalizada
}

impl Fragment {
    pub fn new(
        x: f32,
        y: f32,
        world_pos: Vector3,
        color: Vector3,
        depth: f32,
        normal: Vector3,          // ✅ añade normal aquí
    ) -> Self {
        Fragment {
            position: Vector2::new(x, y),
            world_position: world_pos,
            color,
            depth,
            normal,
        }
    }
}