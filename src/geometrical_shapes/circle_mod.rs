use rand::Rng;
use raster::{Color, Image};

use super::point_line::Point;
use super::{Displayable, Drawable};

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
        let w = width.max(1);
        let h = height.max(1);
        let max_centered_radius = ((w.min(h) - 1) / 2).max(0);
        let preferred_max_radius = (w.min(h) / 8).max(1);
        let max_radius = preferred_max_radius.min(max_centered_radius);

        let radius = if max_radius == 0 {
            0
        } else {
            rng.gen_range(1..=max_radius)
        };

        let min_x = radius;
        let max_x = (w - 1 - radius).max(min_x);
        let min_y = radius;
        let max_y = (h - 1 - radius).max(min_y);

        let x = rng.gen_range(min_x..=max_x);
        let y = rng.gen_range(min_y..=max_y);

        Self::new(&Point::new(x, y), radius)
    }

    #[cfg(test)]
    pub fn is_inside_bounds(&self, width: i32, height: i32) -> bool {
        self.center.x - self.radius >= 0
            && self.center.y - self.radius >= 0
            && self.center.x + self.radius < width
            && self.center.y + self.radius < height
    }

    pub fn draw_to<D: Displayable + ?Sized>(&self, target: &mut D) {
        let mut x = self.radius;
        let mut y = 0;
        let mut error = 0;
        let cx = self.center.x;
        let cy = self.center.y;
        let color = self.color();

        while x >= y {
            target.display(cx + x, cy + y, color.clone());
            target.display(cx + y, cy + x, color.clone());
            target.display(cx - y, cy + x, color.clone());
            target.display(cx - x, cy + y, color.clone());
            target.display(cx - x, cy - y, color.clone());
            target.display(cx - y, cy - x, color.clone());
            target.display(cx + y, cy - x, color.clone());
            target.display(cx + x, cy - y, color.clone());

            y += 1;

            if error <= 0 {
                error += 2 * y + 1;
            }

            if error > 0 {
                x -= 1;
                error -= 2 * x + 1;
            }
        }
    }
}

impl Drawable for Circle {
    fn draw(&self, image: &mut Image) {
        self.draw_to(image);
    }

