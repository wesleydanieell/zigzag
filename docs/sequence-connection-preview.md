# Selected object connections

**Observed in ZigZag code:** View Options → Selected object connections is an
opt-in, session-only overlay, default off. It supplements Jump stitches, which
continues to control all generic jumps independently. Nothing is saved to SVG or
preferences; planning, run flags, exports and undo are unchanged.

Select exactly one sewn object. Blue **In** marks its first planned owned sewing
portion; **Out** marks its last. Dashed links show the preceding other object's
last point and following other object's first point. Repeated passes use the
selected object's first and last occurrences, so an A–B–A selection of A has no
external neighbour links. A sole object has markers only. Multiple selections,
hidden/Skip/unsewn objects, playback and editing gestures suppress the overlay.

These are **planned, schematic** direct endpoint links, not a routing proposal
or a guarantee of a continuous sewn connection. Neutral dashed links indicate
trim-before or a block boundary (color change or Stop, including same-color
Stop). No stitch or machine command is added. Recorded travel ranges refine
entry to the travel's destination; where the plan has no separate travel record,
entry is the first owned point and may include a short connecting hop. Actual
routed paths are not reconstructed. Imported machine files without ownership,
and inconsistent ownership metadata, have no overlay.

The pure projection walks blocks/runs and ownership spans, with a travel-range
lookup: O(runs + spans + travels), no stitch-sized allocation or stitch scan.
It is derived on visible redraws rather than cached; endpoint screen positions
always use the current canvas View, including zoom and pan. Labels and endpoints
use the existing light/dark palette and remain clipped to the canvas.

## Validation (2026-11-23)

All Cargo commands used `CARGO_TARGET_DIR=/home/wesley/Projects/zigzag-s06/target`
and `CARGO_BUILD_JOBS=2`.

- `cargo test --offline --lib`: 330 passed, 7 ignored.
- `cargo test --offline --features gui --bin zigzag-studio`: final host run
  495 passed, 0 failed, 16 ignored. The sandbox run had 28 socket/network-related
  failures; host execution resolves them. All seven new tests pass.
- `cargo build --offline --features gui --bin zigzag-studio --bin zigzag`: passed.
- Targeted Rust 2024 formatting and `git diff --check`: passed. The read-only
  `cargo fmt --all -- --check` reports untouched baseline differences, beginning
  in `src/formats/charts.rs`; these were left unchanged.

**Observed in application:** Studio launched with an original three-path SVG,
`--new-window --no-mcp`, separate temporary XDG configurations, and its native
capture harness (`ZIGZAG_CAPTURE_SHOW=connections`, selecting the second object).
Actual enabled-overlay captures were inspected in dark and light appearances:
`target/s06-proof/clean-dark/still-001.png` and
`target/s06-proof/light-clear/still-001.png`. The latter hides the default Agent
panel through temporary panel configuration to expose both endpoints. Logs and
fixture are under the ignored `target/s06-proof/` directory.

Popup toggling, invalid/hidden/multiple selection, playback, marker drag, replan
and SVG preservation are verified by the GUI harness. Clipping, palette contrast
(at least 3:1 against the overlay backing), and current zoom/pan transformation
are automated checks. Live zoom/pan and continuous desktop pointer interaction
were not manually validated: another concurrent desktop session makes pointer
automation unreliable. No machine-format or hardware behavior is changed.
