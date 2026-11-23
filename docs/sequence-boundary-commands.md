# Object boundary commands

Threads → Sewing Order and By Color list objects under consecutive thread blocks in actual
planned sewing order. Repeated A–B–A colors stay three blocks. Hidden, skipped,
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

Threads → By Color shows the same successive planned passes and nested object
rows as Sewing Order: A–B–A stays three passes, and consecutive same-color passes
separated by a Stop remain separate, marked “(stop)”. Pass numbers and stitch
counts describe the current plan; they are not saved object identities. Both
views share object selection and Start/End menus, with the same parameters and
undo history. Merely viewing or switching modes never adds machine commands.

Click a pass header to select only that pass's objects. In By Color, clicking
any occurrence's swatch selects all document objects with that RGB and opens
the picker for global recoloring (including repeated passes). Sewing Order's
swatch selects and recolors only that pass. Swatch tooltips explain this scope.
Both lists scroll within ten header-row heights; long names have full tooltips.

Drag an object's name in either Threads view to change the document's source
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
