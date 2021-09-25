use crate::triangle::{Rgb, Triangle};
use glam::{IVec3, Mat4, Vec3, Vec4};
use std::{collections::BTreeMap, ops::RangeFrom};

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Buffers {
    Color,
    Depth,
    All,
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Primitive {
    Line,
    Triangle,
}

#[derive(Clone, Copy)]
pub struct PosBufId(u32);

#[derive(Clone, Copy)]
pub struct IndBufId(u32);

pub struct Rasterizer {
    model: Mat4,
    view: Mat4,
    projection: Mat4,
    pos_buf: BTreeMap<u32, Vec<Vec3>>,
    ind_buf: BTreeMap<u32, Vec<IVec3>>,
    frame_buf: Vec<Rgb>,
    depth_buf: Vec<f32>,
    width: u32,
    height: u32,
    next_id: RangeFrom<u32>,
}

impl Rasterizer {
    pub fn new(width: u32, height: u32) -> Self {
        let mut res = Self {
            model: Mat4::default(),
            view: Mat4::default(),
            projection: Mat4::default(),
            pos_buf: BTreeMap::default(),
            ind_buf: BTreeMap::default(),
            frame_buf: Vec::default(),
            depth_buf: Vec::default(),
            width,
            height,
            next_id: (0..),
        };
        res.frame_buf
            .resize((width * height) as usize, Rgb::default());
        res.depth_buf
            .resize((width * height) as usize, f32::default());
        res
    }

    pub fn load_positions(&mut self, position: Vec<Vec3>) -> PosBufId {
        let id = self.next_id.next().unwrap();
        self.pos_buf.insert(id, position);
        PosBufId(id)
    }

    pub fn load_indices(&mut self, position: Vec<IVec3>) -> IndBufId {
        let id = self.next_id.next().unwrap();
        self.ind_buf.insert(id, position);
        IndBufId(id)
    }

    pub fn set_pixel(&mut self, point: &Vec3, color: &Rgb) {
        if point.x < 0.0
            || point.x >= self.width as f32
            || point.y < 0.0
            || point.y >= self.height as f32
        {
            return;
        } else {
            let ind = (self.height as f32 - 1.0 - point.y) * self.width as f32 + point.x;
            self.frame_buf[ind as usize] = *color;
        }
    }

    pub fn get_index(&self, x: u32, y: u32) -> u32 {
        (self.height - 1 - y) * self.width + x
    }

    pub fn set_model(&mut self, m: &Mat4) {
        self.model = *m;
    }

    pub fn set_view(&mut self, v: &Mat4) {
        self.view = *v;
    }

    pub fn set_projection(&mut self, p: &Mat4) {
        self.projection = *p;
    }

    fn draw_line(&mut self, begin: &Vec3, end: &Vec3, line_color: &Rgb) {
        let x1 = begin.x;
        let y1 = begin.y;
        let x2 = end.x;
        let y2 = end.y;

        let dx = (x2 - x1) as i32;
        let dy = (y2 - y1) as i32;
        let dx1 = dx.abs();
        let dy1 = dy.abs();
        let mut px = 2 * dy1 - dx1;
        let mut py = 2 * dx1 - dy1;

        let (mut x, mut y, xe, ye): (i32, i32, i32, i32);

        if dy1 <= dx1 {
            if dx >= 0 {
                x = x1 as i32;
                y = y1 as i32;
                xe = x2 as i32;
            } else {
                x = x2 as i32;
                y = y2 as i32;
                xe = x1 as i32;
            }

            let mut point = Vec3::new(x as f32, y as f32, 1.0);
            self.set_pixel(&point, &line_color);

            while x < xe {
                x += 1;

                if px < 0 {
                    px += 2 * dy1;
                } else {
                    if (dx < 0 && dy < 0) || (dx > 0 && dy > 0) {
                        y += 1;
                    } else {
                        y -= 1;
                    }
                    px += 2 * (dy1 - dx1);
                }
                point = Vec3::new(x as f32, y as f32, 1.0);
                self.set_pixel(&point, &line_color);
            }
        } else {
            if dy >= 0 {
                x = x1 as i32;
                y = y1 as i32;
                ye = y2 as i32;
            } else {
                x = x2 as i32;
                y = y2 as i32;
                ye = y1 as i32;
            }

            let mut point = Vec3::new(x as f32, y as f32, 1.0);
            self.set_pixel(&point, &line_color);

            while y < ye {
                y += 1;

                if py <= 0 {
                    py += 2 * dx1;
                } else {
                    if (dx < 0 && dy < 0) || (dx > 0 && dy > 0) {
                        x += 1;
                    } else {
                        x -= 1;
                    }
                    py += 2 * (dx1 - dy1);
                }

                point = Vec3::new(x as f32, y as f32, 1.0);
                self.set_pixel(&point, &line_color);
            }
        }
    }

    fn rasterize_wireframe(&mut self, t: &Triangle) {
        self.draw_line(&t.c(), &t.a(), &t.color[0]);
        self.draw_line(&t.c(), &t.b(), &t.color[1]);
        self.draw_line(&t.b(), &t.a(), &t.color[2]);
    }

    pub fn draw(&mut self, pos_buf_id: PosBufId, ind_buf_id: IndBufId, ty: Primitive) {
        if ty != Primitive::Triangle {
            unimplemented!()
        } else {
            let buf = self.pos_buf.get(&pos_buf_id.0).unwrap().clone();
            let ind = self.ind_buf.get(&ind_buf_id.0).unwrap().clone();

            let (f1, f2) = ((100.0 - 0.1) / 2.0, (100.0 + 0.1) / 2.0);

            let mvp = self.projection * self.view * self.model;

            for i in &ind {
                let mut t = Triangle::new();

                let mut v = [
                    mvp * to_vec4(buf[i.x as usize], 1.0),
                    mvp * to_vec4(buf[i.y as usize], 1.0),
                    mvp * to_vec4(buf[i.z as usize], 1.0),
                ];

                for i in v.iter_mut() {
                    *i /= i.w;
                }

                for i in v.iter_mut() {
                    i.x = 0.5 * self.width as f32 * (i.x + 1.0);
                    i.y = 0.5 * self.height as f32 * (i.y + 1.0);
                    i.z = i.z * f1 + f2;
                }

                for (idx, i) in v.iter().enumerate() {
                    t.set_vertex(idx, (*i).into());
                }

                t.set_color(0, 255, 0, 0);
                t.set_color(1, 0, 255, 0);
                t.set_color(2, 0, 0, 255);

                self.rasterize_wireframe(&t);
            }
        }
    }

    pub fn clear(&mut self, buff: Buffers) {
        match buff {
            Buffers::Color => {
                self.frame_buf.fill(Rgb::default());
            }
            Buffers::Depth => {
                self.depth_buf.fill(f32::INFINITY);
            }
            Buffers::All => {
                self.frame_buf.fill(Rgb::default());
                self.depth_buf.fill(f32::INFINITY);
            }
        }
    }

    pub fn data(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(
                std::mem::transmute(self.frame_buf.as_ptr()),
                self.frame_buf.len() * 3,
            )
        }
    }
}

fn to_vec4(v3: Vec3, w: f32) -> Vec4 {
    Vec4::new(v3.x, v3.y, v3.z, w)
}
