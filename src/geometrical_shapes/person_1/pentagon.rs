use std::f64::consts::PI;

use crate::geometrical_shapes::{Displayable, Drawable};
use raster::Color;

use super::bresenham::bresenham;
use super::point::Point;

const PENTAGON_COLOR: Color = Color::rgb(40, 200, 120);

#[derive(Clone, Debug)]
pub struct Pentagon {
    center: Point,
    radius: i32,
}

impl Pentagon {
    pub fn new(center: &Point, radius: i32) -> Self {
        Self {
            center: center.clone(),
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
}

impl Drawable for Pentagon {
    fn draw<D: Displayable + ?Sized>(&self, target: &mut D) {
        let v = self.vertices();
        for i in 0..5 {
            let a = &v[i];
            let b = &v[(i + 1) % 5];
            bresenham(target, a.x, a.y, b.x, b.y, self.color());
        }
    }

    fn color(&self) -> Color {
        PENTAGON_COLOR
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometrical_shapes::person_1::test_canvas::Canvas;

    #[test]
    fn pentagon_draws_without_panic_and_has_edges() {
        let w = 200;
        let h = 200;
        let mut c = Canvas::new(w, h);
        let pent = Pentagon::new(&Point::new(100, 100), 50);
        pent.draw(&mut c);
        assert!(c.pixels.len() > 10);
    }
}
