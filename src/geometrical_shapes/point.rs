use rand::Rng;

use crate::geometrical_shapes::{Displayable, Drawable};
use raster::Color;

const POINT_COLOR: Color = Color::rgb(0, 120, 255);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    /// Random point with coordinates in `[0, width)` and `[0, height)`.
    pub fn random(width: i32, height: i32) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            x: rng.gen_range(0..width),
            y: rng.gen_range(0..height),
        }
    }
}

impl Drawable for Point {
    fn draw<D: Displayable + ?Sized>(&self, target: &mut D) {
        target.display(self.x, self.y, self.color());
    }

    fn color(&self) -> Color {
        POINT_COLOR
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometrical_shapes::test_canvas::Canvas;

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
        p.draw(&mut c);
        assert_eq!(c.pixels, vec![(33, 44)]);
    }
}
