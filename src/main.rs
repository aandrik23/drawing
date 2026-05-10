mod geometrical_shapes;

use geometrical_shapes as gs;
use geometrical_shapes::{Displayable, Drawable};
use raster::{Color, Image};

fn main() {
    let mut image = Image::blank(1000, 1000);

    gs::Line::random(image.width, image.height).draw(&mut image);
    gs::Point::random(image.width, image.height).draw(&mut image);
    gs::Pentagon::new(&gs::Point::new(250, 250), 100).draw(&mut image);

    gs::Rectangle::new(&gs::Point::new(150, 300), &gs::Point::new(50, 60)).draw(&mut image);
    gs::Triangle::new(
        &gs::Point::new(500, 500),
        &gs::Point::new(250, 700),
        &gs::Point::new(700, 800),
    )
    .draw(&mut image);

    for _ in 1..50 {
        gs::Circle::random(image.width, image.height).draw(&mut image);
    }

    gs::Cube::new(&gs::Point::new(800, 250), 120).draw(&mut image);

    raster::save(&image, "image.png").unwrap();
}

impl Displayable for Image {
    fn display(&mut self, x: i32, y: i32, color: Color) {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            self.set_pixel(x, y, color).unwrap();
        }
    }
}
