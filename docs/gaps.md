# Gaps

> **Last reviewed:** 2026-10-22 · **Last updated:** 2026-10-22 · **Change:** minor (reviewed input, editing and file fixes; scores unchanged) · **Target:** public documentation and owner requirements

Every known shortfall against Illustrator, one entry each, **ranked by how much it stops a professional from
switching**. This is the work list: unless you were given a task, pick the highest-ranked gap you can make
progress on, write a plan, and update the entry (and [target-app-parity.md](target-app-parity.md)) in the same PR.

Gap ids are stable (PRs say "Part of gap 2"); the rank can change. Hours are Opus 5.5 agent wall-clock hours for
one agent, estimated unless marked; calibration in
[target-app-parity.md](target-app-parity.md#calibration). **Beta** marks the gaps that must close before the beta
stage ([ROADMAP.md](../ROADMAP.md)).

## Ranked

| Rank | Gap | Hours | Beta | Doc |
|---:|---|---:|:---:|---|
| 1 | [G1 Interaction fidelity](#g1-interaction-fidelity) | 60–90 | yes | [ui-parity.md](ui-parity.md) |
| 2 | [G9 Illustrator `.ai` and EPS files from real users](#g9-illustrator-ai-and-eps-files-from-real-users) | 10–20 | yes | [file-format-parity.md](file-format-parity.md) |
| 3 | [G10 Launch and stability on real machines](#g10-launch-and-stability-on-real-machines) | 20–35 | yes | — |
| 4 | [G8 Hardening at scale](#g8-hardening-at-scale) | 30–50 | yes (part) | — |
| 5 | [G11 Correctness bugs in core tools](#g11-correctness-bugs-in-core-tools) | 8–15 | yes | [target-app-parity.md](target-app-parity.md) |
| 6 | [G4 Advanced type](#g4-advanced-type) | 34–47 | core part | [type-parity.md](type-parity.md) |
| 7 | [G12 Preferences and panels that don't act yet](#g12-preferences-and-panels-that-dont-act-yet) | 13–20 | yes | [ui-parity.md](ui-parity.md) |
| 8 | [G2 Photoshop-style raster effects and the Effect Gallery](#g2-photoshop-style-raster-effects-and-the-effect-gallery) | 24–40 | no | [effects-parity.md](effects-parity.md) |
| 9 | [G13 Illustrator 2026 (30.x) additions](#g13-illustrator-2026-30x-additions) | 15–25 | no | [target-app-parity.md](target-app-parity.md) |
| 10 | [G14 Pen hardware beyond Windows pressure](#g14-pen-hardware-beyond-windows-pressure) | 8–16 | macOS pressure | [hardware-parity.md](hardware-parity.md) |
| 11 | [G5 Brushes, symbols and libraries](#g5-brushes-symbols-and-libraries) | 17–28 | no | [target-app-parity.md](target-app-parity.md) |
| 12 | [G3 3D and Materials](#g3-3d-and-materials) | 45–75 | no | [effects-parity.md](effects-parity.md) |
| 13 | [G7 Views and windows](#g7-views-and-windows) | 15–25 | no | [ui-parity.md](ui-parity.md) |
| 14 | [G6 Automation and scripting](#g6-automation-and-scripting) | 8–14 | no | [target-app-parity.md](target-app-parity.md) |
| 15 | [G15 Performance budgets and a GPU renderer](#g15-performance-budgets-and-a-gpu-renderer) | 15–25 | budgets only | [hardware-parity.md](hardware-parity.md) |
| 16 | [G17 Ecosystem: third-party plug-ins](#g17-ecosystem-third-party-plug-ins) | 20–45 | no | — |
| 17 | [G18 Generative AI features](#g18-generative-ai-features) | 30–60 | no | — |
| 18 | [G19 Accessibility](#g19-accessibility) | 15–25 | no | [ui-parity.md](ui-parity.md) |

**Alpha blockers:** none. Every core workflow passes the [alpha gate](roadmap.md#alpha-gate); G9 is the only
partial one, and it blocks beta, not alpha.

**To beta:** G1, G9, G10, G11, G12, the corpus and QA part of G8, the type-core part of G4, macOS pen pressure
from G14 and the budget run of G15: **~170–270 h** one agent, ~50–80 h wall clock with 4–6 agents.

## G1 Interaction fidelity

- **Completed locally (2026-10-22):** Exact-size Alt-click origins and Polygon/Star count persistence, Pencil/Paintbrush zero-distance toggles, clipping arrangement, Recent tab reuse and panel scrollbar columns have focused regressions. Broader modifiers and interaction fidelity remain. [Audit](vectnor/UPSTREAM_AUDIT.md).

- **Completed locally (2026-10-21):** Double-click Type/Escape restoration, keyboard dropdown closure, Transform reference flips, zero-area marquees and isolation guards corrected. Broader modifier/widget fidelity remains. [Audit](vectnor/UPSTREAM_AUDIT.md).

- **Completed locally (2026-10-17):** Direct Selection Escape, filled Pencil stroke None, field focus commits and signed/mixed field arithmetic. Guide/widget cancellation and broader modifiers remain; [audit](vectnor/UPSTREAM_AUDIT.md).

- **Completed locally (2026-10-13):** Direct/Group Selection respects isolation and selected-path priority; anchor Align/Delete uses point semantics and original indexes; grouped Pathfinder, Pen jitter, Shift-double-click Width options, field contrast/slider rails and shortcut discovery are covered by the [reviewed upstream pass](vectnor/UPSTREAM_AUDIT.md). Key anchors and broader modifier work remain.

- **Completed locally (2026-10-12):** optional Alt/Option smoothing from Pencil/Paintbrush borrows the real Smooth tool with saved fidelity. One undo step, no extra path, original tool restored on release/Escape; Cmd/Ctrl selection keeps priority. Interior Pencil path redrawing and unchecked Alt straight-line drawing remain.
- **Completed locally (2026-10-12):** Pencil/Paintbrush Keep selected is configurable and saved separately for each tool. Preview selection, finished/continued strokes, cancellation, tool-switch completion and undo/redo follow the setting.
- **Completed locally (2026-10-12):** Alt/Option swaps a new ruler guide's orientation, preview and snapping while dragging from either ruler; release creates one guide in one undo step. Existing-guide Alt-drag retains copying.
- **Missing:** a tool-by-tool, panel-by-panel pass over modifiers, cursors, the Properties panel per context,
  isolation mode and small behaviours, against the public tool and shortcut documentation. Itemized in
  [ui-parity.md](ui-parity.md). The Shaper's construction-mode widget and gesture refinements remain.
- **Evidence:** no side-by-side session has ever happened (the clean-room rule forbids running Illustrator, so
  the pass works from the documentation); users keep reporting small differences: #991, #975, #973, #955, #908.
- **Impact:** a power user notices this in the first minutes and stops trusting the tool.
- **Estimate:** 60–90 h; splits well by tool across agents.

## G9 Illustrator `.ai` and EPS files from real users

- **Missing:** faithful layer and group structure from the editing data in real files (#951: 109 layers instead
  of 14; #868: round trip with Illustrator 2018 loses layers and groups), guides and non-printing construction
  lines (#779), open speed on large files (#758), legacy type in `.ai` files in PostScript form (those without
  their prolog open since #1027), live effects, brushes and symbols as live objects (#637; pattern fills and global colours are read since #1025). Writing Illustrator's own editing data is out of scope
  (undocumented); Save As `.ai` stays PDF-compatible.
- **Evidence:** the issues above; 28 of the 350 issues are about `.ai`/EPS (about 22 people, the most common single
  format theme), 20 fixed, and the 8 open ones are about structure (layers, guides, speed, live objects), not missing
  art. No real-file `.ai` corpus (users' files may be used locally, never committed).
- **Impact:** blocks anyone who hands files back to Illustrator users. Opening and reworking their own archive
  works, with layer cleanup in some files. This is the main-format gap that keeps the stage at alpha.
- **Estimate:** 10–20 h with users' files to test against.

## G10 Launch and stability on real machines

- **Improved locally (2026-10-13):** macOS saves fall back to plain fsync only for an unsupported full flush; failed fallback retains the original save error and temporary-file cleanup. Actual SMB and other launch/platform reports remain unverified.

- **Missing:** the app not starting or staying in the background on some Windows 11 machines (#858, #964 fixed
  2026-10-10, #620, #818), slow pointer on a Linux AppImage (#575), macOS paste from other apps (#954), a
  web-free crash/hang reporter, a field record.
- **Evidence:** open issues; the never-crash lints and `guard` stop panics, not driver, windowing or packaging
  failures.
- **Impact:** an app that doesn't start on someone's machine is 0% for them.
- **Estimate:** 20–35 h, needs reporters' help on hardware we don't have.

## G8 Hardening at scale

- **Completed locally (2026-10-22):** Recovery folder creation is owner-only on Unix; EPS CRLF payload alignment and huge finite number serialization corrected. Existing asset/clean-room/protocol boundaries retained; new CDR parsers remain deferred. [Audit](vectnor/UPSTREAM_AUDIT.md).

- **Completed locally (2026-10-21):** Mesh size validation, polygon count clamps, huge curve bounds, bounded nested EMF tiles, CSV quoting and SVG metadata/font escaping corrected. General importer depth and XML ID work remain open. [Audit](vectnor/UPSTREAM_AUDIT.md).

- **Completed locally (2026-10-17):** save failure dates/private Unix staging, export filename containment, bounded scatter ticks/Liquify work, pre-decode DIB dimensions, patched Zstandard decoder and MCP message/result/envelope guards. Cross-importer depth, ID allocation/batch rollback and desktop control scheduling remain open; [audit](vectnor/UPSTREAM_AUDIT.md).

- **Missing:** a corpus of real-world SVG/PDF/EPS files; more Affinity versions and platforms (51 pinned files
  now, including 28 original Affinity 3.2.3 feature probes; large interacting documents and complex typography
  missing); Windows, Linux and browser QA by hand; packaging polish (Homebrew cask #588, Flathub #554, update
  checks #615).
- **Impact:** unknown failures on users' files.
- **Estimate:** 30–50 h (the corpus and QA part blocks beta).

## G11 Correctness bugs in core tools

- **Missing:** Shape Builder still misses some planar regions and targets whole shapes (#937); merged line-grid seams (#893) and the recorded loose-end cutting regression are fixed; Transform effect dialog's invisible checkboxes and missing options (#885);
  PDF text boxes moving on open (#722); SVG units reverting to points (#864); bezier drag preview freezing (#834).
- **Resolved regression (2026-10-12):** reviewed upstream #1081/#1230 join shared planar edges and close the persisted `pieces_cut_out_of_their_inputs` failure. The retained seed and original loose-end/grid tests pass with 2,000 property cases. Some near-horizontal loose-end cases remain, as documented in [the upstream audit](vectnor/UPSTREAM_AUDIT.md).
- **Impact:** wrong output in everyday work.
- **Estimate:** 8–15 h.

## G4 Advanced type

- **Completed locally (2026-10-22):** Ordered Bengali Backspace/text and editing-key/IME input, AltGr command suppression, deterministic half-form eligibility and longer-thread release are regression covered. Comprehensive Indic IME/export fidelity remains unassessed. [Audit](vectnor/UPSTREAM_AUDIT.md).

- **Completed locally (2026-10-21):** Indic cluster hit testing, ZWNJ fallback and grapheme-safe Left/Right/Delete receive focused regressions; empty-text style and default style resets corrected. Comprehensive IME/export parity remains unassessed. [Audit](vectnor/UPSTREAM_AUDIT.md).

- **Completed locally (2026-10-13):** character-style capture target keys and explicit Auto-leading resets, with preserved local overrides and native round-trip/history regression coverage. Indic document text remains required.

- **Missing:** hyphenation and justification options, Optical Margin Alignment (type core); CJK vertical
  composition (ruby, mojikumi sets and dialog, proportional vertical metrics #966, kinsoku settings #633, manual
  tate-chu-yoko), Middle Eastern features beyond bidi, variable font axes, spell check with an open dictionary,
  Touch Type, Retype, Snap to Glyph. Itemized in [type-parity.md](type-parity.md).
- **Impact:** professional typography and Japanese/Chinese publishing.
- **Estimate:** 34–47 h (type core 14–19 h blocks beta).

## G12 Preferences and panels that don't act yet

- **Missing:** ~17 Preferences options stored but not acted on (#394: Highlight Alternate Glyphs, Open
  Documents As Tabs, Real-time Drawing, hyphenation language, Appearance of Black, devices…), Properties panel per
  context, context menus beyond the canvas, Layers panel row menus and Collect for Export, SVG Interactivity.
- **Impact:** settings that silently do nothing erode trust.
- **Estimate:** 13–20 h.

## G2 Photoshop-style raster effects and the Effect Gallery

- **Missing:** 29 of 57 filters (all of Artistic and Sketch), the Effect Gallery dialog, Load
  Texture for Glass and Texturizer, Smart Blur's Edge Only and Overlay Edge. Measured in
  [effects-parity.md](effects-parity.md).
- **Done:** the pixel-filter pipeline and 28 filters (Blur, Brush Strokes, Distort, Pixelate, Sharpen, Stylize,
  Texture, Video), Brush Strokes on 2026-10-11.
- **Estimate:** 24–40 h (measured 0.7–1.2 h per filter); parallelizes well.

## G13 Illustrator 2026 (30.x) additions

- **Missing:** gradient dither and perceptual blending, gradient presets (30.0); tangent and perpendicular
  snapping and the consolidated snapping options (30.0); artboard locking, artboard colours and managing
  artboards from the canvas (30.0); the Blend panel with ease in/out and separate colour acceleration (30.7);
  TIFF in Export for Screens (30.2); the Dimension tool (28.x).
- **Evidence:** Adobe's 30.0 and 30.7 release announcements, see
  [target-app-parity.md](target-app-parity.md#how-this-was-measured-2026-10-10).
- **Estimate:** 15–25 h.

## G14 Pen hardware beyond Windows pressure

- **Missing:** pen pressure on macOS (#852) and Wayland (#491, #764) — winit 0.30 reports none there; tilt,
  bearing and barrel rotation for brushes (6D Art Pen, #372); the eraser end.
- **Estimate:** 8–16 h, partly waiting on winit 0.31; needs pen hardware to verify.

## G5 Brushes, symbols and libraries

- **Missing:** the Art brush's Overlap, auto-generated Pattern brush corners, new tile art in Pattern Brush
  Options, the Key Color eyedropper; original brush, symbol and graphic-style libraries generated in code (never
  Adobe's); dynamic symbols; Start Global Edit.
- **Estimate:** 17–28 h.

## G3 3D and Materials

- **Done:** an initial live Revolve in the `three-d` crate (#846, from the contributor who offered in #605).
- **Missing:** Extrude & Bevel, Inflate and Rotate, caps, materials and better lighting, exact visibility for
  intersecting surfaces, 3D Classic, Turntable, 3D export (OBJ, USDA, glTF).
- **Impact:** specialist, but a visible menu of stubs.
- **Estimate:** 45–75 h, the largest single gap; plan in `plan/` before coding.

## G7 Views and windows

- **Missing:** multiple windows and Window › Arrange (Cascade, Tile, Float in Window, Consolidate All Windows),
  panels on another monitor, New View/Edit Views, global and video rulers.
- **Estimate:** 15–25 h.

## G6 Automation and scripting

- **Missing:** a scripting surface over the command registry (Illustrator: JavaScript), Batch, image and graph
  variable kinds, dataset import/export. Actions, Variables (text and visibility) and full agent control over
  MCP are done.
- **Estimate:** 8–14 h.

## G15 Performance budgets and a GPU renderer

- **Measured locally (2026-10-22):** workflow/EPS changes pass 8/10 optimized-development budgets before/after. 400% render 22.16 ms (16 ms limit) and native open 335.00 ms (300 ms limit) remain over. A busy trial was excluded; final load 8.1 and unknown baseline load prevent speed-change conclusions. [Audit](vectnor/UPSTREAM_AUDIT.md).

- **Measured locally (2026-10-21):** reviewed Indic/editing/export changes retain 8/10 optimized-development budgets. 400% render21.46ms (16ms limit) and native open306.05ms (300ms limit) remain over. Differing load averages and single runs do not establish a speed change. [Audit](vectnor/UPSTREAM_AUDIT.md).

- **Measured locally (2026-10-17):** the reviewed file/tool safety update passes 8/10 optimized-development 50k-path budgets before/after. The same 400% render (21.60 ms vs 16 ms) and native open (321.04 ms vs 300 ms) budgets remain over; load average is unknown, so no speedup is claimed. [Evidence](vectnor/UPSTREAM_AUDIT.md).

- **Missing:** an idle-machine run of `vectnor-cli perf` (last: 2026-10-01 on a loaded machine, 290 ms for
  a 50k-path fit against a 16 ms budget), fixes for any budget missed, dirty-region rendering, the GPU backend
  spike (M5).
- **Estimate:** 15–25 h (the budget run and fixes block beta).

## G16 Interface localization — outside current scope

- **Owner decision (2026-10-12):** English-only interface. International UI variants and the former
  70–110 h translation backlog are excluded from active work; this stable gap id is retained as history.
- **Still required:** Indic document text support, including Unicode input, shaping, editing and native
  save/reopen. Font-dependent shaping tests cover nine scripts when suitable fonts are available;
  comprehensive Indic IME, caret/selection and export fidelity remain to be assessed under G4.
- **Evidence:** [language scope](localization-parity.md), `crates/text/src/tests_indic.rs`,
  `crates/engine/src/cmd/typecmd.rs`, English-only menu and legacy-preference tests in `ui-egui`.

## G17 Ecosystem: third-party plug-ins

- **Missing:** Illustrator's ecosystem is a C++ plug-in SDK and a large commercial plug-in market. Vectnor
  has sandboxed WebAssembly plug-ins for object filters and live effects ([plugins.md](plugins.md)) and MCP;
  missing are tool plug-ins, panel plug-ins, a plug-in directory and any third-party plug-ins (#710).
- **Estimate:** 20–45 h for the APIs; the ecosystem itself isn't agent work.

## G18 Generative AI features

- **Missing:** Text to Vector Graphic, Generative Recolor, Generative Shape Fill (#766), Generative Expand,
  Concept to Vector, Prompt to Edit, Rewrite Text. Adobe's run on its cloud models; ours would be a pluggable
  provider.
- **Needs:** an owner decision on models and providers.
- **Estimate:** 30–60 h after that decision.

## G19 Accessibility

- **Missing:** AccessKit is on by default, so egui widgets reach screen readers, but custom panels, the canvas and its objects aren't described and nothing has been tested with VoiceOver, Narrator or Orca; keyboard-only canvas operation; high
  contrast. Illustrator's own accessibility is limited, so this is mostly beyond parity.
- **Estimate:** 15–25 h.

## Revision history

| Date | Change | Summary |
|---|---|---|
| 2026-10-22 | minor | Nineteen reviewed upstream scopes for ordered Indic/AltGr input, threaded text, brush references, clipping, EPS/CSS, exact-size shapes, Recent tabs and panel scrolling. Scores unchanged; no upload. [Audit](vectnor/UPSTREAM_AUDIT.md). |
| 2026-10-21 | minor | Thirty-two reviewed local upstream adaptations: Indic caret/joiners, style resets, selection, bounded mesh/EMF processing, CSV/SVG and raster-scale export correctness. Scores unchanged; no upload. [Audit](vectnor/UPSTREAM_AUDIT.md). |
| 2026-10-17 | minor | Eighteen reviewed upstream adaptations for save/export safety, bounded processing, automation framing and editing; scores unchanged. [Audit](vectnor/UPSTREAM_AUDIT.md). No upload. |
| 2026-10-13 | minor | Reviewed upstream correctness pass: twenty updated adaptations and two alignment dependencies; English interface/Indic text retained; scores unchanged. See the [audit](vectnor/UPSTREAM_AUDIT.md). |
| 2026-10-12 | scope | English-only interface by owner request; Indic document text retained with nine-script shaping and edit/undo/redo/native round-trip checks. Historical readiness estimates unchanged pending reassessment. |
| 2026-10-12 | minor | Optional Alt/Option smoothing from Pencil/Paintbrush borrows the saved Smooth tool, preserves Cmd selection priority and restores the original tool on release/cancel. Scores unchanged. |
| 2026-10-12 | minor | Pencil/Paintbrush Keep selected is configurable and saved per tool; previews remain selected, completed/continued strokes follow the setting with one undo step. Scores unchanged. |
| 2026-10-12 | minor | New ruler guides switch orientation with Alt/Option, including stationary-pointer previews and snapping on the effective axis; one undo step. Scores and estimates unchanged. |
| 2026-10-12 | minor | Nine reviewed upstream fixes adapted: Shape Builder seams/cutting, EPS angle hangs, renderer safety, saved views, MCP no-replay, stroke visibility, DXF closed splines and stable generated sources. Pagenor structure/provenance conventions aligned; scores unchanged. |
| 2026-10-12 | minor | G11: recorded reproducible planar open-line cutting property failure discovered during branding validation |
| 2026-10-11 | minor | G9: legacy `.ai` without the prolog opens with its layers (#1027) |
| 2026-10-11 | minor | G9: `.ai` pattern fills and global colours read from the editing data (#1025) |
| 2026-10-10 | minor | G3: initial Revolve landed (#846), 45–75 h left |
| 2026-10-10 | minor | G9 evidence counted from the tracker (28 `.ai`/EPS issues, 20 fixed); impact narrowed to handing files back |
| 2026-10-10 | minor | Alpha blockers checked against the core-workflow gate: none |
| 2026-10-10 | major | Moved from the ROADMAP's "Where we're lacking"; kept ids G1–G8, added G9–G19 (file formats, stability, bugs, preferences, 30.x additions, pen hardware, performance, localization, ecosystem, AI, accessibility) and re-ranked |
