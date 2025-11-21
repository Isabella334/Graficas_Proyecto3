use crate::Uniforms;
use crate::fragment::Fragment;
use crate::vertex::Vertex;
use crate::simplex::Simplex;
use crate::cellular::CellularNoise;
use raylib::prelude::*;

#[derive(Debug, Clone, Copy)]
pub enum ShaderType {
    Mars,
    Mocca,
    Sun,
    Saturn,
    SaturnRing,
}

fn multiply_matrix_vector4(matrix: &Matrix, vector: &Vector4) -> Vector4 {
    Vector4::new(
        matrix.m0 * vector.x + matrix.m4 * vector.y + matrix.m8 * vector.z + matrix.m12 * vector.w,
        matrix.m1 * vector.x + matrix.m5 * vector.y + matrix.m9 * vector.z + matrix.m13 * vector.w,
        matrix.m2 * vector.x + matrix.m6 * vector.y + matrix.m10 * vector.z + matrix.m14 * vector.w,
        matrix.m3 * vector.x + matrix.m7 * vector.y + matrix.m11 * vector.z + matrix.m15 * vector.w,
    )
}

fn transform_normal(normal: &Vector3, model_matrix: &Matrix) -> Vector3 {
    let normal_vec4 = Vector4::new(normal.x, normal.y, normal.z, 0.0);
    let transformed_normal_vec4 = multiply_matrix_vector4(model_matrix, &normal_vec4);

    let mut transformed_normal = Vector3::new(
        transformed_normal_vec4.x,
        transformed_normal_vec4.y,
        transformed_normal_vec4.z,
    );

    let length = (transformed_normal.x * transformed_normal.x
        + transformed_normal.y * transformed_normal.y
        + transformed_normal.z * transformed_normal.z)
        .sqrt();

    if length > 0.0 {
        transformed_normal.x /= length;
        transformed_normal.y /= length;
        transformed_normal.z /= length;
    }

    transformed_normal
}

pub fn vertex_shader(vertex: &Vertex, uniforms: &Uniforms) -> Vertex {
    let mut position = vertex.position;
    if uniforms.is_ring {
        let radius = (position.x * position.x + position.y * position.y + position.z * position.z).sqrt();
        let nx = if radius > 0.0 { position.x / radius } else { 0.0 };
        let ny = if radius > 0.0 { position.y / radius } else { 0.0 };
        let nz = if radius > 0.0 { position.z / radius } else { 0.0 };

        let longitude = nx.atan2(nz);
        let latitude = ny.asin();

        let inner_radius = 1.3;
        let outer_radius = 2.3;

        let t = (latitude + std::f32::consts::PI / 2.0) / std::f32::consts::PI;
        let t = t.clamp(0.0, 1.0);

        let ring_radius = inner_radius + t * (outer_radius - inner_radius);

        let x = ring_radius * longitude.cos();
        let z = ring_radius * longitude.sin();
        let y = 0.0;

        position = Vector3::new(x, y, z);
    }

    let position_vec4 = Vector4::new(position.x, position.y, position.z, 1.0);
    let world_position_vec4 = multiply_matrix_vector4(&uniforms.model_matrix, &position_vec4);
    let world_position = Vector3::new(
        world_position_vec4.x,
        world_position_vec4.y,
        world_position_vec4.z,
    );

    let view_position = multiply_matrix_vector4(&uniforms.view_matrix, &world_position_vec4);
    let clip_position = multiply_matrix_vector4(&uniforms.projection_matrix, &view_position);

    let ndc = if clip_position.w != 0.0 {
        Vector3::new(
            clip_position.x / clip_position.w,
            clip_position.y / clip_position.w,
            clip_position.z / clip_position.w,
        )
    } else {
        Vector3::new(clip_position.x, clip_position.y, clip_position.z)
    };

    let ndc_vec4 = Vector4::new(ndc.x, ndc.y, ndc.z, 1.0);
    let screen_position = multiply_matrix_vector4(&uniforms.viewport_matrix, &ndc_vec4);

    let transformed_position = Vector3::new(screen_position.x, screen_position.y, screen_position.z);

    let transformed_normal = if uniforms.is_ring {
        Vector3::new(0.0, 1.0, 0.0)
    } else {
        transform_normal(&vertex.normal, &uniforms.model_matrix)
    };

    let local_pos = vertex.position;
    let r_sq = local_pos.x * local_pos.x + local_pos.y * local_pos.y + local_pos.z * local_pos.z;
    let (u, v) = if r_sq > 1e-6 {
        let r = r_sq.sqrt();
        let nx = local_pos.x / r;
        let ny = local_pos.y / r;
        let nz = local_pos.z / r;
        let latitude = ny.asin();
        let longitude = nx.atan2(nz);
        (
            (longitude + std::f32::consts::PI) / (2.0 * std::f32::consts::PI),
            (latitude + std::f32::consts::PI / 2.0) / std::f32::consts::PI,
        )
    } else {
        (0.0, 0.0)
    };

    let mut color = vertex.color;
    color.y = u;
    color.z = v;

    Vertex {
        position: vertex.position,
        normal: vertex.normal,
        tex_coords: vertex.tex_coords,
        color,
        transformed_position,
        transformed_normal,
        world_position,
    }
}

