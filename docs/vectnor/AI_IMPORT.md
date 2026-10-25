# Local AI opening verification

> Last reviewed/updated: 2026-10-25 · minor: general AI text-overflow reporting; scores unchanged.

Status: 2026-10-22; local implementation and verification complete. No upload.

AI/AI-template opening now tries the bounded editing-data reader before importing the PDF
page. A complete result (no omitted-art or omitted-text warnings) is used directly. This
fixes false rejection when the optional PDF page includes margins absent from the native
artboard, and avoids depending on the PDF drawing to recreate editable objects.

Explicit page selection, non-default crop, outlined text, disabled layers or disabled editing
data preserve the PDF route. Incomplete editing data retains page-assisted recovery and its
warnings. The original AI source remains protected against silent overwrite even after a
complete import, because Vectnor cannot write its original editing data back. Saving a new
`.vcnr` preserves the imported document. Vectnor's own embedded native document still takes
precedence. Unsupported live features and writing the other editor's private editing data remain outside this change.

The owner supplied one AI file for local testing. Its private data independently recovers an
A4 artboard (595.2803 × 841.8896 pt), one layer, ten groups, twelve compound paths, 88 path nodes
and four editable text objects. Its optional PDF page is 36 pt larger in each dimension;
the previous import therefore rejected the native structure and used its PDF drawing.
The initial native run reported four unavailable fonts. The final bundled-app check resolves
one of those families; three still use fallback fonts. No font files were added to the repository.
This one-file result does not establish general AI fidelity.

