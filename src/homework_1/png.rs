use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

const MAGIC: [u32; 16] = [
    0, 0x1db71064, 0x3b6e20c8, 0x26d930ac, 0x76dc4190, 0x6b6b51f4, 0x4db26158, 0x5005713c,
    0xedb88320, 0xf00f9344, 0xd6d6a3e8, 0xcb61b38c, 0x9b64c2b0, 0x86d3d2d4, 0xa00ae278, 0xbdbdf21c,
];

const BYTE: [&[u8]; 6] = [
    &[
        137,
        80,
        78,
        71,
        13,
        10,
        26,
        10,
        (13 >> 24) as u8,         // 0
        ((13 >> 16) & 255) as u8, // 0
        ((13 >> 8) & 255) as u8,  // 0
        (13 & 255) as u8,         // 13
    ],
    "IHDR".as_bytes(),
    "\x00\x00\x00".as_bytes(),
    "IDAT".as_bytes(),
    "\x78\x01".as_bytes(),
    "IEND".as_bytes(),
];

pub fn svpng<P: AsRef<Path>>(
    path: P,
    width: u32,
    height: u32,
    data: &[u8],
    alpha: bool,
) -> std::io::Result<()> {
    let mut png = File::create(path)?;
    let mut buf = Vec::<u8>::with_capacity(10);
    let (mut a, mut b, mut c, p) = (
        1u32,
        0u32,
        (!0) as u32,
        width * (if alpha { 4 } else { 3 }) + 1,
    );

    png.write(BYTE[0])?;

    png.write(BYTE[1])?;
    for i in BYTE[1] {
        c ^= *i as u32;
        c = (c >> 4) ^ MAGIC[c as usize & 15];
        c = (c >> 4) ^ MAGIC[c as usize & 15];
    }

    buf.push((width >> 24) as u8);
    buf.push((width >> 16) as u8 & 255);
    buf.push((width >> 8) as u8 & 255);
    buf.push(width as u8 & 255);
    buf.push((height >> 24) as u8);
    buf.push((height >> 16) as u8 & 255);
    buf.push((height >> 8) as u8 & 255);
    buf.push(height as u8 & 255);
    buf.push(8);
    buf.push(if alpha { 6 } else { 2 });
    png.write(buf.as_slice())?;
    for i in &buf {
        c ^= *i as u32;
        c = (c >> 4) ^ MAGIC[c as usize & 15];
        c = (c >> 4) ^ MAGIC[c as usize & 15];
    }
    buf.clear();

    png.write(BYTE[2])?;
    for i in BYTE[2] {
        c ^= *i as u32;
        c = (c >> 4) ^ MAGIC[c as usize & 15];
        c = (c >> 4) ^ MAGIC[c as usize & 15];
    }

    buf.push(((!c) >> 24) as u8);
    buf.push(((!c) >> 16) as u8 & 255);
    buf.push(((!c) >> 8) as u8 & 255);
    buf.push((!c) as u8 & 255);
    buf.push(((2 + height * (5 + p) + 4) >> 24) as u8);
    buf.push(((2 + height * (5 + p) + 4) >> 16) as u8 & 255);
    buf.push(((2 + height * (5 + p) + 4) >> 8) as u8 & 255);
    buf.push((2 + height * (5 + p) + 4) as u8 & 255);
    png.write(buf.as_slice())?;
    buf.clear();

    c = !0;
    png.write(BYTE[3])?;
    for i in BYTE[3] {
        c ^= *i as u32;
        c = (c >> 4) ^ MAGIC[c as usize & 15];
        c = (c >> 4) ^ MAGIC[c as usize & 15];
    }

    png.write(BYTE[4])?;
    for i in BYTE[4] {
        c ^= *i as u32;
        c = (c >> 4) ^ MAGIC[c as usize & 15];
        c = (c >> 4) ^ MAGIC[c as usize & 15];
    }

    for y in 0..height {
        buf.push(if y == height - 1 { 1 } else { 0 });
        buf.push(p as u8 & 255);
        buf.push((p >> 8) as u8 & 255);
        buf.push((!p) as u8 & 255);
        buf.push(((!p) >> 8) as u8 & 255);
        buf.push(0);
        png.write(buf.as_slice())?;
        for i in &buf {
            c ^= *i as u32;
            c = (c >> 4) ^ MAGIC[c as usize & 15];
            c = (c >> 4) ^ MAGIC[c as usize & 15];
        }
        buf.clear();

        a %= 65521;
        b = (b + a) % 65521;

        png.write(data[(y * (p - 1)) as usize..((y + 1) * (p - 1)) as usize].as_ref())?;
        for x in data[(y * (p - 1)) as usize..((y + 1) * (p - 1)) as usize].iter() {
            c ^= *x as u32;
            c = (c >> 4) ^ MAGIC[c as usize & 15];
            c = (c >> 4) ^ MAGIC[c as usize & 15];
            a = (a + *x as u32) % 65521;
            b = (b + a) % 65521;
        }
    }

    buf.push((((b << 16) | a) >> 24) as u8);
    buf.push((((b << 16) | a) >> 16) as u8 & 255);
    buf.push((((b << 16) | a) >> 8) as u8 & 255);
    buf.push(((b << 16) | a) as u8 & 255);
    png.write(buf.as_slice())?;
    for i in &buf {
        c ^= *i as u32;
        c = (c >> 4) ^ MAGIC[c as usize & 15];
        c = (c >> 4) ^ MAGIC[c as usize & 15];
    }
    buf.clear();

    buf.push(((!c) >> 24) as u8);
    buf.push(((!c) >> 16) as u8 & 255);
    buf.push(((!c) >> 8) as u8 & 255);
    buf.push((!c) as u8 & 255);
    buf.push(0);
    buf.push(0);
    buf.push(0);
    buf.push(0);
    png.write(buf.as_slice())?;
    buf.clear();

    c = !0;
    png.write(BYTE[5])?;
    for i in BYTE[5] {
        c ^= *i as u32;
        c = (c >> 4) ^ MAGIC[c as usize & 15];
        c = (c >> 4) ^ MAGIC[c as usize & 15];
    }
    buf.push(((!c) >> 24) as u8);
    buf.push(((!c) >> 16) as u8 & 255);
    buf.push(((!c) >> 8) as u8 & 255);
    buf.push((!c) as u8 & 255);
    png.write(buf.as_slice())?;
    Ok(())
}
