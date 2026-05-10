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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn triangle_new_keeps_points() {
        let a = Point::new(1, 2);
        let b = Point::new(3, 4);
        let c = Point::new(5, 6);
        let triangle = Triangle::new(&a, &b, &c);

        assert_eq!(triangle.a.x, 1);
        assert_eq!(triangle.b.y, 4);
        assert_eq!(triangle.c.x, 5);
    }

    #[test]
    fn rectangle_new_keeps_points() {
        let first = Point::new(5, 8);
        let second = Point::new(20, 30);
        let rectangle = Rectangle::new(&first, &second);

        assert_eq!(rectangle.first.x, 5);
        assert_eq!(rectangle.first.y, 8);
        assert_eq!(rectangle.second.x, 20);
        assert_eq!(rectangle.second.y, 30);
    }
}
