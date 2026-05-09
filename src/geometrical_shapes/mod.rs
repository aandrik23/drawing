//! Geometrical shapes — teammates add modules per `TASKS.md`:
//! `point_line.rs`, `triangle_rect.rs`, `circle_mod.rs`.
//!
//! After implementing, declare modules here and `pub use` the shape types.

use raster::Color;

/// Pixel writer used by all shapes (`Image` implements this in `main.rs`).
pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: Color);
}

/// Something that can be painted onto a [`Displayable`] surface.
pub trait Drawable {
    fn draw<D: Displayable + ?Sized>(&self, target: &mut D);
    fn color(&self) -> Color;
}
