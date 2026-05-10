//! Person 1 — `Point`, `Line`, `Pentagon` (see `TASKS.md`).

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
