use std::f64::consts::PI;

use rand::Rng;
use crate::raster::{Color, Image};

use super::bresenham::bresenham;
use super::{Displayable, Drawable};

#[inline]
fn point_color() -> Color {
    Color::rgb(0, 120, 255)
}

#[inline]
fn line_color() -> Color {
    Color::rgb(255, 60, 60)
}

#[inline]
fn pentagon_color() -> Color {
    Color::rgb(40, 200, 120)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
        let w = width.max(1);
        let h = height.max(1);
        Self {
            x: rng.gen_range(0..w),
            y: rng.gen_range(0..h),
        }
    }

    pub fn draw_to<D: Displayable + ?Sized>(&self, target: &mut D) {
        target.display(self.x, self.y, point_color());
    }
}

impl Drawable for Point {
    fn draw(&self, image: &mut Image) {
        self.draw_to(image);
    }

    fn color(&self) -> Color {
        point_color()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

    pub fn draw_to<D: Displayable + ?Sized>(&self, target: &mut D) {
        bresenham(
            target,
            self.start.x,
            self.start.y,
            self.end.x,
            self.end.y,
            line_color(),
        );
    }
}

impl Drawable for Line {
    fn draw(&self, image: &mut Image) {
        self.draw_to(image);
    }

    fn color(&self) -> Color {
        line_color()
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

    fn vertices(&self) -> [Point; 5] {
        let cx = self.center.x as f64;
        let cy = self.center.y as f64;
        let r = self.radius as f64;
        let mut out = [Point::new(0, 0); 5];
        for i in 0..5 {
            let angle = -PI / 2.0 + (2.0 * PI * i as f64 / 5.0);
            let x = cx + r * angle.cos();
            let y = cy + r * angle.sin();
            out[i] = Point::new(x.round() as i32, y.round() as i32);
        }
        out
    }

    pub fn draw_to<D: Displayable + ?Sized>(&self, target: &mut D) {
        let v = self.vertices();
        for i in 0..5 {
            let a = &v[i];
            let b = &v[(i + 1) % 5];
            bresenham(target, a.x, a.y, b.x, b.y, pentagon_color());
        }
    }
}

impl Drawable for Pentagon {
    fn draw(&self, image: &mut Image) {
        self.draw_to(image);
    }

    fn color(&self) -> Color {
        pentagon_color()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometrical_shapes::test_canvas::Canvas;

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

    #[test]
    fn random_point_stays_inside_dimensions() {
        let w = 640;
        let h = 480;
        for _ in 0..2000 {
            let p = Point::random(w, h);
            assert!(p.x >= 0 && p.x < w && p.y >= 0 && p.y < h);
        }
    }

    #[test]
    fn draw_writes_single_pixel_at_coordinates() {
        let mut c = Canvas::new(100, 100);
        let p = Point::new(33, 44);
        p.draw_to(&mut c);
        assert_eq!(c.pixels, vec![(33, 44)]);
    }

    #[test]
    fn random_line_endpoints_stay_inside_dimensions() {
        let w = 800;
        let h = 600;
        for _ in 0..2000 {
            let line = Line::random(w, h);
            assert!(line.start.x >= 0 && line.start.x < w && line.start.y >= 0 && line.start.y < h);
            assert!(line.end.x >= 0 && line.end.x < w && line.end.y >= 0 && line.end.y < h);
        }
    }

    #[test]
    fn line_segment_includes_both_endpoints() {
        let w = 100;
        let h = 100;
        let mut c = Canvas::new(w, h);
        let a = Point::new(10, 10);
        let b = Point::new(40, 35);
        Line::new(&a, &b).draw_to(&mut c);
        assert!(c.pixels.contains(&(10, 10)));
        assert!(c.pixels.contains(&(40, 35)));
        assert!(c.pixels.len() >= 2);
    }

    #[test]
    fn zero_length_line_draws_single_pixel() {
        let w = 50;
        let h = 50;
        let mut c = Canvas::new(w, h);
        let p = Point::new(20, 20);
        Line::new(&p, &p).draw_to(&mut c);
        assert_eq!(c.pixels.len(), 1);
        assert_eq!(c.pixels[0], (20, 20));
    }

    #[test]
    fn pentagon_draws_without_panic_and_has_edges() {
        let w = 200;
        let h = 200;
        let mut c = Canvas::new(w, h);
        let pent = Pentagon::new(&Point::new(100, 100), 50);
        pent.draw_to(&mut c);
        assert!(c.pixels.len() > 10);
    }
}