The supplied file and decoded content are kept only in the local temporary verification
folder, never in Git. Synthetic regression inputs are written from scratch. No Adobe
application or installation is accessed; embedded prologs are not copied or used as code.
Public behavior reference: [saving artwork](https://helpx.adobe.com/illustrator/using/saving-artwork.html).

Native verification completed using Vectnor's control channel on isolated ports:

- File Open reports no editing-data import warnings and retains the native A4 artboard.
- The initial Missing Fonts dialog reports four unavailable families. A final bundled-app
  check reports three remaining missing families after local font availability changed;
  the agent added no fonts.
- An imported text object accepts Telugu text; undo/redo works and the final undo restores
  the entire layer JSON exactly.
- `.vcnr` save/reopen preserves the entire layer JSON and all four source strings;
  the reopened document is clean and its artwork PNG SHA-256 matches the pre-save render.
- Export over the AI source is refused without acknowledgement. Save As shows the original
  editing-data protection dialog; Cancel leaves the source bytes unchanged.
- Canvas, Missing Fonts and overwrite-dialog screenshots were inspected. Both native test
  instances quit cleanly. The original user-supplied file's SHA-256 is unchanged.

All eight `cargo xtask ci` gates pass: fmt, strict clippy, tests (5,267 passed, zero failed,
four existing ignored), assets, brands, clean-room, layers and core WASM. The separate actual
`vectnor-web` WASM check also succeeds, with the pre-existing `missing_links.rs` unused-function
warning. The initial sandboxed run was blocked by two existing localhost-socket tests;
the full rerun with localhost permission passes. Both native test instances quit cleanly.
The local optimized development app/CLI bundle was refreshed and SHA-256 verified; version
remains 0.9.0. No source, app, file or generated artwork was uploaded.

The 50k-path performance baseline passes eight of ten budgets; zoom is 23.22 ms (16 ms limit)
and native open is 368.57 ms (300 ms limit), at unknown load with initial compilation activity.
The first after trial reports a busy machine (load 16.7 on 12 cores) and is excluded from
performance inference. Its existing zoom/open overruns persist (21.64 ms zoom; 303.67 ms
open). A later load check rose to 41.9, so no further performance trial was attempted.
No speedup or regression is claimed.

Local evidence, including the supplied file and generated native copy, is under `/tmp/vectnor-ai-import-2026-10-22/`.
No file from that folder is included in the commit.


## Original text frames — 2026-10-22

Status: local implementation and verification complete. No upload.

A synthetic AI file with two original area-text frames reproduced the reported fragmentation:
the previous bundled importer produced five text objects when an unrelated unsupported
symbol forced the PDF fallback. The updated importer produces two original frames, including
multiline content and styled character runs. Source frame identities, text and styles come
from the AI editing data. The PDF fallback continues to carry unsupported non-text artwork.

The text-recovery reader tolerates unsupported artwork only to extract known original text
frames; it is never used as a whole-document import. Syntax, nesting, decompression and other
reader limits remain enforced. Additional caps bound recovery bytes, nodes, text, candidate
pairs and string-comparison work. Replacement requires complete text agreement, matching
orientation and matching complete artboard sets. Fragments must be contiguous siblings in one
graphics context. Every plausible source frame claims its fragments, including frames that
cannot themselves be restored, so overlapping identities cannot consume a separate box.

Ambiguous, incomplete or mismatched text remains unchanged. If any source text frame
cannot be read, frame recovery is withheld so its story cannot absorb another original frame. Recovery across groups/masks,
intervening artwork, different crops or missing/ambiguous artboards, and fallback path type
remain unsupported. Complete linked stories can now be restored atomically; see the real-file check below. Supported direct native AI text/thread/path import is unchanged.
When original editing data is absent or unreadable, the precise original boxes cannot be
reliably reconstructed from printed fragments alone. Existing `.vcnr` conversions need the
original AI file reopened; their discarded source metadata cannot be recreated.

Tests cover multiline area and point frames, mixed styles, separate/overlapping originals,
unread frames sharing a story,
Indic text and joiners, mismatches, crop differences, threading, rotated fragments and
separate graphics contexts. An integration test keeps unsupported-symbol fallback while
restoring the original paragraph. Mutated AI text-document fuzzing exercises recovery.

Public behavior reference: [point, paragraph and area text](https://helpx.adobe.com/illustrator/using/tool-techniques/type-tool.html).
No Adobe software, installations, prologs, samples or fonts are accessed or added. The earlier
owner-supplied single-line AI does not reproduce this multiline report; the affected file
has been requested for case-specific verification. No real-file fidelity claim is made here.

Verification:

- Synthetic AI opening changes five fragmented text objects into the two original area frames;
  the first retains three styled character runs. Unsupported non-text artwork and its warning
  remain on the fallback route.
- Native Type-tool selection spans the first frame's full 42-byte text across old line
  boundaries. Telugu editing, undo/redo and the separate second frame are verified.
- `.vcnr` save/reopen keeps the complete layer JSON, text, frame kinds and styles; rendered
  artwork PNG SHA-256 is identical and the reopened document is clean. Canvas and reopened
  screenshots were inspected. Both native test instances quit cleanly.
- The earlier supplied single-line AI produces identical CLI information before and after.
  It remains a regression check, rather than evidence for the reported multiline file.
- All eight CI gates pass: 5,276 tests passed, zero failed, four existing ignored. This includes
  eight new EPS tests and the new mutated-text recovery property. The separate browser WASM
  check passes, with the pre-existing `missing_links.rs` unused-function warning.
- The SHA-256-verified local app/CLI bundle was refreshed, version 0.9.0 unchanged. No upload.
- The 50k-path baseline overlapped initial compilation and passed six of ten budgets; its
  timings are not suitable for comparison. The final trial after builds and GUI exit passes
  nine of ten at load 6.8/12 cores: zoom 21.34 ms exceeds the 16 ms budget; native open is
  294.37 ms against 300 ms. The earlier after trial had open 304.12 ms and zoom 21.28 ms.
  These single trials do not establish a speedup/regression, and parity scores remain unchanged.

Verification evidence: `/tmp/vectnor-ai-text-2026-10-22/` (local only).


## Multi-artboard linked frames — 2026-10-22

The owner supplied an affected two-artboard AI for local checking. The prior app imported
143 text fragments. Its editing data contains 10 original text objects, including two
stories each linked through two area frames. An omitted identity matrix made one point
frame unreadable, area-frame translations were ignored, and fallback recovery skipped
multiple artboards and linked stories. These cases are now handled. Common Latin
presentation ligatures are compared with their underlying letters without changing the
original story text; superscript/subscript character positions retain their native styles.

Every native artboard must match a printed artboard in file order and size. Each frame
maps through its own artboard translation. Frames spanning ambiguous artboards are left
alone. Linked-story recovery requires complete text agreement, every original frame,
unique fragment ownership and contiguous siblings within each frame's graphics context;
no partial linked story is restored. Reader and matching-work caps remain enforced.
Malformed explicit matrices are rejected; translated area frames are supported, while
other area affine transforms require corresponding style work and remain unsupported.
Printed fragments that are already area type can participate in a complete original frame.

The supplied document now imports as 10 text objects, preserving both pairs of linked
frames. Unsupported placed artwork still uses the PDF drawing and retains its warning;
this is recovery of original AI text metadata, not a whole-document native-artwork claim.
Six Open Sans styles are unavailable locally, so fallback fonts alter text wrapping
and can leave text overset in its original frame; all story text is retained for editing. No fonts or assets were installed or added. The supplied file, decoded
metadata and generated artwork stay under `/tmp/vectnor-ai-announcement-2026-10-22/` only,
never in Git; the original external file remains unchanged.

Synthetic regressions cover default/malformed point matrices, translated/unsupported area
matrices, script positions, different artboard arrangements and missing/cropped boards,
atomic linked stories, presentation ligatures, already-area fragments, incomplete stories
and overlapping frames. Mutated AI recovery includes translated area/thread and default
point matrix variants. First baselines are restored from each frame's saved layout; this
prevents the supplied title's last line becoming overset after grouping. Native screenshots
confirm all three title lines. Whole-paragraph Type selection, Telugu editing/history and a
linked-frame edit/history pass. Native save/reopen preserves complete layer JSON, both link
lists and exact source stories; artwork PNG SHA-256 is identical and the reopened copy is clean.
Area auto leading no longer treats printed paragraph gaps as character leading, avoiding
counting paragraph spacing twice. A full second-artboard inspection caught and corrected
that error. With the six unavailable Open Sans styles, the last frames of both linked
stories still report overflow: their complete text remains editable, but this local check
does not establish visual parity with the original fonts. Both artboard renders remain
identical across native save/reopen. All eight quality gates pass (5,286 tests passed, zero failed, four existing ignored),
including ten new synthetic regressions and the extended AI recovery mutation property.
The actual browser WASM build passes with the existing `missing_links.rs` unused-function
warning. The final optimized development app/CLI bundle (0.9.0) is SHA-256 verified.
Normal File Open with All Pages also imports the same ten objects and links, and the
earlier supplied single-line AI's CLI information is unchanged. No source or build uploaded.


## Revision history

| Date | Change | Evidence |
|---|---|---|
| 2026-10-25 | General AI/AIT overflow import notes after thread reflow; original content and frame sizes retained. | Synthetic import/visibility/limit tests and local file verification. |
| 2026-10-22 | Multi-artboard and atomic linked text recovery; matrix defaults/translations, saved first baselines, script positions and area Auto leading. Missing-font overflow remains documented. | Supplied AI: 143 fragments → 10 objects; two linked pairs; both-artboard native round trip. |
| 2026-10-22 | Original text recovery during unsupported-artwork fallback. | Synthetic five fragments → two original frames. |
| 2026-10-22 | Direct native AI opening before optional PDF drawing. | Earlier supplied single-line AI and synthetic margin cases. |


The optimized-development 50k-path performance baseline passed eight of ten budgets
(zoom 21.88 ms against 16 ms; native open 310.21 ms against 300 ms). The final trial after
all builds and test apps quit passed nine of ten (zoom 20.93 ms; open 292.60 ms).
Load was unavailable in both reports. Native-open timing fluctuates around its limit, and
the existing zoom overrun remains; no speedup or release-performance claim is made.


## General AI overflow reporting — 2026-10-25

The shared AI/AIT load path checks the final text layout after linked stories flow through
their original frames. An import note reports how many visible frames contain overflow
text with the current fonts. It explains that some text is not visible inside its frame
and that the full text is retained for editing. This applies to every foreign AI/AIT import,
including PostScript-based AI, without filename or content-specific rules. The desktop
status message, CLI and control/MCP document-open results share the same warning. The
overflow note comes first so the desktop status shows it; all other import notes are retained.

The diagnostic preserves original characters, styles, frame dimensions and threading.
Fitting text, hidden frames and unbounded point type do not produce overflow notes;
a story that fits after flowing into a later frame is not a false positive. Native Vectnor
payload restoration keeps its existing behavior. The note does not diagnose a cause:
missing fonts, unsupported layout details or already-overset source text can all matter.

Checks are bounded by visited nodes, frame count and text bytes. If checking stops at a
safety limit, a separate note explicitly says that remaining frames need review, rather
than implying that the whole document fits. Regression cases cover generic names/AI/AIT,
PostScript AI, exact source text/geometry preservation, fitting and linked type, hidden
ancestors, point type and an oversized frame skipped before shaping.


Verification: all eight CI gates pass (5,290 tests passed, zero failed, four existing ignored),
plus final engine lint and actual browser WASM checks. The browser check retains the existing
`missing_links.rs` unused-function warning. Normal File Open / All Pages shows the new note
in the desktop status. Its reported count of three matches individual layout queries for
all six area frames in that font environment; the CLI reports two in its own font environment.
Except for the added first warning, the CLI's document information and prior warnings are
identical to the baseline. The native app still has ten text objects and two linked pairs;
whole-paragraph selection, Telugu editing/history, linked editing/history and native
save/reopen preserve the layer JSON and both artboard PNG hashes. The reopened file is clean.
Original source bytes remain unchanged; no fonts or assets were added, and nothing uploaded.
