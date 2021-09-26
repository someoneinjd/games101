use glam::{IVec3, Mat4, Vec3};

mod rst;
mod triangle;

fn get_model_matrix(rotation_angle: f32) -> Mat4 {
    Mat4::from_rotation_z(rotation_angle.to_radians())
}

#[rustfmt::skip]
fn get_view_matrix(eye_pos: Vec3) -> Mat4 {
    Mat4::from_cols_array(&[
        1.0,        0.0,        0.0,        0.0,
        0.0,        1.0,        0.0,        0.0,
        0.0,        0.0,        1.0,        0.0,
        -eye_pos.x, -eye_pos.y, -eye_pos.z, 1.0,
    ])
}

#[rustfmt::skip]
fn get_projection_matrix(eye_fov: f32, aspect_radio: f32, z_near: f32, z_far: f32) -> Mat4 {
    let top = -(eye_fov / 2.0).to_radians().tan() * z_near.abs();
    let right = top * aspect_radio;
    Mat4::from_cols_array(&[
        z_near / right, 0.0,          0.0,                                      0.0,
        0.0,            z_near / top, 0.0,                                      0.0,
        0.0,            0.0,          (z_near + z_far) / (z_near - z_far),      1.0,
        0.0,            0.0,          -2.0 * z_near * z_far / (z_near - z_far), 0.0,
    ])
}

fn main() -> std::io::Result<()> {
    let mut angle = 0.0f32;
    let mut r = rst::Rasterizer::new(700, 700);
    let eye_pos = Vec3::new(0.0, 0.0, 5.0);
    let pos = vec![
        Vec3::new(2.0, 0.0, -2.0),
        Vec3::new(0.0, 2.0, -2.0),
        Vec3::new(-2.0, 0.0, -2.0),
    ];
    let ind = vec![IVec3::new(0, 1, 2)];
    let pos_id = r.load_positions(pos);
    let ind_id = r.load_indices(ind);

    let mut key = String::new();
    let stdin = std::io::stdin();

    r.set_model(&get_model_matrix(angle));
    r.set_view(&get_view_matrix(eye_pos));

    r.set_projection(&get_projection_matrix(45.0, 1.0, 0.1, 50.0));
    r.draw(pos_id, ind_id, rst::Primitive::Triangle);
    games101::save_image("output.png", r.data(), 700, 700);
    games101::display_image("output.png");

    stdin.read_line(&mut key)?;

    while !key.starts_with("e") {
        angle += if key.starts_with("a") {
            10.0
        } else if key.starts_with("d") {
            -10.0
        } else {
            eprintln!("Unkonwn key");
            0.0
        };
        r.set_model(&get_model_matrix(angle));
        r.set_view(&get_view_matrix(eye_pos));

        r.set_projection(&get_projection_matrix(45.0, 1.0, 0.1, 50.0));
        r.draw(pos_id, ind_id, rst::Primitive::Triangle);
        games101::save_image("output.png", r.data(), 700, 700);
        games101::display_image("output.png");

        key.clear();
        stdin.read_line(&mut key)?;
    }

    Ok(())
}
