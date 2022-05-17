#[macro_use]
extern crate glium;

use glium::index::PrimitiveType;
use glium::{glutin, Surface};
use glutin::event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use glutin::event_loop::ControlFlow;
use support::save_image;

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

fn main() {
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

    r.set_model(&get_model_matrix(angle));
    r.set_view(&get_view_matrix(eye_pos));

    r.set_projection(&get_projection_matrix(45.0, 1.0, 0.1, 50.0));
    r.draw(pos_id, ind_id, rst::Primitive::Triangle);

    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new().with_vsync(true);
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let vertex_buffer = {
        #[derive(Copy, Clone)]
        struct Vertex {
            position: [f32; 2],
            tex_coords: [f32; 2],
        }

        implement_vertex!(Vertex, position, tex_coords);

        glium::VertexBuffer::new(
            &display,
            &[
                Vertex {
                    position: [-1.0, -1.0],
                    tex_coords: [0.0, 0.0],
                },
                Vertex {
                    position: [-1.0, 1.0],
                    tex_coords: [0.0, 1.0],
                },
                Vertex {
                    position: [1.0, 1.0],
                    tex_coords: [1.0, 1.0],
                },
                Vertex {
                    position: [1.0, -1.0],
                    tex_coords: [1.0, 0.0],
                },
            ],
        )
        .unwrap()
    };

    // building the index buffer
    let index_buffer =
        glium::IndexBuffer::new(&display, PrimitiveType::TriangleStrip, &[1_u16, 2, 0, 3])
            .unwrap();

    // compiling shaders and linking them together
    let program = program!(&display,
        140 => {
            vertex: "
                #version 140
                uniform mat4 matrix;
                in vec2 position;
                in vec2 tex_coords;
                out vec2 v_tex_coords;
                void main() {
                    gl_Position = matrix * vec4(position, 0.0, 1.0);
                    v_tex_coords = tex_coords;
                }
            ",

            fragment: "
                #version 140
                uniform sampler2D tex;
                in vec2 v_tex_coords;
                out vec4 f_color;
                void main() {
                    f_color = texture(tex, v_tex_coords);
                }
            "
        },

        110 => {
            vertex: "
                #version 110
                uniform mat4 matrix;
                attribute vec2 position;
                attribute vec2 tex_coords;
                varying vec2 v_tex_coords;
                void main() {
                    gl_Position = matrix * vec4(position, 0.0, 1.0);
                    v_tex_coords = tex_coords;
                }
            ",

            fragment: "
                #version 110
                uniform sampler2D tex;
                varying vec2 v_tex_coords;
                void main() {
                    gl_FragColor = texture2D(tex, v_tex_coords);
                }
            ",
        },

        100 => {
            vertex: "
                #version 100
                uniform lowp mat4 matrix;
                attribute lowp vec2 position;
                attribute lowp vec2 tex_coords;
                varying lowp vec2 v_tex_coords;
                void main() {
                    gl_Position = matrix * vec4(position, 0.0, 1.0);
                    v_tex_coords = tex_coords;
                }
            ",

            fragment: "
                #version 100
                uniform lowp sampler2D tex;
                varying lowp vec2 v_tex_coords;
                void main() {
                    gl_FragColor = texture2D(tex, v_tex_coords);
                }
            ",
        },
    )
    .unwrap();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        let image = glium::texture::RawImage2d::from_raw_rgb(r.data().into(), (700, 700));
        let opengl_texture = glium::texture::CompressedSrgbTexture2d::new(&display, image).unwrap();
        let uniforms = uniform! {
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0f32]
            ],
            tex: &opengl_texture
        };
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 0.0);
        target
            .draw(
                &vertex_buffer,
                &index_buffer,
                &program,
                &uniforms,
                &Default::default(),
            )
            .unwrap();
        target.finish().unwrap();
        if let Event::WindowEvent { event, .. } = event {
            match event {
                WindowEvent::CloseRequested => {
                    save_image("output.png", r.data(), 700, 700);
                    *control_flow = ControlFlow::Exit;
                }
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            virtual_keycode: Some(virtual_code),
                            state,
                            ..
                        },
                    ..
                } => match (virtual_code, state) {
                    (VirtualKeyCode::Left, ElementState::Pressed) => angle += 10.0,
                    (VirtualKeyCode::Right, ElementState::Pressed) => angle -= 10.0,
                    _ => (),
                },
                _ => (),
            }
        }
        r.set_model(&get_model_matrix(angle));
        r.set_view(&get_view_matrix(eye_pos));
        r.set_projection(&get_projection_matrix(45.0, 1.0, 0.1, 50.0));
        r.draw(pos_id, ind_id, rst::Primitive::Triangle);
    });
}
