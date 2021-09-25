use glam::{Mat3A, Vec3A};
use std::f32::consts::PI;

fn main() {
    let origin = Vec3A::new(2.0, 1.0, 1.0);
    let mut matrix = Mat3A::from_angle(PI / 4.0);
    *matrix.col_mut(2) = Vec3A::new(1.0, 2.0, 1.0);

    println!("{}", matrix * origin);
}
