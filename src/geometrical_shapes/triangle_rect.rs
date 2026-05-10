use raster::{Color, Image};

use super::point_line::Point;
use super::Drawable;

#[derive(Clone, Copy, Debug)]
pub struct Triangle {
    pub a: Point,
    pub b: Point,
    pub c: Point,
}

impl Triangle {
    pub fn new(a: &Point, b: &Point, c: &Point) -> Self {
        Self {
            a: *a,
            b: *b,
            c: *c,
        }
    }
}

impl Drawable for Triangle {
    fn draw(&self, _image: &mut Image) {}

    fn color(&self) -> Color {
        Color::rgb(0, 0, 255)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Rectangle {
    pub first: Point,
    pub second: Point,
}

impl Rectangle {
    pub fn new(first: &Point, second: &Point) -> Self {
        Self {
            first: *first,
            second: *second,
        }
    }
}

impl Drawable for Rectangle {
    fn draw(&self, _image: &mut Image) {}

    fn color(&self) -> Color {
        Color::rgb(255, 255, 0)
    }
}
