# Drawing exercise — work split for three people

The crate layout matches who owns which types. Each person works mainly in **one file** under `src/geometrical_shapes/`.

Workload is spread so everyone has a similar amount of work: **three deliverables** for Person 1, **two core shapes** for Person 2 (they provide shared polygon edges others reuse), and **two heavier shapes** for Person 3 (filled circle plus the bonus cube).

---

## Person 1 — `point_line.rs`

- **`Point`**: `new`, `random`, `Drawable` (single pixel), chosen color.
- **`Line`**: `new`, `random`, Bresenham `draw`, color.
- **`Pentagon`** (bonus): structure (e.g. center + radius, or five `Point` vertices); `Drawable`; draw as a closed polygon using the same line drawing as edges (reuse Bresenham from `Line`).
- **Tests** in the same file: random stays in canvas bounds; sanity checks.

**Integration:** `main.rs` draws one random line and one random point first, then one pentagon (fixed or random — team chooses).

---

## Person 2 — `triangle_rect.rs`

- **`Triangle`**: `new` from three `Point`s; draw as three edges (reuse `bresenham` from `point_line`).
- **`Rectangle`**: `new` from two `Point`s; normalize min/max; draw four edges.
- **Tests**: e.g. rectangle min/max independent of point order.

**Integration:** `main.rs` draws one fixed rectangle and one fixed triangle (coordinates given there).

---

## Person 3 — `circle_mod.rs`

- **`Circle`**: `random` (center + radius fully inside the image), filled disk `draw`, color.
- **`Cube`** (bonus): structure and traits needed to draw it in 2D (e.g. wireframe or isometric projection with twelve edges, or visible faces — pick one approach and document it); `Drawable`; stay inside the canvas where possible.
- **Tests**: random circles stay inside bounds for a large canvas; cube tests as appropriate (e.g. projected vertices in range).

**Integration:** `main.rs` draws **49** random circles in a loop, then one cube (fixed pose/size).

---

## Shared rules (everyone)

- **`Displayable`** / **`Drawable`** live in `mod.rs`; do not change their signatures without team agreement.
- **`main.rs`** wires `Image` as `Displayable` (bounds-checked `set_pixel`).
- Run **`cargo test`** and **`cargo run`** → produces **`image.png`** in the project root.
- After adding bonus types, export them from `mod.rs` and extend `main.rs` in the order described above.
