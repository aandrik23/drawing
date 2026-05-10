use crate::geometrical_shapes::Displayable;
use raster::Color;

/// Rasterize a segment between two pixels using Bresenham's algorithm.
/// Safe for degenerate segments (`(x0,y0) == (x1,y1)`): draws a single pixel.
pub fn bresenham<D: Displayable + ?Sized>(
    target: &mut D,
    mut x0: i32,
    mut y0: i32,
    x1: i32,
    y1: i32,
    color: Color,
) {
    let dx = (x1 - x0).abs();
    let dy = -(y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;

    loop {
        target.display(x0, y0, color);
        if x0 == x1 && y0 == y1 {
            break;
        }
        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x0 += sx;
        }
        if e2 <= dx {
            err += dx;
            y0 += sy;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometrical_shapes::person_1::test_canvas::Canvas;
    use raster::Color;

    #[test]
    fn degenerate_segment_draws_single_pixel() {
        let mut c = Canvas::new(20, 20);
        bresenham(&mut c, 7, 7, 7, 7, Color::rgb(1, 2, 3));
        assert_eq!(c.pixels.len(), 1);
        assert_eq!(c.pixels[0], (7, 7));
    }
}
