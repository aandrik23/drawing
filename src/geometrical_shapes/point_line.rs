use rand::Rng;
use raster::{Color, Image};

use super::Drawable;

#[derive(Clone, Copy, Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn random(width: i32, height: i32) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            x: rng.gen_range(0..width.max(1)),
            y: rng.gen_range(0..height.max(1)),
        }
    }
}

impl Drawable for Point {
    fn draw(&self, _image: &mut Image) {}

    fn color(&self) -> Color {
        Color::rgb(255, 255, 255)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Line {
    pub start: Point,
    pub end: Point,
}

impl Line {
    pub fn new(start: &Point, end: &Point) -> Self {
        Self {
            start: *start,
            end: *end,
        }
    }

    pub fn random(width: i32, height: i32) -> Self {
        Self {
            start: Point::random(width, height),
            end: Point::random(width, height),
        }
    }
}

impl Drawable for Line {
    fn draw(&self, _image: &mut Image) {}

    fn color(&self) -> Color {
        Color::rgb(255, 0, 0)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Pentagon {
    pub center: Point,
    pub radius: i32,
}

impl Pentagon {
    pub fn new(center: &Point, radius: i32) -> Self {
        Self {
            center: *center,
            radius,
        }
    }
}

impl Drawable for Pentagon {
    fn draw(&self, _image: &mut Image) {}

    fn color(&self) -> Color {
        Color::rgb(0, 255, 0)
    }
}
