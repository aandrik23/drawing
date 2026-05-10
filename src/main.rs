mod geometrical_shapes;

use geometrical_shapes as gs;
use geometrical_shapes::Displayable;
use raster::{Color, Image};

fn main() {
    let mut image = Image::blank(1000, 1000);

    // Person 1 — line, point, pentagon (see `TASKS.md`). Teammates add rectangle, triangle, circles, cube.
    gs::Line::random(image.width, image.height).draw(&mut image);

    gs::Point::random(image.width, image.height).draw(&mut image);

    gs::Pentagon::new(&gs::Point::new(200, 200), 120).draw(&mut image);

    raster::save(&image, "image.png").unwrap();
}

impl Displayable for Image {
    fn display(&mut self, x: i32, y: i32, color: Color) {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            self.set_pixel(x, y, color).unwrap();
        }
    }
}
