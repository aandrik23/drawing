use raster::{Color, Image};

use super::point_line::{Line, Point};
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
    fn draw(&self, image: &mut Image) {
        let color = self.color();
        Line::new(&self.a, &self.b).draw_with_color(image, color.clone());
        Line::new(&self.b, &self.c).draw_with_color(image, color.clone());
        Line::new(&self.c, &self.a).draw_with_color(image, color);
    }

    fn color(&self) -> Color {
        Color::rgb(255, 215, 0)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Rectangle {
    pub first: Point,
    pub second: Point,
}

impl Rectangle {
    pub fn new(first: &Point, second: &Point) -> Self {
        let min_x = first.x.min(second.x);
        let max_x = first.x.max(second.x);
        let min_y = first.y.min(second.y);
        let max_y = first.y.max(second.y);

        Self {
            first: Point::new(min_x, min_y),
            second: Point::new(max_x, max_y),
        }
    }
}

impl Drawable for Rectangle {
    fn draw(&self, image: &mut Image) {
        let top_left = self.first;
        let bottom_right = self.second;
        let top_right = Point::new(bottom_right.x, top_left.y);
        let bottom_left = Point::new(top_left.x, bottom_right.y);
        let color = self.color();

        Line::new(&top_left, &top_right).draw_with_color(image, color.clone());
        Line::new(&top_right, &bottom_right).draw_with_color(image, color.clone());
        Line::new(&bottom_right, &bottom_left).draw_with_color(image, color.clone());
        Line::new(&bottom_left, &top_left).draw_with_color(image, color);
    }

    fn color(&self) -> Color {
        Color::rgb(255, 180, 0)
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
    fn rectangle_new_normalizes_points() {
        let first = Point::new(150, 300);
        let second = Point::new(50, 60);
        let rectangle = Rectangle::new(&first, &second);

        assert_eq!(rectangle.first.x, 50);
        assert_eq!(rectangle.first.y, 60);
        assert_eq!(rectangle.second.x, 150);
        assert_eq!(rectangle.second.y, 300);
    }

    #[test]
    fn rectangle_new_keeps_points_when_already_ordered() {
        let first = Point::new(5, 8);
        let second = Point::new(20, 30);
        let rectangle = Rectangle::new(&first, &second);

        assert_eq!(rectangle.first.x, 5);
        assert_eq!(rectangle.first.y, 8);
        assert_eq!(rectangle.second.x, 20);
        assert_eq!(rectangle.second.y, 30);
    }
}
