//! Test-only [`Displayable`] that records pixels and checks canvas bounds.

use raster::Color;

use crate::geometrical_shapes::Displayable;

pub struct Canvas {
    pub pixels: Vec<(i32, i32)>,
    w: i32,
    h: i32,
}

impl Canvas {
    pub fn new(w: i32, h: i32) -> Self {
        Self {
            pixels: Vec::new(),
            w,
            h,
        }
    }

    pub fn sorted_unique(&self) -> Vec<(i32, i32)> {
        let mut v = self.pixels.clone();
        v.sort_unstable();
        v.dedup();
        v
    }
}

impl Displayable for Canvas {
    fn display(&mut self, x: i32, y: i32, _color: Color) {
        assert!(x >= 0 && x < self.w && y >= 0 && y < self.h);
        self.pixels.push((x, y));
    }
}
