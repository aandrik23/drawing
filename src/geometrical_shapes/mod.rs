//! Shared geometry module wiring for the team project.
//!
//! Each teammate implements their assigned shapes in one module.

mod bresenham;

#[cfg(test)]
pub(crate) mod test_canvas;

pub mod circle_mod;
pub mod point_line;
pub mod triangle_rect;

pub use circle_mod::{Circle, Cube};
pub use point_line::{Line, Pentagon, Point};
pub use triangle_rect::{Rectangle, Triangle};

pub trait Drawable {
    fn draw(&self, image: &mut crate::raster::Image);
    fn color(&self) -> crate::raster::Color;
}

pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: crate::raster::Color);
}
