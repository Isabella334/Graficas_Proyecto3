mod framebuffer;
mod triangle;
mod obj_loader;
mod matrix;
mod fragment;
mod vertex;
mod camera;
mod shaders;
mod light;
mod simplex;
mod cellular;

use triangle::triangle;
use obj_loader::Obj;
use framebuffer::Framebuffer;
use raylib::prelude::*;
use std::thread;
use std::time::Duration;
use std::f32::consts::PI;
use matrix::{create_model_matrix_y, create_projection_matrix, create_viewport_matrix};
use vertex::Vertex;
use camera::Camera;
use shaders::{vertex_shader, fragment_shaders};
use light::Light;
use crate::shaders::ShaderType;
use rand::Rng;

pub struct Uniforms {
    pub model_matrix: Matrix,
    pub view_matrix: Matrix,
    pub projection_matrix: Matrix,
    pub viewport_matrix: Matrix,
    pub is_ring: bool,
}

fn draw_stars(framebuffer: &mut Framebuffer, seed: u64) {
    use rand::SeedableRng;
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
    let num_stars = 600;

    for _ in 0..num_stars {
        let x = rng.gen_range(0..framebuffer.width);
        let y = rng.gen_range(0..framebuffer.height);
        let brightness = rng.gen_range(0.5..1.0);

        let size = if rng.gen_bool(0.1) { 2 } else { 1 };

        let star_color = Color::new(
            (255.0 * brightness) as u8,
            (255.0 * brightness) as u8,
            (255.0 * brightness) as u8,
            255,
        );

        framebuffer.color_buffer.draw_pixel(x, y, star_color);

        if size == 2 {
            if x + 1 < framebuffer.width {
                framebuffer.color_buffer.draw_pixel(x + 1, y, star_color);
            }
            if y + 1 < framebuffer.height {
                framebuffer.color_buffer.draw_pixel(x, y + 1, star_color);
            }
        }
    }
}

fn render(
    framebuffer: &mut Framebuffer,
    uniforms: &Uniforms,
    vertex_array: &[Vertex],
    light: &Light,
    shader_type: ShaderType,
    time: f32,
) {
    let mut transformed_vertices = Vec::with_capacity(vertex_array.len());
    for vertex in vertex_array {
        let transformed = vertex_shader(vertex, uniforms);
        transformed_vertices.push(transformed);
    }

    let mut triangles = Vec::new();
    for i in (0..transformed_vertices.len()).step_by(3) {
        if i + 2 < transformed_vertices.len() {
            triangles.push([
                transformed_vertices[i].clone(),
                transformed_vertices[i + 1].clone(),
                transformed_vertices[i + 2].clone(),
            ]);
        }
    }

    let mut fragments = Vec::new();
    for tri in &triangles {
        fragments.extend(triangle(&tri[0], &tri[1], &tri[2], light));
    }

    for fragment in fragments {
        let final_color = fragment_shaders(&fragment, uniforms, shader_type, time);

        framebuffer.point(
            fragment.position.x as i32,
            fragment.position.y as i32,
            fragment.depth,
            final_color,
        );
    }
}

