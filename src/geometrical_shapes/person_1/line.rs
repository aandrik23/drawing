use crate::geometrical_shapes::{Displayable, Drawable};
use raster::Color;

use super::bresenham::bresenham;
use super::point::Point;

const LINE_COLOR: Color = Color::rgb(255, 60, 60);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Line {
    pub p1: Point,
    pub p2: Point,
}

impl Line {
    pub fn new(a: &Point, b: &Point) -> Self {
        Self {
            p1: a.clone(),
            p2: b.clone(),
        }
    }

    /// Random segment whose endpoints both lie inside `[0, width) × [0, height)`.
    pub fn random(width: i32, height: i32) -> Self {
        Self {
            p1: Point::random(width, height),
            p2: Point::random(width, height),
        }
    }
}

impl Drawable for Line {
    fn draw<D: Displayable + ?Sized>(&self, target: &mut D) {
        bresenham(
            target,
            self.p1.x,
            self.p1.y,
            self.p2.x,
            self.p2.y,
            self.color(),
        );
    }

    fn color(&self) -> Color {
        LINE_COLOR
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometrical_shapes::person_1::test_canvas::Canvas;

    #[test]
    fn random_line_endpoints_stay_inside_dimensions() {
        let w = 800;
        let h = 600;
        for _ in 0..2000 {
            let line = Line::random(w, h);
            assert!(line.p1.x >= 0 && line.p1.x < w && line.p1.y >= 0 && line.p1.y < h);
            assert!(line.p2.x >= 0 && line.p2.x < w && line.p2.y >= 0 && line.p2.y < h);
        }
    }

    #[test]
    fn line_accepts_endpoints_in_either_order_same_pixels() {
        let w = 100;
        let h = 100;
        let mut c1 = Canvas::new(w, h);
        let mut c2 = Canvas::new(w, h);
        let a = Point::new(10, 10);
        let b = Point::new(40, 35);
        Line::new(&a, &b).draw(&mut c1);
        Line::new(&b, &a).draw(&mut c2);
        assert_eq!(c1.sorted_unique(), c2.sorted_unique());
    }

    #[test]
    fn zero_length_line_draws_single_pixel() {
        let w = 50;
        let h = 50;
        let mut c = Canvas::new(w, h);
        let p = Point::new(20, 20);
        Line::new(&p, &p).draw(&mut c);
        assert_eq!(c.pixels.len(), 1);
        assert_eq!(c.pixels[0], (20, 20));
    }
}
