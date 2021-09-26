use image::{save_buffer, ColorType};
use std::path::Path;
use std::process::Command;

pub fn save_image<P: AsRef<Path>>(path: P, data: &[u8], width: u32, height: u32) {
    save_buffer(path, data, width, height, ColorType::Rgb8).unwrap();
}

pub fn display_image(path: &str) {
    Command::new("kitty")
        .args(["+kitten", "icat", path])
        .output()
        .expect("Display Failed");
}
