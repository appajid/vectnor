# Local AI opening verification

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
orientation and a single matching artboard. Fragments must be contiguous siblings in one
graphics context. Every plausible source frame claims its fragments, including frames that
cannot themselves be restored, so overlapping identities cannot consume a separate box.

Ambiguous, incomplete or mismatched text remains unchanged. If any source text frame
cannot be read, frame recovery is withheld so its story cannot absorb another original frame. Recovery across groups/masks,
intervening artwork, multiple artboards or different crops, and fallback threaded/path type
remain outside this change. Supported direct native AI text/thread/path import is unchanged.
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
