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

        let max_radius = (width.min(height) / 8).max(1);
        let radius = rng.gen_range(1..=max_radius);

        let x = rng.gen_range(radius..width - radius);
        let y = rng.gen_range(radius..height - radius);

        Self {
            center: Point::new(x, y),
            radius,
        }
    }

    pub fn is_inside_bounds(&self, width: i32, height: i32) -> bool {
        self.center.x - self.radius >= 0
            && self.center.y - self.radius >= 0
            && self.center.x + self.radius < width
            && self.center.y + self.radius < height
    }
}

impl Drawable for Circle {
    fn draw(&self, image: &mut Image) {
        let mut x = self.radius;
        let mut y = 0;
        let mut error = 0;

        while x >= y {
            let cx = self.center.x;
            let cy = self.center.y;
            let color = self.color();

            image.display(cx + x, cy + y, color.clone());
            image.display(cx + y, cy + x, color.clone());
            image.display(cx - y, cy + x, color.clone());
            image.display(cx - x, cy + y, color.clone());
            image.display(cx - x, cy - y, color.clone());
            image.display(cx - y, cy - x, color.clone());
            image.display(cx + y, cy - x, color.clone());
            image.display(cx + x, cy - y, color.clone());

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

        let max_size = (width.min(height) / 4).max(30);
        let size = rng.gen_range(30..=max_size);
        let depth = size / 3;

        let x = rng.gen_range(0..width - size - depth);
        let y = rng.gen_range(depth..height - size);

        Self {
            top_left: Point::new(x, y),
            size,
            depth,
        }
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
    fn random_cubes_stay_inside_large_canvas() {
        for _ in 0..1000 {
            let cube = Cube::random(1000, 1000);
            assert!(cube.is_inside_bounds(1000, 1000));
        }
    }

    #[test]
    fn cube_new_calculates_depth() {
        let cube = Cube::new(&Point::new(300, 300), 90);
        assert_eq!(cube.depth, 30);
    }
}