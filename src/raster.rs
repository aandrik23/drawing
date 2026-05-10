use std::io;
use std::io::Write;
use std::path::Path;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

#[derive(Clone, Debug)]
pub struct Image {
    pub width: i32,
    pub height: i32,
    pixels: Vec<u8>,
}

impl Image {
    pub fn blank(width: i32, height: i32) -> Self {
        let width = width.max(0);
        let height = height.max(0);
        let len = (width as usize)
            .saturating_mul(height as usize)
            .saturating_mul(3);

        Self {
            width,
            height,
            pixels: vec![0; len],
        }
    }

    pub fn set_pixel(&mut self, x: i32, y: i32, color: Color) -> Result<(), &'static str> {
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            return Err("pixel coordinates out of bounds");
        }

        let offset = ((y as usize) * (self.width as usize) + (x as usize)) * 3;
        self.pixels[offset] = color.r;
        self.pixels[offset + 1] = color.g;
        self.pixels[offset + 2] = color.b;
        Ok(())
    }
}

pub fn save<P: AsRef<Path>>(image: &Image, path: P) -> io::Result<()> {
    let mut file = std::fs::File::create(path)?;
    let width = image.width as u32;
    let height = image.height as u32;

    file.write_all(b"\x89PNG\r\n\x1a\n")?;

    let mut ihdr = Vec::with_capacity(13);
    ihdr.extend_from_slice(&width.to_be_bytes());
    ihdr.extend_from_slice(&height.to_be_bytes());
    ihdr.push(8);
    ihdr.push(2);
    ihdr.push(0);
    ihdr.push(0);
    ihdr.push(0);
    write_chunk(&mut file, b"IHDR", &ihdr)?;

    let stride = (image.width as usize) * 3;
    let mut raw = Vec::with_capacity((stride + 1) * (image.height as usize));
    for y in 0..(image.height as usize) {
        raw.push(0);
        let start = y * stride;
        raw.extend_from_slice(&image.pixels[start..start + stride]);
    }

    let compressed = zlib_store_compress(&raw);
    write_chunk(&mut file, b"IDAT", &compressed)?;
    write_chunk(&mut file, b"IEND", &[])?;
    Ok(())
}

fn write_chunk<W: Write>(writer: &mut W, chunk_type: &[u8; 4], data: &[u8]) -> io::Result<()> {
    writer.write_all(&(data.len() as u32).to_be_bytes())?;
    writer.write_all(chunk_type)?;
    writer.write_all(data)?;

    let mut crc_input = Vec::with_capacity(chunk_type.len() + data.len());
    crc_input.extend_from_slice(chunk_type);
    crc_input.extend_from_slice(data);
    writer.write_all(&crc32(&crc_input).to_be_bytes())?;
    Ok(())
}

fn zlib_store_compress(data: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(data.len() + (data.len() / 65_535 + 1) * 5 + 6);
    out.push(0x78);
    out.push(0x01);

    let mut offset = 0;
    while offset < data.len() {
        let remaining = data.len() - offset;
        let block_len = remaining.min(65_535);
        let final_block = offset + block_len == data.len();

        out.push(if final_block { 0x01 } else { 0x00 });
        out.extend_from_slice(&(block_len as u16).to_le_bytes());
        out.extend_from_slice((!(block_len as u16)).to_le_bytes().as_slice());
        out.extend_from_slice(&data[offset..offset + block_len]);
        offset += block_len;
    }

    out.extend_from_slice(&adler32(data).to_be_bytes());
    out
}

fn adler32(data: &[u8]) -> u32 {
    const MOD_ADLER: u32 = 65_521;
    let mut a = 1u32;
    let mut b = 0u32;

    for &byte in data {
        a = (a + byte as u32) % MOD_ADLER;
        b = (b + a) % MOD_ADLER;
    }

    (b << 16) | a
}

fn crc32(data: &[u8]) -> u32 {
    let mut crc = 0xffff_ffffu32;

    for &byte in data {
        crc ^= byte as u32;
        for _ in 0..8 {
            let mask = (crc & 1).wrapping_neg() & 0xedb8_8320;
            crc = (crc >> 1) ^ mask;
        }
    }

    !crc
}
