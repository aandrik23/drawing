use rand::Rng;
use raster::{Color, Image};

use super::point_line::Point;
use super::Drawable;

#[derive(Clone, Copy, Debug)]
pub struct Circle {
    pub center: Point,
    pub radius: i32,
}

impl Circle {
    pub fn new(center: &Point, radius: i32) -> Self {
        Self {
            center: *center,
            radius,
        }
    }

    pub fn random(width: i32, height: i32) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            center: Point::random(width, height),
            radius: rng.gen_range(1..=20),
        }
    }
}

impl Drawable for Circle {
    fn draw(&self, _image: &mut Image) {}

    fn color(&self) -> Color {
        Color::rgb(255, 0, 255)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Cube {
    pub center: Point,
    pub size: i32,
}

impl Cube {
    pub fn new(center: &Point, size: i32) -> Self {
        Self {
            center: *center,
            size,
        }
    }
}

impl Drawable for Cube {
    fn draw(&self, _image: &mut Image) {}

    fn color(&self) -> Color {
        Color::rgb(0, 255, 255)
    }
}

#[cfg(test)]
mod tests {
    use super::Circle;

    #[test]
    fn circle_random_has_positive_radius() {
        let circle = Circle::random(500, 500);
        assert!(circle.radius > 0);
    }
}
