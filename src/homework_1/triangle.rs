use glam::{Vec2, Vec3, Vec4};

#[derive(Clone, Copy, Default, Debug)]
pub struct Rgb(u8, u8, u8);

impl From<&Vec3> for Rgb {
    fn from(i: &Vec3) -> Self {
        Self(i.x as u8, i.y as u8, i.z as u8)
    }
}

#[derive(Default)]
pub struct Triangle {
    pub v: [Vec3; 3],
    pub color: [Rgb; 3],
    pub tex_croods: [Vec2; 3],
    pub normal: [Vec3; 3],
}

impl Triangle {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn a(&self) -> Vec3 {
        self.v[0]
    }
    pub fn b(&self) -> Vec3 {
        self.v[1]
    }
    pub fn c(&self) -> Vec3 {
        self.v[2]
    }
    pub fn set_vertex(&mut self, ind: usize, ver: Vec3) {
        self.v[ind] = ver;
    }
    pub fn set_normal(&mut self, ind: usize, n: Vec3) {
        self.normal[ind] = n;
    }
    pub fn set_color(&mut self, ind: usize, r: u8, g: u8, b: u8) {
        self.color[ind] = Rgb(r, g, b);
    }
    pub fn set_tex_crood(&mut self, ind: usize, s: f32, t: f32) {
        self.tex_croods[ind] = Vec2::new(s, t);
    }
    pub fn to_vec4(&self) -> [Vec4; 3] {
        let mut res: [Vec4; 3] = [Vec4::default(); 3];

        for (ind, i) in self.v.iter().enumerate() {
            res[ind] = Vec4::new(i.x, i.y, i.z, 1.0);
        }
        res
    }
}
