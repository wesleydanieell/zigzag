# Object boundary commands

Threads → Sewing Order lists objects under consecutive machine passes in actual
planned sewing order. By Color → Advanced groups adjacent same-RGB passes for
presentation only, with objects under expandable color parents. Repeated A–B–A colors stay three blocks. Hidden, skipped,
and non-sewing objects are absent; an object producing multiple runs appears once
per block. Click its name to select it on the canvas. The two inline menus edit
commands on that object, independent of selection.

| Slot | Supported choices |
| --- | --- |
| Start command | Auto, Trim |
| End command | Auto, Trim, Stop, Trim + Stop |

Auto clears the explicit override and lets routing decide. It does not suppress
inferred trims. Start Trim cuts only if a thread is already running. Adjacent end
and start trims produce one boundary cut. Stop requests a pause after the object;
it keeps the thread color. Stop is an end-slot choice, not the name of the slot.

Stitch → Sewing keeps its compact Trim Before, Trim After and Stop checkboxes;
these update the same persisted parameters as the Sequence View menus. The
existing Start/End canvas-marker buttons still edit geometric entry/exit
positions. They are unrelated to machine commands.

Commands use existing SVG ID tokens: `trim_before`, `trim`, and `stop`. Old IDs,
including `trim=1 stop=1` and explicit false values, remain readable. Displaying
controls preserves false tokens; choosing Auto removes overrides. There is no
second persisted command state. Undo/redo and regeneration use document history;
commands remain attached to shapes across reorder, not to sequence row numbers.
Agent `set_object` and `get_design` retain their existing fields.

Export remains unchanged: stops split blocks with the same thread color. The
existing export stream lowers block boundaries to a trim and color-change opcode,
including same-thread stops. A dedicated machine-stop encoding is not added here;
the format and machine determine pause behavior. Machine files do not preserve
editable SVG object boundaries. No physical machine validation is claimed.

The full DG command catalog is **not implemented**. Start Stop, explicit No
Command, Jump, speed, borer, laser, sequin and other machine-specific interventions
are not offered. This is a clean-room UI over the existing trim/stop planner,
not a new embroidery engine or export capability.

In Sewing Order, a pass with one sewn object uses one horizontal row: pass number, thread swatch,
object name, and Start/End controls. Wider docks also show the thread name; narrow
docks keep it in the swatch tooltip and truncate object names with full tooltips.
Pass and object selection remain separate click targets. Multiple-object passes
keep a header followed by compact object rows with the same controls.

Threads → By Color → Advanced always shows a color parent and child object rows,
including a single color with a single object. Adjacent same-RGB machine passes
merge visually, even across Stop boundaries; A–B–A remains three ordered groups.
An object spanning multiple adjacent passes appears once, in first-sewn order.
Parent groups now show composite filled artwork previews; nested objects have individual thumbnails.
Groups default expanded. Disclosure buttons support keyboard activation and keep
expansion through selection, replan, Stop edits and undo using first object UID, rather than planned block index or color. Stop remains visible in the child's
existing End menu. Group stitch totals describe the plan; viewing never edits it.

Click a pass header (or Advanced color parent) to select only its objects. In By Color, clicking
any occurrence's swatch selects all document objects with that RGB and opens
the picker for global recoloring (including repeated passes). Sewing Order's
swatch selects and recolors only that pass. Swatch tooltips explain this scope.
Both lists scroll within ten header-row heights; long names have full tooltips.

Drag an object's name in Sewing Order or By Color → Advanced to change the document's source
sewing order. The accent insertion line shows before/after the target name;
the floating name previews the single object being moved. Drop on the bottom
half of the last name, or the small space below the list, to sew it last.
Hover near the scroll viewport's edges to scroll while dragging. Start/End
buttons remain separate click targets; clicking a name still selects on canvas.
Each effective move is one undo step. Escape, dropping outside the name targets
and end space, or dropping in the same place leaves the document unchanged.

Only the dragged object moves, even with several objects selected. Hidden and
Skip objects have no sequence rows; locked objects cannot be dragged or selected
by their name. Commands and selection follow stable object UIDs through moves
and undo. Layers uses the same structural reorder/history path. No Stop is
inserted to preserve a former color pass: A–B–A can become A–A–B and merge passes.

The existing planner still applies its travel shortening within stretches of
three or more eligible same-color objects, and cap ordering when enabled.
Consequently planned rows may differ from source order under those rules;
reordering does not disable optimization or change planner/export semantics.
An object with multiple planned color runs still moves as one source object,
from any of its occurrences; passes are not independently draggable.

## Simple and Advanced color views

