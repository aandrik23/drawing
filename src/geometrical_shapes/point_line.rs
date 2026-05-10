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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_random_stays_in_bounds() {
        let p = Point::random(100, 200);

        assert!((0..100).contains(&p.x));
        assert!((0..200).contains(&p.y));
    }

    #[test]
    fn line_new_keeps_endpoints() {
        let start = Point::new(10, 20);
        let end = Point::new(30, 40);
        let line = Line::new(&start, &end);

        assert_eq!(line.start.x, 10);
        assert_eq!(line.start.y, 20);
        assert_eq!(line.end.x, 30);
        assert_eq!(line.end.y, 40);
    }
}
