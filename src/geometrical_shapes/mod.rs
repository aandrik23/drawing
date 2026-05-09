//! Geometrical shapes — teammates add modules per `TASKS.md`:
//! `point_line.rs`, `triangle_rect.rs`, `circle_mod.rs`.
//!
//! After implementing, declare modules here and `pub use` the shape types.

pub mod circle_mod;
pub mod point_line;
pub mod triangle_rect;

pub use circle_mod::{Circle, Cube};
pub use point_line::{Line, Pentagon, Point};
pub use triangle_rect::{Rectangle, Triangle};

pub trait Drawable {
    fn draw(&self, image: &mut raster::Image);
    fn color(&self) -> raster::Color;
}

pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: raster::Color);
}
