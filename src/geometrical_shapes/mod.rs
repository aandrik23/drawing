//! Geometrical shapes — teammates add modules per `TASKS.md`:
//! `triangle_rect.rs`, `circle_mod.rs`.

mod bresenham;
mod line;
mod pentagon;
mod point;

#[cfg(test)]
pub(crate) mod test_canvas;

pub use bresenham::bresenham;
pub use line::Line;
pub use pentagon::Pentagon;
pub use point::Point;

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