fn smoothstep(edge0: f32, edge1: f32, x: f32) -> f32 {
    let t = ((x - edge0) / (edge1 - edge0)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

fn hash21(p: Vector2) -> f32 {
    let p = Vector2::new(
        p.x.sin() * 127.1 + p.y.cos() * 311.7,
        p.x.cos() * 269.5 + p.y.sin() * 183.3,
    );
    (p.x.sin() * 43758.5453).fract()
}

fn noise2(p: Vector2) -> f32 {
    let i = Vector2::new(p.x.floor(), p.y.floor());
    let f = Vector2::new(p.x.fract(), p.y.fract());
    let u = Vector2::new(
        3.0 * f.x * f.x - 2.0 * f.x * f.x * f.x,
        3.0 * f.y * f.y - 2.0 * f.y * f.y * f.y,
    );

    let a = hash21(i);
    let b = hash21(i + Vector2::new(1.0, 0.0));
    let c = hash21(i + Vector2::new(0.0, 1.0));
    let d = hash21(i + Vector2::new(1.0, 1.0));

    let mix_ab = a + u.x * (b - a);
    let mix_cd = c + u.x * (d - c);
    mix_ab + u.y * (mix_cd - mix_ab)
}

fn fbm2(p: Vector2, octaves: usize) -> f32 {
    let mut value = 0.0;
    let mut amplitude = 1.0;
    let mut frequency = 1.0;
    let p = p;

    for _ in 0..octaves {
        value += amplitude * noise2(p * frequency);
        amplitude *= 0.5;
        frequency *= 2.0;
    }
    value
}

pub fn mars_shader(fragment: &Fragment) -> Vector3 {
    let desert      = Vector3::new(1.58, 0.29, 0.15);
    let volcanic    = Vector3::new(2.49, 0.10, 0.10);
    let dust        = Vector3::new(2.43, 1.26, 0.24);
    let ice         = Vector3::new(0.91, 0.90, 0.88);

    let u = fragment.color.y;
    let v = fragment.color.z;
    let uv = Vector2::new(u, v);

    let base = fbm2(uv * 2.5, 3);
    let red_spots = fbm2(uv * 4.0 + Vector2::new(10.0, 20.0), 2);
    let dark_zones = fbm2(uv * 8.0 + Vector2::new(30.0, 40.0), 2);
    let polar_blend = smoothstep(0.85, 1.0, v) + smoothstep(0.0, 0.15, v);

    let mut color = if base > 0.6 {
        dust
    } else if base > 0.35 {
        desert
    } else {
        desert * 0.9
    };

    if red_spots > 0.85 { color = volcanic; }
    if dark_zones < 0.15 { color = volcanic * 0.85; }

    let syrtis_center = Vector2::new(0.3, 0.5);
    let dist = (uv.x - syrtis_center.x).hypot(uv.y - syrtis_center.y);
    let syrtis = smoothstep(0.12, 0.04, dist);
    if syrtis > 0.7 {
        color = volcanic * 1.1;
    }

    if polar_blend > 0.1 {
        color = color * (1.0 - polar_blend * 0.5) + ice * polar_blend * 0.5;
    }

    color
}

pub fn saturn_shader(fragment: &Fragment) -> Vector3 {
    let u = fragment.color.y;
    let v = fragment.color.z;
    let uv = Vector2::new(u, v);

    let layer1 = Vector3::new(0.96, 0.92, 0.82);
    let layer2 = Vector3::new(0.92, 0.85, 0.70);
    let layer3 = Vector3::new(0.65, 0.52, 0.38);

    let detail = fbm2(uv * Vector2::new(2.0, 20.0), 2) * 0.03;

    let mut color = layer1;

    if v > 0.4 && v < 0.6 { color = layer3; }
    if v > 0.2 && v < 0.3 { color = layer2; }
    if v > 0.7 && v < 0.8 { color = layer2; }
    if v < 0.15 || v > 0.85 { color = layer3 * 0.8; }

    color += Vector3::new(detail, detail * 0.9, detail * 0.7);

    let equator_blend = 1.0 - (v - 0.5).abs() * 2.0;
    color *= 1.0 + equator_blend * 0.15;

    color
}

pub fn saturn_ring_shader(fragment: &Fragment) -> Vector3 {
    let x = fragment.world_position.x;
    let z = fragment.world_position.z;
    let radius = (x * x + z * z).sqrt();

    let inner = 1.3;
    let outer = 2.3;
    let t = ((radius - inner) / (outer - inner)).clamp(0.0, 1.0);

    let color = if t < 0.3 {
        Vector3::new(0.88, 0.80, 0.65)
    } else if t < 0.7 {
        Vector3::new(0.75, 0.65, 0.50)
    } else {
        Vector3::new(0.95, 0.90, 0.80) 
    };
    if (t - 0.5).abs() < 0.05 {
        return color * 0.4;
    }
    let angle = x.atan2(z);
    let noise = ((angle * 10.0).sin() * 0.5 + 0.5) * 0.03;
    (color + Vector3::new(noise, noise * 0.8, noise * 0.6)) * 0.7
}

pub fn mocca_shader(fragment: &Fragment) -> Vector3 {
    let u = fragment.color.y;
    let v = fragment.color.z;
    let uv = Vector2::new(u, v);

    let layer1 = Vector3::new(0.86, 0.70, 0.55);
    let mid = fbm2(uv * 4.0 + Vector2::new(5.0, 10.0), 2);
    let layer2 = Vector3::new(0.55, 0.36, 0.22);
    let dark = fbm2(uv * 8.0 + Vector2::new(15.0, 25.0), 2);
    let layer3 = Vector3::new(0.35, 0.22, 0.12);
    let foam = fbm2(uv * 16.0 + Vector2::new(100.0, 200.0), 2);
    let layer4 = Vector3::new(0.95, 0.92, 0.88);

    let mut color = layer1;
    if mid > 0.6 { color = layer2; }
    if dark < 0.3 { color = layer3; }
    if foam > 0.85 { color = color * 0.7 + layer4 * 0.3; }

    let star = (
        (uv.x * 6.0).sin() * (uv.y * 4.0).cos() +
        (uv.x * 3.0 + 1.0).cos() * (uv.y * 5.0 + 2.0).sin()
    ).abs() * 0.3;
    if star > 0.7 {
        color = Vector3::new(0.95, 0.90, 0.80);
    }

    color
}

pub fn sun_shader(fragment: &Fragment, time: f32) -> Vector3 {
    let simplex = Simplex::new();
    let cellular = CellularNoise::new();
    
    let u = fragment.color.y;
    let v = fragment.color.z;
    
    let cell_scale = 8.0;
    let cell_speed = 0.3;
    let cells = cellular.cellular2d(
        u * cell_scale + time * cell_speed,
        v * cell_scale + time * cell_speed * 0.7
    );
    
    let cell_edges = cellular.cellular2d_f2_f1(
        u * cell_scale * 0.5 - time * cell_speed * 0.4,
        v * cell_scale * 0.5 + time * cell_speed * 0.3
    );
    
    let surface_scale = 6.0;
    let surface_speed = 0.4;
    let surface = simplex.fbm(
        u * surface_scale + time * surface_speed,
        v * surface_scale + time * surface_speed * 0.6,
        3,
        2.0,
        0.5
    );
    
    let turbulence_scale = 10.0;
    let turbulence_speed = 0.5;
    let turbulence = simplex.fbm(
        u * turbulence_scale - time * turbulence_speed * 0.6,
        v * turbulence_scale + time * turbulence_speed * 0.8,
        4,
        2.2,
        0.45
    );
    
    let wave_scale = 2.5;
    let wave_speed = 0.25;
    let waves = simplex.noise2d(
        u * wave_scale + time * wave_speed,
        v * wave_scale - time * wave_speed * 1.2
    );
    
    let base_yellow = Vector3::new(1.0, 0.88, 0.25);
    let bright_yellow = Vector3::new(1.0, 0.95, 0.40);
    let warm_yellow = Vector3::new(1.0, 0.80, 0.18);
    let golden = Vector3::new(0.95, 0.70, 0.12);
    let hot_spot = Vector3::new(1.0, 0.98, 0.55);
    
    let cell_norm = (1.0 - cells).clamp(0.0, 1.0);
    let mut color = base_yellow;
    
    if cell_norm > 0.7 {
        let intensity = smoothstep(0.7, 0.9, cell_norm);
        color = color * (1.0 - intensity * 0.6) + hot_spot * intensity * 0.6;
    }
    
    let edge_norm = (cell_edges * 2.0).clamp(0.0, 1.0);
    if edge_norm > 0.4 {
        let intensity = smoothstep(0.4, 0.7, edge_norm);
        color = color * (1.0 - intensity * 0.4) + golden * intensity * 0.4;
    }
    
    let surface_norm = surface * 0.5 + 0.5;
    color = color * (0.8 + surface_norm * 0.4);
    
    let turb_norm = turbulence * 0.5 + 0.5;
    if turb_norm > 0.6 {
        let blend = smoothstep(0.6, 0.8, turb_norm) * 0.5;
        color = color * (1.0 - blend) + bright_yellow * blend;
    } else if turb_norm < 0.4 {
        let blend = smoothstep(0.4, 0.2, turb_norm) * 0.4;
        color = color * (1.0 - blend) + warm_yellow * blend;
    }
    
    let wave_norm = waves * 0.5 + 0.5;
    let wave_intensity = wave_norm * 0.15;
    color = color * (1.0 + wave_intensity);
    
    let global_pulse = ((time * 0.8).sin() * 0.5 + 0.5) * 0.25;
    color = color * (1.0 + global_pulse);
    
    let cell_pulse = ((time * 1.5 + cell_norm * 3.14).sin() * 0.5 + 0.5) * 0.2;
    color = color * (1.0 + cell_pulse);
    
    color.x = color.x.clamp(0.0, 1.0);
    color.y = color.y.clamp(0.0, 1.0);
    color.z = color.z.clamp(0.0, 1.0);
    
    color
}

pub fn fragment_shaders(
    fragment: &Fragment,
    _uniforms: &Uniforms,
    shader_type: ShaderType,
    time: f32,
) -> Vector3 {
    let base_color = match shader_type {
        ShaderType::Mars => mars_shader(fragment),
        ShaderType::Mocca => mocca_shader(fragment),
        ShaderType::Sun => sun_shader(fragment, time),
        ShaderType::Saturn => saturn_shader(fragment),
        ShaderType::SaturnRing => saturn_ring_shader(fragment),
    };

    if matches!(shader_type, ShaderType::Sun) {
        return base_color;
    }

    base_color * fragment.color.x
}
