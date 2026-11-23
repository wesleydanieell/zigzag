# Layers artwork previews

Purpose: show recognizable source geometry beside every Layers object and a
combined preview for each actual SVG group. Observed in ZigZag code: the shared
`src/studio/shape_thumbnail.rs` painter projects `Shape.geometry`, using thread
colors and one aspect-preserving transform over member union bounds. Group
previews omit hidden members; individual hidden objects retain their preview.
Fill rings now have solid color interiors rendered with bounded horizontal spans
that respect SVG EvenOdd and NonZero fill rules, retaining concavities and holes
without introducing a tessellator. Open strokes remain open; closed paths remain closed.
Empty paths paint nothing, points paint dots, and degenerate bounds center safely.
Coordinates are normalized before subtraction to avoid overflow.

The shared painter has no separate interaction area. Layers and Threads use compact
20 px previews, with 24 px rows and theme-native gaps and hierarchy indentation.
Existing row selection,
collapse, eye/lock controls, counts, warnings, swatches, labels and drag/reorder
remain in place. `show_rows` still virtualizes Layers, offscreen previews are
skipped, and each preview emits at most 2,048 sampled outline points (128 per path).
Fill interiors use a bounded 22-cell scanline grid and an independent 16,384
source-point/2,048-span budget; extremely complex fills can fall back to outlines.
Bounds use all finite source points. Extremely dense composites can omit paths
once that drawing budget is exhausted; outline sampling can simplify fine detail.

The initial Layers-only PR excluded Threads. Later stacked PRs added the shared
renderer to Threads, solid fills and common row metrics. File/schema changes,
planning/export changes and external assets remain out of scope. No migration.

## Acceptance and validation

- Pure renderer tests: uniform scaling, group member positions and colors,
  same-color duplicates, open/closed paths, extreme coordinates, points, empty
  geometry, hidden members and the drawing budget.
- GUI fixture at 260 px in dark/light: both individual paths and both members of
  the composite are painted; object selection, group collapse and group drag
  still work. Existing object/group drag/reorder tests run in the full suite.
- `CARGO_BUILD_JOBS=2 cargo test --offline --lib` on host:
  **330 passed, 0 failed, 7 ignored**.
- `CARGO_BUILD_JOBS=2 cargo test --offline --features gui --bin zigzag-studio`
  on host: **477 passed, 0 failed, 16 ignored**.
- `CARGO_BUILD_JOBS=2 cargo build --offline --features gui --bin zigzag-studio --bin zigzag`:
  passed.
- Targeted Rust 2024 formatting applied; unrelated baseline formatting preserved.
  `cargo fmt --all -- --check` reports existing formatting differences (including
  `src/formats/charts.rs` and existing Studio code). `git diff --check`: passed.

Observed in application: real Studio captures of an author-created multielement
SVG show coral/blue petals together in their actual group, separate member
previews, an open green stem and a gold ring with a visible hole. Dark/light
screenshots are `/tmp/zigzag-thumbnail-captures/still-001.png` and `still-002.png`;
fixture is `/tmp/zigzag-thumbnail-fixture.svg`. Capture command (repeat with
`ZIGZAG_THEME=light`):

```sh
XDG_CONFIG_HOME=/tmp/zigzag-thumbnail-config ZIGZAG_THEME=dark \
ZIGZAG_CAPTURE=/tmp/zigzag-thumbnail-captures ZIGZAG_CAPTURE_STILL=2 \
target/debug/zigzag-studio /tmp/zigzag-thumbnail-fixture.svg
```

Real captures use the default wider dock; 260 px layout and interactions are
covered by the GUI harness. Initial sandbox GUI runs could not run local-server
and process tests; the complete host run above passed.

## Later stacked improvements (23 November 2026)

- `fix/thumbnail-filled-shapes` (`72b8010`): source-color solid areas with
  EvenOdd/NonZero holes and nested islands, bounded scanline rendering.
  Tests: 330 core passed; 486 GUI passed.
- `feat/layers-row-layout` (`5e7cd89`): compact 24 px tree rows, consistent
  16 px depth steps, 20 px miniatures, name/count columns; duplicate member
  counts removed from group labels. Tests: 330 core, 487 GUI passed.
- `feat/threads-row-layout` (`aa1fa8c`): matching 16 px nested object hierarchy,
  16 px thread swatches and 20 px artwork previews; right-aligned Start/End
  commands retained. Tests: 330 core, 487 GUI passed.

A real Studio run and authored SVG check included a solid circle, diamond,
concave region, holed ring and open strokes. Source RGBs, pass grouping and
manual machine commands are unchanged by presentation-only improvements.
Full `cargo fmt --all -- --check` still reports unrelated baseline formatting
differences; the scoped diff passed `git diff --check`.
