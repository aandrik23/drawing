# rust-drawing

Rust exercise that generates an `image.png` with geometric shapes drawn on a `1000x1000` canvas using the `raster` crate.

The program draws:

- 1 random line
- 1 random point
- 1 pentagon
- 1 rectangle
- 1 triangle
- 49 random circles
- 1 random wireframe cube

## Requirements

- Rust
- Cargo

## Run

```bash
cargo run
```

The generated file is saved as `image.png` in the project root.

## Tests

```bash
cargo test
```

The tests cover core invariants such as:

- `random` methods staying within canvas bounds
- correct `Rectangle` normalization when points are provided in reverse order
- degenerate and edge cases such as a zero-length line
- validation that `Circle` and `Cube` are generated within bounds

When run locally, all tests pass: `15 passed`.

## Project Structure

```text
src/
  main.rs
  geometrical_shapes/
    mod.rs
    bresenham.rs
    point_line.rs
    triangle_rect.rs
    circle_mod.rs
    test_canvas.rs
```

## Shapes

- `Point`: created with `new(x, y)` and randomly generated with `random(width, height)`
- `Line`: created from two `Point`s and drawn with Bresenham's algorithm
- `Triangle`: drawn using 3 edges
- `Rectangle`: created from 2 points with normalized bounds
- `Circle`: randomly generated inside the canvas and drawn as a circle
- `Pentagon`: regular pentagon defined by `center` and `radius`
- `Cube`: 2D wireframe cube using an offset/depth projection

## Traits

The core traits are defined in [`src/geometrical_shapes/mod.rs`](/home/chbaikas/Downloads/cohort/rust-drawing/src/geometrical_shapes/mod.rs):

- `Drawable`: used to draw a shape and return its color
- `Displayable`: abstraction over the drawing surface where pixels are written

[`src/main.rs`](/home/chbaikas/Downloads/cohort/rust-drawing/src/main.rs) implements `Displayable` for `raster::Image` with bounds checking before each `set_pixel`.

## Dependencies

- `raster = "0.2"`
- `rand = "0.8"`

## Notes

- Each shape uses a different color.
- The project is organized into modules by shape group.
- A helper test canvas is included for unit tests without requiring a real image backend.