**Observed in ZigZag code:** By Color starts in Simple mode. Its small
Simple / Advanced segment appears only within By Color. Simple lists one summary
per distinct document object RGB (excluding Skip objects), in first-object order:
color swatch, chart thread name, and summed per-object stitch count. A–B–A has two
summaries. It uses `design_threads()`, not `design.blocks`; Stop can split the plan
but cannot split a Simple summary. There are no expandable pass rows in Simple.
Selecting a summary selects all its objects; its swatch selects and recolors all
objects of that RGB, including repeated passes, with normal undo history.

Advanced shares child controls and source-object drag handling with Sewing Order.
Its small pure `color_runs` helper merges only adjacent RGB passes and deduplicates
object indices without changing `design.blocks`, `block_shapes` or the document.
Sewing Order retains its actual pass boundaries and singleton inline controls.
Neither switching mode nor showing groups inserts Stop or edits the document.

The `ViewTools::thread_advanced` flag defaults to false and is UI-only. Replan,
Stop edits, undo, save, and switching Sewing Order / By Color preserve the user's
choice for this Studio session. It is not serialized to SVG. Planner, parameters,
export and previews are outside this change; existing machine-format limitations
above remain applicable. Existing SVG compatibility is unchanged.

### Validation of this bounded change (2026-11-23)

Automated evidence is in `threads::tests`: distinct repeated RGB summaries;
selection of all matching objects; a real Stop split in Advanced followed by
stable Simple membership and undo; save/replan and view switching in both modes;
260px summary bounds and keyboard activation; global swatch recoloring with one
undo. Existing `dock::tests` exercise Advanced commands and reorder explicitly;
`sequence_list` layout, menus, keyboard operation and Sewing Order remain covered.

Commands and results:

- `CARGO_BUILD_JOBS=2 cargo test --offline --lib`: 330 passed, 7 ignored,
  both sandbox and host.
- `CARGO_BUILD_JOBS=2 cargo test --offline --features gui --bin zigzag-studio`:
  final host: 474 passed, 0 failed, 16 ignored. Sandbox socket tests cannot bind local
  sockets (`Operation not permitted`); rerun on the host, outside the sandbox.
- `CARGO_BUILD_JOBS=2 cargo build --offline --features gui --bin zigzag-studio --bin zigzag`:
  passed.
- Targeted `rustfmt --edition 2024` on threads.rs and view_tools.rs; dock.rs
  changes are only two test setup lines. `cargo fmt --all -- --check` reports
  existing differences in untouched files (including formats/charts.rs and
  vendored modules); no blanket formatting applied. `git diff --check` passes.

**Observed in application:** Studio launched on the host with an author-created
three-path A–B–A SVG, `--new-window --no-mcp`, temporary `XDG_CONFIG_HOME`, and
capture mode. The dark-theme wide dock shows three Sewing Order passes; clicking
By Color shows Simple selected and two swatch/name/count summaries (12 red,
6 green stitches), with no object command rows. Screenshots and launch fixture:
`/tmp/zigzag-simple-proof/` (`shots/still-001.png`, `shots/still-002.png`).
Narrow 260px bounds and keyboard interaction are verified by the GUI harness;
live narrow/light-theme interaction has not been manually inspected.

No branch switches, commits, pushes, PR creation, or other-worktree edits.
The reviewer owned that stage; subsequent stacked PRs have added preview work and row spacing.

### Advanced automatic grouping (2026-11-23)

**Observed in ZigZag code:** This bounded change touches Threads/Dock presentation
and this document only. No dependencies, thumbnail rendering, planner, SVG command
serialization or exporter changes. Existing files and machine behavior are unchanged.
The visual expansion identity follows the first-object UID, so recoloring
does not unexpectedly collapse the group. A group with a new first object starts expanded.

Automated acceptance: pure adjacent A–A–B–A grouping and cross-pass deduplication;
one/two-object parent and child rows; Stop visible on child; keyboard disclosure,
collapse surviving Stop undo; existing parent/child selection, global RGB recolor,
drag/undo and 200/260px command-layout tests. Existing Sewing Order and Simple tests
remain enabled. Host full GUI suite: 479 passed, 0 failed, 16 ignored.
`CARGO_BUILD_JOBS=2 cargo test --offline --lib`: 330 passed, 0 failed, 7 ignored.
`CARGO_BUILD_JOBS=2 cargo build --offline --features gui --bin zigzag-studio --bin zigzag`:
passed. Sandbox GUI socket tests fail to bind; full suite rerun on host as above.
Targeted Rust 2024 formatting applied only to changed code; full read-only
`cargo fmt --all -- --check` reports untouched baseline differences, starting in
formats/charts.rs. `git diff --check` passes. No branch switch, commit, push or PR.
