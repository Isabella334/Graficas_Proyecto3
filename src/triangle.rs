use crate::fragment::Fragment;
use crate::vertex::Vertex;
use crate::Light;
use raylib::prelude::*;

fn barycentric_coordinates(p_x: f32, p_y: f32, a: &Vertex, b: &Vertex, c: &Vertex) -> (f32, f32, f32) {
    let a_x = a.transformed_position.x;
    let b_x = b.transformed_position.x;
    let c_x = c.transformed_position.x;
    let a_y = a.transformed_position.y;
    let b_y = b.transformed_position.y;
    let c_y = c.transformed_position.y;

    let area = (b_y - c_y) * (a_x - c_x) + (c_x - b_x) * (a_y - c_y);

    if area.abs() < 1e-10 {
        return (-1.0, -1.0, -1.0);
    }

    let w1 = ((b_y - c_y) * (p_x - c_x) + (c_x - b_x) * (p_y - c_y)) / area;
    let w2 = ((c_y - a_y) * (p_x - c_x) + (a_x - c_x) * (p_y - c_y)) / area;
    let w3 = 1.0 - w1 - w2;

    (w1, w2, w3)
}

pub fn triangle(v1: &Vertex, v2: &Vertex, v3: &Vertex, light: &Light) -> Vec<Fragment> {
    let mut fragments = Vec::new();

    let min_x = v1.transformed_position.x.min(v2.transformed_position.x).min(v3.transformed_position.x).floor() as i32;
    let max_x = v1.transformed_position.x.max(v2.transformed_position.x).max(v3.transformed_position.x).ceil() as i32;
    let min_y = v1.transformed_position.y.min(v2.transformed_position.y).min(v3.transformed_position.y).floor() as i32;
    let max_y = v1.transformed_position.y.max(v2.transformed_position.y).max(v3.transformed_position.y).ceil() as i32;

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let p_x = x as f32 + 0.5;
            let p_y = y as f32 + 0.5;

            let (w1, w2, w3) = barycentric_coordinates(p_x, p_y, v1, v2, v3);

            if w1 >= 0.0 && w2 >= 0.0 && w3 >= 0.0 {
                let interpolated_normal = Vector3::new(
                    w1 * v1.transformed_normal.x + w2 * v2.transformed_normal.x + w3 * v3.transformed_normal.x,
                    w1 * v1.transformed_normal.y + w2 * v2.transformed_normal.y + w3 * v3.transformed_normal.y,
                    w1 * v1.transformed_normal.z + w2 * v2.transformed_normal.z + w3 * v3.transformed_normal.z,
                );

                let normal_length = (interpolated_normal.x * interpolated_normal.x
                    + interpolated_normal.y * interpolated_normal.y
                    + interpolated_normal.z * interpolated_normal.z).sqrt();
                let mut normalized_normal = interpolated_normal;
                if normal_length > 0.0 {
                    normalized_normal.x /= normal_length;
                    normalized_normal.y /= normal_length;
                    normalized_normal.z /= normal_length;
                }

                let world_pos = Vector3::new(
                    w1 * v1.world_position.x + w2 * v2.world_position.x + w3 * v3.world_position.x,
                    w1 * v1.world_position.y + w2 * v2.world_position.y + w3 * v3.world_position.y,
                    w1 * v1.world_position.z + w2 * v2.world_position.z + w3 * v3.world_position.z,
                );

                let interpolated_color = Vector3::new(
                    w1 * v1.color.x + w2 * v2.color.x + w3 * v3.color.x,
                    w1 * v1.color.y + w2 * v2.color.y + w3 * v3.color.y,
                    w1 * v1.color.z + w2 * v2.color.z + w3 * v3.color.z, 
                );

                // Calcular iluminación
                let mut light_dir = Vector3::new(
                    light.position.x - world_pos.x,
                    light.position.y - world_pos.y,
                    light.position.z - world_pos.z,
                );

                let light_length = (light_dir.x * light_dir.x + light_dir.y * light_dir.y + light_dir.z * light_dir.z).sqrt();
                if light_length > 0.0 {
                    light_dir.x /= light_length;
                    light_dir.y /= light_length;
                    light_dir.z /= light_length;
                }

                let diffuse = (normalized_normal.x * light_dir.x
                    + normalized_normal.y * light_dir.y
                    + normalized_normal.z * light_dir.z).max(0.0);

                let wrap = 0.4;
                let wrapped_diffuse = (diffuse + wrap) / (1.0 + wrap);

                let ambient = 0.2;
                let intensity = ambient + wrapped_diffuse * (1.0 - ambient);

                // Solo .x es iluminación; .y/.z son UVs (se usarán en el shader)
                let final_color = Vector3::new(intensity, interpolated_color.y, interpolated_color.z);

                let depth = w1 * v1.transformed_position.z
                    + w2 * v2.transformed_position.z
                    + w3 * v3.transformed_position.z;

                fragments.push(Fragment::new(p_x, p_y, world_pos, final_color, depth));
            }
        }
    }

    fragments
}
