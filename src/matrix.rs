use raylib::prelude::*;

fn dot_product(a: &Vector3, b: &Vector3) -> f32 {
    a.x * b.x + a.y * b.y + a.z * b.z
}

pub fn new_matrix4(
    r0c0: f32, r0c1: f32, r0c2: f32, r0c3: f32,
    r1c0: f32, r1c1: f32, r1c2: f32, r1c3: f32,
    r2c0: f32, r2c1: f32, r2c2: f32, r2c3: f32,
    r3c0: f32, r3c1: f32, r3c2: f32, r3c3: f32,
) -> Matrix {
    Matrix {
        m0: r0c0, m1: r1c0, m2: r2c0, m3: r3c0,
        m4: r0c1, m5: r1c1, m6: r2c1, m7: r3c1,
        m8: r0c2, m9: r1c2, m10: r2c2, m11: r3c2,
        m12: r0c3, m13: r1c3, m14: r2c3, m15: r3c3,
    }
}

pub fn create_model_matrix_y(
    translation: Vector3,
    scale: f32,
    rotation_y: f32,
) -> Matrix {
    let (sin_y, cos_y) = rotation_y.sin_cos();

    let rotation_matrix_y = new_matrix4(
        cos_y,  0.0, sin_y, 0.0,
        0.0,    1.0, 0.0,   0.0,
        -sin_y, 0.0, cos_y, 0.0,
        0.0,    0.0, 0.0,   1.0
    );

    let scale_matrix = new_matrix4(
        scale, 0.0,   0.0,   0.0,
        0.0,   scale, 0.0,   0.0,
        0.0,   0.0,   scale, 0.0,
        0.0,   0.0,   0.0,   1.0
    );

    let translation_matrix = new_matrix4(
        1.0, 0.0, 0.0, translation.x,
        0.0, 1.0, 0.0, translation.y,
        0.0, 0.0, 1.0, translation.z,
        0.0, 0.0, 0.0, 1.0
    );

    translation_matrix * scale_matrix * rotation_matrix_y
}

#[allow(dead_code)]
pub fn create_model_matrix(translation: Vector3, scale: f32, rotation: Vector3) -> Matrix {
    let (sin_x, cos_x) = rotation.x.sin_cos();
    let (sin_y, cos_y) = rotation.y.sin_cos();
    let (sin_z, cos_z) = rotation.z.sin_cos();

    let rx = new_matrix4(
        1.0, 0.0,    0.0,    0.0,
        0.0, cos_x,  -sin_x, 0.0,
        0.0, sin_x,  cos_x,  0.0,
        0.0, 0.0,    0.0,    1.0
    );

    let ry = new_matrix4(
        cos_y,  0.0, sin_y, 0.0,
        0.0,    1.0, 0.0,   0.0,
        -sin_y, 0.0, cos_y, 0.0,
        0.0,    0.0, 0.0,   1.0
    );

    let rz = new_matrix4(
        cos_z, -sin_z, 0.0, 0.0,
        sin_z, cos_z,  0.0, 0.0,
        0.0,   0.0,    1.0, 0.0,
        0.0,   0.0,    0.0, 1.0
    );

    let rotation_matrix = rz * ry * rx;
    let scale_matrix = new_matrix4(scale, 0.0, 0.0, 0.0, 0.0, scale, 0.0, 0.0, 0.0, 0.0, scale, 0.0, 0.0, 0.0, 0.0, 1.0);
    let translation_matrix = new_matrix4(1.0, 0.0, 0.0, translation.x, 0.0, 1.0, 0.0, translation.y, 0.0, 0.0, 1.0, translation.z, 0.0, 0.0, 0.0, 1.0);

    translation_matrix * scale_matrix * rotation_matrix
}

pub fn create_view_matrix(eye: Vector3, target: Vector3, up: Vector3) -> Matrix {
    let mut forward = Vector3::new(target.x - eye.x, target.y - eye.y, target.z - eye.z);
    let len = (forward.x * forward.x + forward.y * forward.y + forward.z * forward.z).sqrt();
    if len > 0.0 {
        forward.x /= len;
        forward.y /= len;
        forward.z /= len;
    }

    let mut right = Vector3::new(
        forward.y * up.z - forward.z * up.y,
        forward.z * up.x - forward.x * up.z,
        forward.x * up.y - forward.y * up.x,
    );
    let len = (right.x * right.x + right.y * right.y + right.z * right.z).sqrt();
    if len > 0.0 {
        right.x /= len;
        right.y /= len;
        right.z /= len;
    }

    let up = Vector3::new(
        right.y * forward.z - right.z * forward.y,
        right.z * forward.x - right.x * forward.z,
        right.x * forward.y - right.y * forward.x,
    );

    new_matrix4(
        right.x, right.y, right.z, -dot_product(&right, &eye),
        up.x, up.y, up.z, -dot_product(&up, &eye),
        -forward.x, -forward.y, -forward.z, dot_product(&forward, &eye),
        0.0, 0.0, 0.0, 1.0,
    )
}

pub fn create_projection_matrix(fov_y: f32, aspect: f32, near: f32, far: f32) -> Matrix {
    let tan_half_fov = (fov_y / 2.0).tan();
    new_matrix4(
        1.0 / (aspect * tan_half_fov), 0.0, 0.0, 0.0,
        0.0, 1.0 / tan_half_fov, 0.0, 0.0,
        0.0, 0.0, -(far + near) / (far - near), -(2.0 * far * near) / (far - near),
        0.0, 0.0, -1.0, 0.0,
    )
}

pub fn create_viewport_matrix(x: f32, y: f32, width: f32, height: f32) -> Matrix {
    let hw = width * 0.5;
    let hh = height * 0.5;
    new_matrix4(
        hw, 0.0, 0.0, x + hw,
        0.0, -hh, 0.0, y + hh,
        0.0, 0.0, 255.0, 0.0,
        0.0, 0.0, 0.0, 1.0,
    )
}