fn main() {
    let window_width = 1300;
    let window_height = 900;

    let (mut window, raylib_thread) = raylib::init()
        .size(window_width, window_height)
        .title("Sistema Solar - Shader Dinámico")
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();

    let mut framebuffer = Framebuffer::new(window_width, window_height);

    let mut camera = Camera::new(
        Vector3::new(0.0, 8.0, 30.0),
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
    );

    let light = Light::new(Vector3::new(0.0, 0.0, 0.0));

    let obj = Obj::load("assets/models/sphere.obj").expect("No se pudo cargar sphere.obj");
    let vertex_array = obj.get_vertex_array();

    let spaceship_obj = Obj::load("assets/models/spaceship.obj").expect("No se pudo cargar spaceship.obj");
    let spaceship_vertices = spaceship_obj.get_vertex_array();

    framebuffer.set_background_color(Color::new(5, 5, 15, 255));

    let mut time: f32 = 0.0;

    while !window.window_should_close() {
        camera.process_input(&window);

        framebuffer.clear();
        framebuffer.set_current_color(Color::new(200, 200, 255, 255));
        draw_stars(&mut framebuffer, 42);

        let view_matrix = camera.get_view_matrix();
        let projection_matrix = create_projection_matrix(
            PI / 3.0,
            window_width as f32 / window_height as f32,
            0.1,
            100.0,
        );
        let viewport_matrix =
            create_viewport_matrix(0.0, 0.0, window_width as f32, window_height as f32);

        time += 0.02;

        let sun_model_matrix = create_model_matrix_y(
            Vector3::new(0.0, 0.0, 0.0),
            1.5,
            time * 0.3,
        );
        let sun_uniforms = Uniforms {
            model_matrix: sun_model_matrix,
            view_matrix: view_matrix,
            projection_matrix: projection_matrix,
            viewport_matrix: viewport_matrix,
            is_ring: false,
        };
        render(&mut framebuffer, &sun_uniforms, &vertex_array, &light, ShaderType::Sun, time);

        let mars_translation = Vector3::new(
            4.5 * (time * 1.5).cos(),
            0.0,
            4.5 * (time * 1.5).sin(),
        );
        let mars_model_matrix = create_model_matrix_y(
            mars_translation,
            0.8,
            time * 3.5,
        );
        let mars_uniforms = Uniforms {
            model_matrix: mars_model_matrix,
            view_matrix: view_matrix,
            projection_matrix: projection_matrix,
            viewport_matrix: viewport_matrix,
            is_ring: false,
        };
        render(&mut framebuffer, &mars_uniforms, &vertex_array, &light, ShaderType::Mars, time);

        let mocca_translation = Vector3::new(
            9.0 * (time * 1.2).cos(),
            0.0,
            9.0 * (time * 1.2).sin(),
        );
        let mocca_model_matrix = create_model_matrix_y(
            mocca_translation,
            0.75,
            time * 3.6,
        );
        let mocca_uniforms = Uniforms {
            model_matrix: mocca_model_matrix,
            view_matrix: view_matrix,
            projection_matrix: projection_matrix,
            viewport_matrix: viewport_matrix,
            is_ring: false,
        };
        render(&mut framebuffer, &mocca_uniforms, &vertex_array, &light, ShaderType::Mocca, time);

        let saturn_translation = Vector3::new(
            10.0 * (time * 3.8).cos(),
            0.0,
            10.0 * (time * 3.8).sin(),
        );
        let saturn_model_matrix = create_model_matrix_y(
            saturn_translation,
            1.1,
            time * 6.0,
        );
        let saturn_uniforms = Uniforms {
            model_matrix: saturn_model_matrix,
            view_matrix: view_matrix,
            projection_matrix: projection_matrix,
            viewport_matrix: viewport_matrix,
            is_ring: false,
        };
        render(&mut framebuffer, &saturn_uniforms, &vertex_array, &light, ShaderType::Saturn, time);

        let saturn_ring_uniforms = Uniforms {
            model_matrix: saturn_model_matrix,
            view_matrix: view_matrix,
            projection_matrix: projection_matrix,
            viewport_matrix: viewport_matrix,
            is_ring: true,
        };
        render(&mut framebuffer, &saturn_ring_uniforms, &vertex_array, &light, ShaderType::SaturnRing, time);

                // ====== Urano ======
        let uranus_translation = Vector3::new(
            17.5 * (time * 2.1).cos(),  // Órbita más grande y lenta que Saturno
            0.0,
            17.5 * (time * 2.1).sin(),
        );
        let uranus_model_matrix = create_model_matrix_y(
            uranus_translation,
            0.85,              // tamaño ligeramente menor que Saturno
            time * 4.2,        // rotación más lenta
        );
        let uranus_uniforms = Uniforms {
            model_matrix: uranus_model_matrix,
            view_matrix,
            projection_matrix,
            viewport_matrix,
            is_ring: false,
        };
        render(&mut framebuffer, &uranus_uniforms, &vertex_array, &light, ShaderType::Uranus, time);

        // ====== Neptuno ======
        let neptune_translation = Vector3::new(
            26.0 * (time * 1.7).cos(),  // Órbita aún más grande y más lenta
            0.0,
            26.0 * (time * 1.7).sin(),
        );
        let neptune_model_matrix = create_model_matrix_y(
            neptune_translation,
            0.82,
            time * 2.8,
        );
        let neptune_uniforms = Uniforms {
            model_matrix: neptune_model_matrix,
            view_matrix,
            projection_matrix,
            viewport_matrix,
            is_ring: false,
        };
        render(&mut framebuffer, &neptune_uniforms, &vertex_array, &light, ShaderType::Neptune, time);

        let orbit_radius = 6.0;
        let orbit_speed = 2.5;
        let elevation = 25.5;
        let spin_speed = 2.0;

        let spaceship_translation = Vector3::new(
            100.0 * (time * 1.7).cos(),  // radio = 6.0, velocidad angular = 2.5
            elevation,
            100.0 * (time * 1.7).sin(),
        );

        let spaceship_model_matrix = create_model_matrix_y(
            spaceship_translation,
            0.08,       // ✅ más pequeña: 0.2 (antes era ~0.8 para planetas)
            time * 2.8,       // ✅ rotación fija = 0.0 (no gira sobre sí misma)
        );

        let spaceship_uniforms = Uniforms {
            model_matrix: spaceship_model_matrix,
            view_matrix,
            projection_matrix,
            viewport_matrix,
            is_ring: false,
        };

        render(&mut framebuffer, &spaceship_uniforms, &spaceship_vertices, &light, ShaderType::Spaceship, time);
        
        framebuffer.swap_buffers(&mut window, &raylib_thread);
        thread::sleep(Duration::from_millis(16));
    }
}