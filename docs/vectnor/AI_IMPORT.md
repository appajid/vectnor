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
