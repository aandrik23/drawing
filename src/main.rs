mod geometrical_shapes;

use geometrical_shapes::Displayable;
use raster::{Color, Image};

fn main() {
    let image = Image::blank(1000, 1000);
    // Wire shape draws here once `Point`, `Line`, `Rectangle`, `Triangle`, `Circle` exist (see `TASKS.md`).
    raster::save(&image, "image.png").unwrap();
}

impl Displayable for Image {
    fn display(&mut self, x: i32, y: i32, color: Color) {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            self.set_pixel(x, y, color).unwrap();
        }
    }
}