    fn color(&self) -> Color {
        Color::rgb(255, 0, 255)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Cube {
    pub top_left: Point,
    pub size: i32,
    pub depth: i32,
}

impl Cube {
    pub fn new(top_left: &Point, size: i32) -> Self {
        Self {
            top_left: *top_left,
            size,
            depth: size / 3,
        }
    }

    pub fn random(width: i32, height: i32) -> Self {
        let mut rng = rand::thread_rng();
        let w = width.max(1);
        let h = height.max(1);
        let preferred_max_size = (w.min(h) / 4).max(1);
        let mut fitted_max_size = 0;

        for candidate in 1..=preferred_max_size {
            let depth = candidate / 3;
            if candidate + depth <= w - 1 && candidate <= h - 1 {
                fitted_max_size = candidate;
            }
        }

        let size = if fitted_max_size == 0 {
            0
        } else {
            rng.gen_range(1..=fitted_max_size)
        };
        let depth = size / 3;

        let max_x = (w - 1 - size - depth).max(0);
        let min_y = depth.min(h - 1);
        let max_y = (h - 1 - size).max(min_y);
        let x = rng.gen_range(0..=max_x);
        let y = rng.gen_range(min_y..=max_y);

        Self::new(&Point::new(x, y), size)
    }

    pub fn vertices(&self) -> [Point; 8] {
        let x = self.top_left.x;
        let y = self.top_left.y;
        let s = self.size;
        let d = self.depth;

        [
            Point::new(x, y),
            Point::new(x + s, y),
            Point::new(x, y + s),
            Point::new(x + s, y + s),
            Point::new(x + d, y - d),
            Point::new(x + s + d, y - d),
            Point::new(x + d, y + s - d),
            Point::new(x + s + d, y + s - d),
        ]
    }

    #[cfg(test)]
    pub fn is_inside_bounds(&self, width: i32, height: i32) -> bool {
        self.vertices()
            .iter()
            .all(|p| p.x >= 0 && p.x < width && p.y >= 0 && p.y < height)
    }

    fn draw_line(&self, image: &mut Image, p1: Point, p2: Point) {
        let mut x0 = p1.x;
        let mut y0 = p1.y;
        let x1 = p2.x;
        let y1 = p2.y;

        let dx = (x1 - x0).abs();
        let dy = -(y1 - y0).abs();

        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };

        let mut error = dx + dy;

        loop {
            image.display(x0, y0, self.color());

            if x0 == x1 && y0 == y1 {
                break;
            }

            let e2 = 2 * error;

            if e2 >= dy {
                error += dy;
                x0 += sx;
            }

            if e2 <= dx {
                error += dx;
                y0 += sy;
            }
        }
    }
}

impl Drawable for Cube {
    fn draw(&self, image: &mut Image) {
        let [ftl, ftr, fbl, fbr, btl, btr, bbl, bbr] = self.vertices();

        self.draw_line(image, ftl, ftr);
        self.draw_line(image, ftr, fbr);
        self.draw_line(image, fbr, fbl);
        self.draw_line(image, fbl, ftl);

        self.draw_line(image, btl, btr);
        self.draw_line(image, btr, bbr);
        self.draw_line(image, bbr, bbl);
        self.draw_line(image, bbl, btl);

        self.draw_line(image, ftl, btl);
        self.draw_line(image, ftr, btr);
        self.draw_line(image, fbl, bbl);
        self.draw_line(image, fbr, bbr);
    }

    fn color(&self) -> Color {
        Color::rgb(0, 255, 255)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn random_circles_stay_inside_large_canvas() {
        for _ in 0..1000 {
            let circle = Circle::random(1000, 1000);
            assert!(circle.is_inside_bounds(1000, 1000));
        }
    }

    #[test]
    fn circle_new_keeps_center_and_radius() {
        let center = Point::new(12, 34);
        let circle = Circle::new(&center, 7);
        assert_eq!(circle.center.x, 12);
        assert_eq!(circle.center.y, 34);
        assert_eq!(circle.radius, 7);
    }

    #[test]
    fn circle_outline_draws_perimeter_pixels() {
        use crate::geometrical_shapes::test_canvas::Canvas;

        let mut canvas = Canvas::new(20, 20);
        let circle = Circle::new(&Point::new(10, 10), 2);
        circle.draw_to(&mut canvas);

        assert!(canvas.pixels.contains(&(12, 10)));
        assert!(canvas.pixels.contains(&(10, 12)));
        assert!(canvas.pixels.contains(&(8, 10)));
        assert!(canvas.pixels.contains(&(10, 8)));
        assert!(!canvas.pixels.contains(&(10, 10)));
        assert!(canvas.pixels.len() > 8);
    }

    #[test]
    fn random_circle_is_safe_on_small_canvas() {
        let circle = Circle::random(1, 1);
        assert_eq!(circle.center.x, 0);
        assert_eq!(circle.center.y, 0);
        assert_eq!(circle.radius, 0);
    }

    #[test]
    fn random_cubes_stay_inside_large_canvas() {
        for _ in 0..1000 {
            let cube = Cube::random(1000, 1000);
            assert!(cube.is_inside_bounds(1000, 1000));
        }
    }

    #[test]
    fn random_cube_is_safe_on_small_canvas() {
        let cube = Cube::random(1, 1);
        assert!(cube.is_inside_bounds(1, 1));
    }

    #[test]
    fn cube_new_calculates_depth() {
        let cube = Cube::new(&Point::new(300, 300), 90);
        assert_eq!(cube.depth, 30);
    }
}
