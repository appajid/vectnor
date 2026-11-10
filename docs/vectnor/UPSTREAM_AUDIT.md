# Upstream review

> **Last reviewed:** 2026-11-11 · **Last updated:** 2026-11-11 · **Change:** minor (reviewed AI style, parsing and release-position fixes; scores unchanged)

## 2026-11-11 local update — M0.upstream-import-robustness

Screened **1,501 PRs updated since the previous capture** (2026-10-31 11:43:55 UTC):
**1,226 merged, 198 open and 77 closed without merge**. Capture: 2026-11-10T20:49:04.621639+00:00.
Sixteen public REST pages held 1,600 distinct records; page sixteen crossed the cutoff.
Only [VectorCraft](https://github.com/storytold/vectorcraft) was checked. Metadata screening is
not full source review: scoped 29 candidate diffs and adapted **15 merged PR scopes**. Verified
33 Rust postimage blob identities, including reviewed files from deferred scopes not copied.
The [dated snapshot](upstream-audit-2026-11-11.json) pins authors, heads, raw diff SHA-256,
postimages and every screened record/decision. Existing MIT OR Apache-2.0 notices remain.

| PR | Source author / immutable head | Adapted scope |
|---|---|---|
| [#5374](https://github.com/storytold/vectorcraft/pull/5374) | @dylanmtaylor / `9d3c45a7b563` | Refuse editing-data arrays nested past the existing 256-depth limit; discard stale operand marks before checking nesting. |
| [#4945](https://github.com/storytold/vectorcraft/pull/4945) | @dotCooCoo / `d2d431552449` | Keep a zero byte that belongs to a Zstandard frame; identify padding only after consuming complete frames. Existing frame/window/output caps remain. |
| [#4801](https://github.com/storytold/vectorcraft/pull/4801) | @mockydevs / `3602bd6ea358` | Read character-style sheet inheritance in AI editing data, giving run attributes precedence over based-on sheets and document defaults. Cap sheets at 4096 and inheritance at 16 links; preserve original frame identities, geometry and linked-story recovery. |
| [#5497](https://github.com/storytold/vectorcraft/pull/5497) | @dylanmtaylor / `daf224755a83` | Cap localized font-name enumeration at 1024 records and 1 MiB decoded text per name ID in the existing font database. No font files added. |
| [#5924](https://github.com/storytold/vectorcraft/pull/5924) | @jayzed80 / `7d465d4fa356` | Stop area-layout baseline searches that cannot advance at extreme gutters. Fit to the existing stepping loop and shared gutter model; newer row-gutter API not imported. |
| [#6128](https://github.com/storytold/vectorcraft/pull/6128) | @Silvertree2010 / `194364304bec` | Strip CSS comments from inline style declarations and rules while retaining comment markers inside quoted strings; retain prior semicolon/parenthesis handling. |
| [#5107](https://github.com/storytold/vectorcraft/pull/5107) | @TheGalitube / `87979644cddf` | Decode paired DXF UTF-16 surrogate escapes as one Unicode character and retain malformed pairs literally under the existing plain/MTEXT parser. |
| [#6253](https://github.com/storytold/vectorcraft/pull/6253) | @Silvertree2010 / `5a4772d24a38` | Require plain decimal digits in MCP object-resource IDs; signed, empty and overflowing IDs remain invalid. Vectnor URI identity retained. |
| [#5473](https://github.com/storytold/vectorcraft/pull/5473) | @dotCooCoo / `1ad74d1ddffa` | Reject more than one leading sign in OpenType feature tags. Existing supported-feature set retained; refused edits leave text and undo history unchanged. |
| [#5328](https://github.com/storytold/vectorcraft/pull/5328) | @dotCooCoo / `c633e09d3ad9` | Do not select empty text runs for Select Same character matching. Regression covers the existing Fill Color and Stroke Color commands; newer combined command is absent. |
| [#6096](https://github.com/storytold/vectorcraft/pull/6096) | @Silvertree2010 / `b73a880b3cb1` | Truncate PSD Unicode layer names before a character would exceed 255 UTF-16 units, retaining whole surrogate pairs. |
| [#5299](https://github.com/storytold/vectorcraft/pull/5299) | @dylanmtaylor / `1bd47ea30d9e` | Reject repeated ICC tag signatures before profile decoding. Fit to the existing ICC reader; newer localized-text budget infrastructure not imported. |
| [#6225](https://github.com/storytold/vectorcraft/pull/6225) | @slafebre / `b34de477a8b1` | Open AI/AI-template editing-data documents without an Import PDF dialog. Add a metadata-only private-key query using the existing bounded PDF dictionary walk; no stream inflation for the query. PDFs, password-protected files, AI files containing only PDF pages, and Place retain their dialog behavior. The newer shared PDF-reader API is not imported. |
| [#5198](https://github.com/storytold/vectorcraft/pull/5198) | @dotCooCoo / `2a41c746900b` | Reject unclosed document groups and compound paths. Use the existing pattern flag to leave pattern subreaders their own balance check; retain the existing unclosed-layer error. |
| [#5195](https://github.com/storytold/vectorcraft/pull/5195) | @dotCooCoo / `ac298b683b22` | End canvas tool drags at the actual button-up position, before subsequent hover moves in the same frame. Feed the release position to the existing preview before Up only for a completed egui drag; ordinary clicks keep their behavior. No stylus/multi-sample pipeline migration. |

The AI character-style change resolves run overrides before inherited sheets and document defaults;
it preserves the local original-frame/linked-story recovery path. Inheritance and name decoding are
bounded. The Import PDF dialog is skipped only when opening an AI/AI-template document carrying
editing-data keys or a native editing copy. A protected PDF still asks for its password, and ordinary
PDF/AI-page selection and Place retain the dialog. The local metadata query uses the existing bounded
dictionary walk without decompressing private streams; the newer shared PDF-open API is deferred.
The canvas change fits the existing egui 0.36 preview/Up contract; newer stylus sampling is deferred.

Deferred scoped chains: per-character language-aware shaping (#4959) and Indic hyphenation with
external patterns (#5946), legacy tab stops and layout (#5811/#5885/#6148), newer AI baseline,
filled clipping paths, smooth anchors and default paragraph sheets (#4791/#6032/#5122/#5777),
DocSetup normalization (#4952), the newer Unicode-code insertion API (#6240), pixelated-image
model/rendering (#4888), mask/pattern mode routing (#4458), and stylus sample filtering (#5268).
Other records remain metadata-only with decisions in the snapshot. No new format or broader
AI/CDR/DWG fidelity claim. Earlier adaptations are not counted again.

English UI, Indic document text, original AI boxes/stories, `.vcnr`, source-overwrite protection,
overflow notices, Contributors and Models remain. No new dependency, fonts, artwork, pattern assets,
community controls or publication workflow. Owner AGENTS.md/CLAUDE.md edits preserved; no Pagenor
messages. Version, estimates and scores unchanged. No source/build upload.

Verification: **all eight CI gates pass — 5,345 tests passed, zero failed, four existing ignored**;
UI suite: 947. Strict Clippy, attribution, branding, dependency layers and core WASM checks pass.
Optimized-development 0.9.0 native/CLI build and actual browser WASM check pass; the latter retains
the existing unused `ask_update_changed` warning in unchanged `missing_links.rs`. Binaries in
`dist/Vectnor.app` are refreshed and SHA-256 checked against the completed build.

Both supplied AI files retain exactly their previous CLI summaries and full command/layer JSON.
Desktop File Open skips the Import PDF dialog and retains ten original announcement text objects,
two linked pairs and four point-text objects in the three-lines file. Standalone characters,
complete linked stories and frame geometry are exact; font-dependent reflow between existing
linked frames is allowed. Whole-paragraph Type selection, Telugu/ZWNJ insertion, neighboring-frame
isolation and exact undo/redo pass. Native save/reopen retains node/thread JSON, both artboard PNG
hashes and dirty=false. Three overflow frames reported by File Open match actual current-font area
queries; missing fonts remain missing. Synthetic regressions cover inherited character styles,
bounded cyclic inheritance, invalid inputs, Unicode names, extreme gutters, PDF-dialog exceptions
and rectangle/pencil/selection releases. Screenshots inspected; isolated app on port 18008 quit.
Both original AI sources and owner instruction-file edits are SHA-256 unchanged. Evidence:
`/tmp/vectnor-upstream-2026-11-11/`. Version and scores unchanged; nothing uploaded.

## 2026-10-31 local update — M0.upstream-import-editing

Screened **687 PRs updated since the last capture** (2026-10-25 14:49:05 UTC): **445 merged,
199 open and 43 closed without merge**. Capture: 2026-10-31T11:43:55.000294+00:00. Eight public REST pages
held 800 distinct records; page seven crossed the cutoff and page eight verified the older range.
Only [VectorCraft](https://github.com/storytold/vectorcraft) was checked. Metadata screening is not
full source review: scoped 24 candidate diffs, adapted **14 merged PR scopes** after 27 Rust
postimage blob checks (including one reviewed RTF file not copied).

The [dated snapshot](upstream-audit-2026-10-31.json) pins authors, heads, raw diff SHA-256, verified
Rust postimages and every screened record/decision. Existing MIT OR Apache-2.0 notices remain.

| PR | Source author / immutable head | Adapted scope |
|---|---|---|
| [#4549](https://github.com/storytold/vectorcraft/pull/4549) | @bflatastic / `e0c92a4d906e` | Keep a base letter and combining marks in one fallback face using grapheme clusters. Fit to existing source-run attribution, joiner handling and unweighted fallback API; no numeral-system or newer font-style API imported. Cap candidate lookup to 64 distinct drawn characters per cluster. Synthetic fonts test Vietnamese marks and Indic face selection; no font files added. |
| [#4250](https://github.com/storytold/vectorcraft/pull/4250) | @DCaioP / `311b34321932` | Text creation and family-only style changes store an available closest style when the family has no Regular. Explicit styles and missing-family requests remain unchanged. Extract existing style-fit helper; synthetic numbered family from existing licensed font bytes. |
| [#4708](https://github.com/storytold/vectorcraft/pull/4708) | @mockydevs / `530605d4fc86` | AI editing-data character-style key 12 retains All Caps and Small Caps while preserving original characters. Small Caps uses the existing document default size; custom source size remains unread. |
| [#4179](https://github.com/storytold/vectorcraft/pull/4179) | @youwishgames / `9af023b084b4` | Read an undefined radial-gradient highlight as no highlight and recognize XC colour-space metadata. Reject undefined numbers in other shown-art operands, including nested arrays via an iterative scan. Original text-frame recovery and native-first import remain. |
| [#4553](https://github.com/storytold/vectorcraft/pull/4553) | @TheGalitube / `2b37302fe5f7` | Keep CSS comment markers inside quoted strings during stylesheet parsing. |
| [#4230](https://github.com/storytold/vectorcraft/pull/4230) | @Silvertree2010 / `29a98b750ac9` | Split CSS declarations on semicolons only outside quotes and parentheses, preserving font names and data URLs. |
| [#4514](https://github.com/storytold/vectorcraft/pull/4514) | @TheGalitube / `c714797e99c8` | DXF integer parsing accepts whole-number floats and rejects fractional values instead of truncating them. |
| [#4460](https://github.com/storytold/vectorcraft/pull/4460) | @Silvertree2010 / `ddd017eb0a5d` | PostScript radix numbers refuse a sign after #. |
| [#4449](https://github.com/storytold/vectorcraft/pull/4449) | @Silvertree2010 / `aca98ffe7952` | EPS Unicode glyph names require hexadecimal digits. |
| [#4321](https://github.com/storytold/vectorcraft/pull/4321) | @Silvertree2010 / `c2fb35b24485` | ColorKey formatting handles saturated negative Lab values without signed absolute-value overflow. |
| [#4409](https://github.com/storytold/vectorcraft/pull/4409) | @Silvertree2010 / `de6b14b5417e` | MCP line reader accepts exactly the content-byte cap followed by CRLF; overlong content remains refused. |
| [#4288](https://github.com/storytold/vectorcraft/pull/4288) | @Silvertree2010 / `76e0e5b0ac1a` | MCP resource percent decoding requires two hexadecimal digits and retains malformed escapes literally; Vectnor URI identity unchanged. |
| [#4540](https://github.com/storytold/vectorcraft/pull/4540) | @bflatastic / `44ef4ab4bc16` | Align the canvas input rectangle to device pixels before fitting and existing custom scrollbar layout, stabilizing pointer mapping under subpixel dock widths. Keep egui 0.36 and the local scrollbar module. |
| [#4580](https://github.com/storytold/vectorcraft/pull/4580) | @bflatastic / `9534a364162f` | New Layer and New Sublayer are refused while editing an opacity mask. Guard uses existing mask-edit state and retains current layer command options. |

The cluster fix preserves the current grapheme caret, shaping-span source attribution and joiner
rules. Its candidate search is bounded; glyph selection tests with synthetic Indic cmaps establish
whole-cluster font choice, not Indic visual fidelity of those synthetic Latin outlines. Existing
Indic layout/editing tests remain. No numeral-system model, weighted fallback API, new fonts or
interface translations imported. The capitalization reader keeps source characters; custom source
Small Caps sizes remain unread. Undefined gradient placeholders are permitted only for the known
highlight operator; invalid use in shown art, including nested arrays, still rejects native artwork.

Deferred source scopes: newer clipboard/IME changes (#4608), numeric-field commit changes (#4213),
standalone RTF reader (#4330, absent here), Swatches document-focus tracking (#4733), Eyedropper
picked/default paint routing (#4583), broad dropdown UI (#4337), soft-return model changes (#4584),
and AI mesh/layer-state/placed-file chains (#4755/#4740/#4748). Other updates remain metadata-only,
with per-record decisions. Larger AI capacity limits, new CDR/DWG importers, add-ons, auto-updating,
platform/library migrations and translation catalogs are not copied. Earlier adapted PRs are not
counted again. No claim of full AI/CDR/DWG compatibility or improved performance.

Native-first AI opening, original text boxes/linked stories, source-overwrite protection, overflow
warnings, English UI, Indic text, `.vcnr` branding and existing Contributors/Models remain. No
Pagenor messages or other upstream project review. Owner instruction-file edits are preserved.
Scores, estimates and version unchanged. No source/build upload.

Verification: **all eight CI gates pass — 5,327 tests passed, zero failed, four existing ignored**;
UI suite: 943. Optimized-development 0.9.0 native/CLI build and actual browser WASM check pass.
The latter retains the existing unused `ask_update_changed` warning in unchanged `missing_links.rs`.
Binaries in `dist/Vectnor.app` are refreshed and SHA-256 checked against the completed build.

Both supplied AI files retain exactly their previous CLI summaries and full command/layer JSON.
Normal desktop File Open retains ten original text objects, two linked pairs, standalone characters,
complete linked stories and original frame geometry. Desktop font metrics redistribute the speaker
story between its existing linked frames relative to the CLI; concatenated characters are exact.
Whole-paragraph Type selection, Telugu/ZWNJ insertion, neighboring-frame isolation and exact
undo/redo pass. Native save/reopen retains node/thread JSON, both artboard PNG hashes and dirty=false.
Three overflow frames reported by File Open match actual current-font area queries; missing fonts
remain missing. A synthetic native AI fixture retains source strings plus All Caps/Small Caps flags
through native save/reopen, independently of its deliberately unrelated PDF drawing. Mask-editing
layer creation is refused without model/history changes and works after leaving. Screenshots
inspected; isolated app on port 18007 quit. Both original AI sources and owner instruction-file edits
are SHA-256 unchanged. Evidence: `/tmp/vectnor-upstream-2026-10-31/`. Nothing uploaded.

## 2026-10-25 local update — M0.upstream-ai-text

Screened **541 PRs updated after the last capture** (2026-10-22 11:47:26 UTC): **309 merged,
137 open and 95 closed without merge**. Capture: 2026-10-25 14:49:05 UTC. Seven public REST pages
held 700 distinct records; page six crossed the cutoff and page seven verified no newer records.
Review is limited to [VectorCraft](https://github.com/storytold/vectorcraft). Metadata screening is
not a full source review: 23 candidate diffs were scoped and **eight merged PR scopes** adapted.

The [dated snapshot](upstream-audit-2026-10-25.json) records all screened records and decisions,
authors, immutable heads, raw diff SHA-256 and 21 verified Rust postimage blob identities (including
reviewed source/test files excluded from adaptation). Existing MIT OR Apache-2.0 notices remain.

| PR | Source author / head | Local scope and compatibility |
|---|---|---|
| [#3941](https://github.com/storytold/vectorcraft/pull/3941) | @fabian-co / `9090afea1c99` | Read AI palette swatches, unused spot/global colours, colour groups, gradients/pattern references and Registration operators. Fit to existing lexer/reader; use a running entry count to avoid repeated group scans. Swatches-panel selection retention and newer fuzz fixture are excluded. |
| [#3927](https://github.com/storytold/vectorcraft/pull/3927) | @bflatastic / `76c9aa579de6` | Mark oversized editing-data images as incomplete, reject that native-first candidate and retain the visible PDF artwork with existing original text-frame recovery. Without usable PDF artwork retain existing partial-import warnings. Current layered_ai API preserved. |
| [#3685](https://github.com/storytold/vectorcraft/pull/3685) | @ham9zah / `1abc5d4066d3` | Keep wrapped/unwrapped end brackets of closed-path AI text and finite defaults; preserve clamping on open paths. Existing ATE paths already close matching endpoints, and the older native planner does not overwrite them with PDF paths, so the newer native.rs helper is unnecessary. |
| [#3755](https://github.com/storytold/vectorcraft/pull/3755) | @fortesnow / `bfceca7b4d1a` | Resolve imported family/style pairs split from a PostScript name once that exact PostScript face is installed. Retain current canonical-name API. |
| [#3861](https://github.com/storytold/vectorcraft/pull/3861) | @FractalEncrypt / `25abd1f4728f` | Recheck loaded fallback faces after concurrent system lookup, even when this thread added no face. Retain existing fallback signature without newer weight/italic parameters. |
| [#3921](https://github.com/storytold/vectorcraft/pull/3921) | @Taha-Mahmoodi / `6b52e54a1b81` | Publish all faces of a catalogued family atomically and deduplicate under the write lock. Deterministic publication-hook regression uses only existing licensed font bytes. |
| [#3827](https://github.com/storytold/vectorcraft/pull/3827) | @dotCooCoo / `0083587131b9` | Converting a linked area-text frame to point type removes its own characters from the remaining thread, preserving other frames and undo. Tests use the existing selection-based text.thread.create API; no new threading command. |
| [#3735](https://github.com/storytold/vectorcraft/pull/3735) | @Silvertree2010 / `ffd831761e96` | Limit in-memory font-collection face enumeration to the existing 256-face file-scan cap; damaged collection regression. |

The AI importer still prefers complete original editing data independently of the PDF drawing.
Oversized-image omissions now make that candidate incomplete; available PDF artwork is retained,
with the existing conservative original-frame recovery. Without usable PDF artwork, partial native
imports retain omission warnings. No larger image allocation limit was introduced. Closed-path
ranges use the existing ATE geometry, without replacing original area-text boundaries or baselines.
Lab inks also require the small named-colour Lab branch from the same verified #3941 postimage,
recorded as a prerequisite outside that PR diff. Swatch parsing keeps a running entry count, capped at 20,000 entries and 20,000 groups, and 64
operand values; unknown palette entries do not prevent supported colours from importing.

Deferred source scopes: connected AI meshes (#3811) need a newer reader; paragraph continuation
(#4031/#4072) spans the model, layout, copies and recovery; outline/raster/symbol/transparency thread
exits (#3977/#4047/#4059/#4039) need their shared identity and overflow helpers. PDF glyph placement
(#3715/#3641/#3742/#3753) and PostScript interpreter/resource changes (#3771/#3816/#3821/#3810) remain
separate compatibility passes. The Swatches-panel retention portion of #3941 is excluded. Previously
adapted PRs are not counted again; metadata-only re-updates do not claim fresh source verification.
Other PRs remain metadata-only, with per-record reasons. No CDR/DWG or broader AI compatibility claim.

English UI, Indic document text, `.vcnr` identity, overflow notices, original AI frame recovery and
source overwrite protection remain. No dependencies, fonts, artwork, contributor profiles, community
controls, services or publication workflows added. This update is separate from Pagenor, with no
cross-chat messages. Owner edits to AGENTS.md and CLAUDE.md remain untouched. No upload.

Verification: **all eight CI gates pass — 5,306 tests passed, zero failed, four existing ignored**;
UI suite: 942. Actual browser WASM check passes with the existing unused `ask_update_changed`
warning in unchanged `missing_links.rs`. The final unseeded property suite completed in 135.52 s;
a retained process sample showed active unchanged Pathfinder work. No timeout, skip or seed change.

Optimized-development 0.9.0 native/CLI binaries refreshed in `dist/Vectnor.app` and SHA-256 checked.
Both supplied AI files retain exactly their previous CLI summaries; the announcement's full layer
JSON, linked threads, inspection and warnings also match. Normal desktop File Open checks ten text
objects, two linked pairs, exact source stories, whole-paragraph selection, Telugu edit/undo/redo,
linked edit/undo and linked-frame Convert To Point Type with undo/redo. Native save/reopen preserves
node/thread JSON and both artboard PNG hashes, with dirty=false. Overflow notes match actual area
queries. An independent palette fixture retains an unused Lab ink, its colour group and Registration.
Screenshots inspected; the isolated app on port 18006 quit. Missing fonts remain missing.
Evidence: `/tmp/vectnor-upstream-2026-10-25/`. Version, scores and estimates unchanged; nothing uploaded.

## 2026-10-22 local update — M0.upstream-workflow

Screened **329 PRs updated since `fcfa46a`** (2026-10-21 08:05:34 UTC): **211 merged,
106 open and 12 closed without merge**. Four public REST pages held 400 distinct records and crossed
the cutoff; a fifth 100-record verification page contained no post-cutoff updates. Capture time:
2026-10-22 11:47:26 UTC. Review is limited to PRs targeting the [upstream repository](https://github.com/storytold/vectorcraft).
Metadata screening is not full source review. Scoped 40 candidate diffs (including six prior adaptation
rechecks), and adapted **19 PR scopes: 18 merged and one open**, including one regression-only PR.

The [dated snapshot](upstream-audit-2026-10-22.json) pins author logins, exact heads, raw diff SHA-256,
41 verified Rust postimage Git blob IDs, compatibility decisions and every metadata-screened record.
Older snapshots remain unchanged. Existing upstream MIT OR Apache-2.0 attribution is retained.

| PR | State at capture | Author / immutable head | Local adaptation |
|---|---|---|---|
| [#3382](https://github.com/storytold/vectorcraft/pull/3382) | merged | @dotCooCoo / `76058fb1b557` | AltGr characters typed while editing text do not also run command chords. |
| [#2636](https://github.com/storytold/vectorcraft/pull/2636) | open | @dotCooCoo / `2adce89eb1aa` | Process typed text, IME, editing keys, Enter and clipboard events in their received order; fitted to current shortcut router, with command-chord text suppression from the pinned postimage. |
| [#3376](https://github.com/storytold/vectorcraft/pull/3376) | merged | @dotCooCoo / `da197ba75c1b` | Half-form test selects only a capable fallback face; build positive/negative GSUB fixtures in code from an existing OFL font. |
| [#3409](https://github.com/storytold/vectorcraft/pull/3409) | merged | @lehni / `23dbf5d214aa` | Releasing a middle threaded-text frame preserves one story; regression uses existing thread.create because thread.add is absent. |
| [#3385](https://github.com/storytold/vectorcraft/pull/3385) | merged | @dotCooCoo / `4b56ce44a161` | Brush rename/delete reaches masks, symbols, pattern tiles and retained pattern originals, graphic styles and brush art. Graph designs are absent; existing single-rename contract retained. |
| [#3206](https://github.com/storytold/vectorcraft/pull/3206) | merged | @Taha-Mahmoodi / `4fdb3e1f9b29` | Arrange art inside a clipped group/layer without moving the bottom clipping path. |
| [#3614](https://github.com/storytold/vectorcraft/pull/3614) | merged | @bflatastic / `76399175ebb3` | EPS readline consumes CRLF as one terminator, preserving counted and binary payload alignment. |
| [#3240](https://github.com/storytold/vectorcraft/pull/3240) | merged | @Silvertree2010 / `2b0d71dbea04` | EPS number serialization avoids accidental inf for huge finite values; format numeric-range validation is separate. |
| [#3284](https://github.com/storytold/vectorcraft/pull/3284) | merged | @MMexicann / `97d133d9d408` | Preserve fractional CSS gradient-stop percentages. |
| [#3340](https://github.com/storytold/vectorcraft/pull/3340) | merged | @dotCooCoo / `ba8c1e042c3d` | Zero Edit Selected Paths/Close Paths distances disable endpoint continuation/automatic closure for Pencil/Paintbrush. |
| [#3257](https://github.com/storytold/vectorcraft/pull/3257) | merged | @FractalEncrypt / `b9fa08f9e6eb` | Remember Polygon/Star exact-size counts through the existing tool.setOption command. |
| [#3348](https://github.com/storytold/vectorcraft/pull/3348) | merged | @MarekSpodBiedry / `cde39698ffa7` | Alt-click exact-size rectangles and ellipses use the clicked centre. |
| [#3502](https://github.com/storytold/vectorcraft/pull/3502) | merged | @nrq6993-a11y / `e4db022f494a` | Welcome recent thumbnails and Open Recent activate an existing tab, preserving unsaved edits, instead of opening another copy. |
| [#3460](https://github.com/storytold/vectorcraft/pull/3460) | merged | @dotCooCoo / `4bd55029e8d6` | Live Paint face picking/filling skips hidden faces. Existing edge-picking contract retained. |
| [#3406](https://github.com/storytold/vectorcraft/pull/3406) | merged | @oracola91 / `d8e876b721fc` | Create recovery folders owner-only on Unix while preserving modes of pre-existing chosen folders. |
| [#3518](https://github.com/storytold/vectorcraft/pull/3518) | merged | @nrq6993-a11y / `3d4d34fc15e1` | Color-book library tab names follow ordinary file basenames; existing copy behavior retained. |
| [#3534](https://github.com/storytold/vectorcraft/pull/3534) | merged | @nrq6993-a11y / `8735170a3422` | Regression coverage only: filled paths without stroke accept Type on Path/Area Type on their outline, not inside the fill; current implementation already matches. |
| [#3550](https://github.com/storytold/vectorcraft/pull/3550) | merged | @Taha-Mahmoodi / `a8bdf5043f94` | Properties/flyout/docked-column scrollbars reserve a column, using a small scroll-style prerequisite from the same postimage. New shared-panel docking module absent and excluded. |
| [#3554](https://github.com/storytold/vectorcraft/pull/3554) | merged | @Taha-Mahmoodi / `7141932dfb27` | Layers scrollbar reserves its own column and avoids selection/target marks. |

Compatibility and exclusions:

- Ordered input comes from open #2636, explicitly recorded as a reviewed local proposal adaptation.
  Its small shortcut-modifier helper comes from the same pinned postimage; the previous Type/Escape
  claims and grapheme caret behavior remain. English interface, Indic document support and native
  `.vcnr`/`vectnor` identity are preserved. The half-form test creates fonts in memory from existing
  licensed bytes; no font files, assets, translation catalogs or dependency changes are introduced.
- Thread release uses existing Create Thread commands in its regression. Brush rename adapts direct
  mask fields and the existing single-rename API; the newer graph-design model is absent and excluded.
  Native Recent uses the existing activation/sync/home behavior and keeps the thumbnail layout.
  Live Paint keeps current edge-picking semantics. Scroll fixes cover current Properties, flyouts,
  docked columns and Layers; a newer shared-panel docking module is absent and not imported.
- Previously adapted #3230/#3323/#3331/#3337/#3254/#2671 are now merged with rebased heads. Their relevant
  production scopes were rechecked and remain implemented; they are not counted again. Existing
  group/layer-only isolation is retained rather than importing single-object/guide isolation contracts.
- CDR proposals #3123 and #2890 were scoped for architecture, parser boundaries and integration needs.
  They add substantial competing importers and require full untrusted-input review, fuzzing and file
  validation. They remain deferred; **this update adds no CDR support**. Existing supported AI/PDF/EPS
  import remains partial, with format limitations described in [file-format-parity](../file-format-parity.md).
- New save/export extension rules require the newer picker and overwrite-confirmation contracts;
  those are deferred together. Broader paragraph visitation, modal layout, rendering, import/backend,
  platform, scripting and docking changes remain compatibility work. Translations, community controls,
  promotional/service integrations and automatic publication are excluded by owner scope.
- This task stayed separate from Pagenor: no external messages or other project PR review. Owner
  policy edits are preserved, including the authorized public repository status; no source upload.

Verification: **all eight CI gates pass — 5,265 tests passed, zero failed, four existing ignored**.
The UI suite passes 942 tests. The actual browser app also checks for WASM, with the existing unused
`ask_update_changed` warning in unchanged `dialogs/missing_links.rs`.

Native control port 17994 checks Bengali Backspace-plus-replacement in one real UI frame, exact-size
Alt-click centring, Polygon count persistence/reopen/Cancel and Open Recent activation of an existing
dirty document without losing unsaved geometry or adding a tab. Bengali, Telugu, Hindi and joiners
save/reopen as `.vcnr` with identical node JSON and artwork PNG SHA-256; reopened dirty=false.
Inspected Canvas, short-window Layers, English Preferences and About screenshots: scrollbar column,
Vectnor branding, Developer: Appaji Ambarisha and Contributors/Models remain. The test app quit cleanly.
Evidence remains local under `/tmp/vectnor-upstream-2026-10-22/`.

Performance remains **8/10 budgets before and after** on 50k synthetic paths in the optimized-development
build: 400% render 22.35 → 22.16 ms (16 ms limit), native open 314.54 → 335.00 ms (300 ms limit).
An earlier busy trial (load 19.3 on 12 cores) is retained separately and excluded from inference.
The final trial followed completed builds and app exit, at load 8.1; baseline load is unknown.
Single runs with these load differences establish neither a speedup nor a regression; the two overruns
remain open under G15. `dist/Vectnor.app` is refreshed from SHA-256-verified native/CLI binaries.

Scores, estimates and application version remain unchanged. No source, build, PR or hosted app is uploaded.

## 2026-10-21 local update — M0.upstream-indic

Screened **687 PRs updated since `a17ee43`** (2026-10-16 19:11:59 UTC): **394 merged,
260 open and 33 closed without merge**. Ten public REST pages held 1,000 distinct records and
crossed the cutoff. Metadata screening is not full source review. Adapted the selected scopes of
**32 PRs** after checking immutable source postimages against the raw diff's Git blob IDs.
The [dated snapshot](upstream-audit-2026-10-21.json) preserves authors, exact heads, diff SHA-256,
69 verified postimages, the grapheme-helper prerequisite and every metadata-screened decision.
Older snapshots remain unchanged. Retained upstream MIT OR Apache-2.0 attribution.

| PR | State at capture | Author / immutable head | Adapted scope |
|---|---|---|---|
| [#1975](https://github.com/storytold/vectorcraft/pull/1975) | merged | @el-j / `4913418d1cac` | Double-click text returns to the invoking tool on Escape; preserve explicit Type tool behavior. |
| [#2237](https://github.com/storytold/vectorcraft/pull/2237) | merged | @EuanTop / `074651951749` | Native font-family metadata can fall back to localized legacy names; interface remains English. |
| [#2400](https://github.com/storytold/vectorcraft/pull/2400) | merged | @marcoarnulfo / `68c9d440ab1f` | Transform menu flips use the chosen reference point. |
| [#2464](https://github.com/storytold/vectorcraft/pull/2464) | merged | @marcoarnulfo / `63b95c8dd3d1` | Deleting all text retains the insertion character style. |
| [#2501](https://github.com/storytold/vectorcraft/pull/2501) | merged | @dotCooCoo / `b9b681c7487c` | Character/paragraph styles capture and reset default-valued attributes supported by this model. |
| [#2726](https://github.com/storytold/vectorcraft/pull/2726) | merged | @dotCooCoo / `f5d61f6f27e1` | Trim new/duplicate text-style and library style names; symbol subpatch deferred. |
| [#2824](https://github.com/storytold/vectorcraft/pull/2824) | merged | @MarekSpodBiedry / `df1bf3fe3216` | Close a dropdown and release focus after keyboard choice; newer Enter-opening prerequisite deferred. |
| [#2844](https://github.com/storytold/vectorcraft/pull/2844) | merged | @dotCooCoo / `138e9ca46635` | Use a finite control hull for huge or nonfinite Bezier extrema inputs. |
| [#2884](https://github.com/storytold/vectorcraft/pull/2884) | merged | @nrq6993-a11y / `5906f53ca5a9` | Detect fonts activated while the app stays focused; synthetic font-service fixture only. |
| [#2931](https://github.com/storytold/vectorcraft/pull/2931) | merged | @MortenHusted / `eba9abc21e14` | CLI/MCP raster scale does not resize vector DXF exports; explicit DXF options remain supported. |
| [#3230](https://github.com/storytold/vectorcraft/pull/3230) | open | @Silvertree2010 / `13d3ec6891b3` | Measure tool normalizes negative zero angles. |
| [#3323](https://github.com/storytold/vectorcraft/pull/3323) | open | @Silvertree2010 / `83a7277f248b` | Blank GPL palette Name retains the filename fallback. |
| [#3331](https://github.com/storytold/vectorcraft/pull/3331) | open | @dotCooCoo / `88989aeb70c2` | Catch a palette-library reader failure and continue listing other libraries. |
| [#3337](https://github.com/storytold/vectorcraft/pull/3337) | open | @dotCooCoo / `f5c28ef0c91a` | Select bristle strokes by brush type rather than brush name. |
| [#2602](https://github.com/storytold/vectorcraft/pull/2602) | merged | @dotCooCoo / `5a3e833ec887` | Devanagari cluster-aware text hit testing, including on a path. |
| [#2670](https://github.com/storytold/vectorcraft/pull/2670) | merged | @dotCooCoo / `2ae992870b13` | ZWNJ keeps the preceding fallback face. |
| [#2795](https://github.com/storytold/vectorcraft/pull/2795) | merged | @dotCooCoo / `bb7d6b487dff` | Caret placement handles zero-advance ties and soft-wrap stops. |
| [#3004](https://github.com/storytold/vectorcraft/pull/3004) | merged | @sanyamsingh001 / `c07945b93489` | Grapheme-safe logical/visual caret movement and forward Delete; Backspace retains character semantics. |
| [#3193](https://github.com/storytold/vectorcraft/pull/3193) | merged | @Matoo125 / `bf6692ae8211` | Checked mesh-grid dimensions before envelope validation. |
| [#3194](https://github.com/storytold/vectorcraft/pull/3194) | merged | @Matoo125 / `da8e6af7e822` | Refuse malformed mesh point removal without mutation. |
| [#3163](https://github.com/storytold/vectorcraft/pull/3163) | merged | @Matoo125 / `38e73060a7e8` | Clamp deserialized polygon sides to 3..1000. |
| [#3224](https://github.com/storytold/vectorcraft/pull/3224) | merged | @Matoo125 / `dd7991cb200b` | Bound EMF pattern tiles globally across nested scenes. |
| [#3152](https://github.com/storytold/vectorcraft/pull/3152) | merged | @Silvertree2010 / `05017111baa4` | Escape and filter SVG title/description to XML 1.0 characters. |
| [#2657](https://github.com/storytold/vectorcraft/pull/2657) | merged | @Silvertree2010 / `a631a0df6cea` | Escape CSS font-family strings. |
| [#3254](https://github.com/storytold/vectorcraft/pull/3254) | open | @Silvertree2010 / `dfe2f5b3ffff` | Quote CSS font names that are not identifiers. |
| [#2568](https://github.com/storytold/vectorcraft/pull/2568) | merged | @Silvertree2010 / `92647fa5b626` | CSV quoted cells, embedded delimiters/newlines and graph serialization round trip. |
| [#2610](https://github.com/storytold/vectorcraft/pull/2610) | merged | @Silvertree2010 / `8ca4cd2cbf1b` | Format huge graph axis values without saturating integer casts. |
| [#3138](https://github.com/storytold/vectorcraft/pull/3138) | merged | @Silvertree2010 / `ec4975f2374f` | Zero-area marquees can intersect paths. |
| [#2720](https://github.com/storytold/vectorcraft/pull/2720) | merged | @MarekSpodBiedry / `949473f1e392` | Magic Wand stays inside the isolated group/layer. |
| [#2671](https://github.com/storytold/vectorcraft/pull/2671) | open | @dotCooCoo / `ac75c220280c` | Isolation refuses hidden/locked containers before changing state. |
| [#2734](https://github.com/storytold/vectorcraft/pull/2734) | open | @dotCooCoo / `a4fd078e7793` | Isolation refuses template-layer ancestry; existing group/layer-only scope retained. |
| [#2895](https://github.com/storytold/vectorcraft/pull/2895) | merged | @Silvertree2010 / `f2601137ab0d` | Plain feet and feet/inches use distinct unit names. |

Compatibility and limits:

- The existing model lacks newer character fields and hyphenation options; style capture/reset only
  includes its supported attributes. Existing isolation remains group/layer-only and respects hidden,
  locked and template ancestors. Magic Wand uses the existing ancestry API.
- Escape claims the key while text is being edited, a small prerequisite present in the pinned
  #1975 postimage; this lets the UI apply its tool-switch request before isolation shortcuts.
- Grapheme helpers use `unicode-segmentation` 1.13.3, already present in the lockfile; adding a direct
  text-crate dependency introduces no package version change. Left/Right and forward Delete avoid
  internal grapheme positions; Backspace keeps its existing character deletion behavior.
- Native font names are document font metadata, not interface translations. No fonts, assets, language
  catalogs, contributor profiles, community controls or services are added. Synthetic font fixtures use
  existing licensed font bytes and never inspect Adobe installations.
- SVG XML filtering is limited to title/description metadata. General text/control sanitization and XML
  ID work remain deferred. Nested Repeat budgets and broad renderer/import/backend/automation changes
  require separate compatibility passes; the snapshot lists scoped exclusions.
- Another chat had partially applied overlapping patches in this checkout. Its writes stopped; a full
  snapshot was preserved locally, the extra candidates were independently reviewed, and missing or
  duplicate hunks were repaired in an isolated checkout before reconciliation. Owner policy edits are
  kept separate. No cross-chat message was sent without authorization.

Verification: **all eight CI gates pass — 5,238 tests passed, zero failed, four existing ignored**.
The focused UI suite passes all 933 tests. The actual browser application also checks for WASM,
with the existing unused `ask_update_changed` warning in unchanged `dialogs/missing_links.rs`.
Native control on port 17993 verifies Hindi grapheme movement 6 → 18 → 6 through real Left/Right
input in mixed Hindi/Hebrew type, Telugu edit/undo/redo, ZWNJ/ZWJ text and `.vcnr` save/reopen.
Reopened node JSON and artwork PNG SHA-256 are identical; the reopened document is clean.
Inspected Canvas, Preferences and About screenshots confirm English UI, Indic text, Vectnor branding,
Developer: Appaji Ambarisha and Contributors/Models. The test app quit cleanly. Evidence remains local
under `/tmp/vectnor-pr-review-2026-10-21-vectnor/`.

Performance remains **8/10 budgets before and after** on 50k synthetic paths in the optimized-development
build: 400% render 22.91 → 21.46 ms (16 ms limit), native open 311.25 → 306.05 ms (300 ms limit).
The after run follows completed builds/checks and native app exit. Load averages differ (7.9 before,
3.9 after, 12 cores); these single runs establish neither a speedup nor a regression. The two overruns
remain work under G15. `dist/Vectnor.app` is refreshed from SHA-256-verified native/CLI binaries.

Scores, estimates and application version stay unchanged. No source, build, PR or hosted app is uploaded.

## 2026-10-17 local update — M0.upstream-safety

Screened **551 PRs updated since `f6d40ed`** (2026-10-13 11:01:43 UTC): 364 merged, 171 open and
16 closed without merge. Ten public GitHub REST pages contained 1,000 distinct metadata records; page six
crossed the cutoff. Scoped **27 candidate diffs** for applicability, then reviewed and adapted code/tests from **18 PRs** (17 merged and one open).
Metadata screening is not a claim that every proposal's code was reviewed or implemented. The
[dated snapshot](upstream-audit-2026-10-17.json) records every screened decision, source-author logins,
immutable heads, raw diff SHA-256 and verified postimage Git blob IDs. Older audit snapshots are unchanged.

| PR | State | Author / inspected head | Local adaptation |
|---|---|---|---|
| [#1774](https://github.com/storytold/vectorcraft/pull/1774) | merged | @MarekSpodBiedry / `8a7e36ede870` | Stamp File Info save dates only after a successful write, including edits during background saving. |
| [#1775](https://github.com/storytold/vectorcraft/pull/1775) | merged | @MarekSpodBiedry / `1f44a18c7a07` | A failed live preview clears the preview journal and pending paint; committing it records no failed edit. |
| [#1777](https://github.com/storytold/vectorcraft/pull/1777) | merged | @MarekSpodBiedry / `d82d6e362048` | Sanitize export prefixes and suffixes as filename text so files remain inside the export folder. |
| [#1778](https://github.com/storytold/vectorcraft/pull/1778) | merged | @MarekSpodBiedry / `122fe05cee3a` | Create replacement staging files owner-only on Unix; retain Vectnor atomic rename and original-file preservation on failure. |
| [#1780](https://github.com/storytold/vectorcraft/pull/1780) | merged | @MarekSpodBiedry / `7abc1cfa1093` | Use finite, counted and capped scatter-axis ticks instead of an unbounded floating-point loop. |
| [#1779](https://github.com/storytold/vectorcraft/pull/1779) | merged | @MarekSpodBiedry / `ce53bb7c5782` | Enforce the Liquify dab limit inside long-segment expansion and repeated-point input. |
| [#1803](https://github.com/storytold/vectorcraft/pull/1803) | merged | @AyberkA0 / `e80cc9b49b9d` | Escape cancels existing Direct Selection drag states and clears snapping state; fit to the earlier tool fields. |
| [#1804](https://github.com/storytold/vectorcraft/pull/1804) | merged | @AyberkA0 / `ef2d678cbf50` | A filled Pencil path preserves stroke None; invisible unfilled paths and brushes still receive a visible stroke. |
| [#1787](https://github.com/storytold/vectorcraft/pull/1787) | merged | @MarekSpodBiedry / `d775549258a8` | Commit typed panel values on focus loss before canvas selection changes; keep Escape cancellation and external-value synchronization. |
| [#2022](https://github.com/storytold/vectorcraft/pull/2022) | merged | @Silvertree2010 / `18c1bb0d55e9` | Apply a negative picas sign to the whole picas-plus-points value. |
| [#2209](https://github.com/storytold/vectorcraft/pull/2209) | merged | @Silvertree2010 / `a59621fa78ba` | Treat a sign after an arithmetic operator as the sign of the next operand. |
| [#2069](https://github.com/storytold/vectorcraft/pull/2069) | merged | @Silvertree2010 / `1524ceae096f` | Evaluate mixed multiplication and division from left to right. Local adapter bounds input bytes and expression recursion. |
| [#2470](https://github.com/storytold/vectorcraft/pull/2470) | open | @Silvertree2010 / `39a7cc5b41e1` | Reject signs and other non-hex digits before parsing color hex components. |
| [#1776](https://github.com/storytold/vectorcraft/pull/1776) | merged | @MarekSpodBiedry / `63402451cce0` | Check embedded DIB PNG/JPEG dimensions from headers before allocating decoded pixels. |
| [#1807](https://github.com/storytold/vectorcraft/pull/1807) | merged | @AyberkA0 / `0d28ee08722e` | Raise ruzstd to 0.9.1 for RUSTSEC-2026-0344, preserving the other dependency versions. |
| [#1764](https://github.com/storytold/vectorcraft/pull/1764) | merged | @fahrim / `5336d8019623` | Bound MCP request/reply framing, legacy batch messages and command_batch steps/retained replies. Keep existing per-step undo semantics; local cap covers error replies too. |
| [#1768](https://github.com/storytold/vectorcraft/pull/1768) | merged | @fahrim / `7e5606684798` | Require a JSON-RPC 2.0 MCP envelope and correlate app reply IDs. An unmatched reply closes the connection without replay. |
| [#1761](https://github.com/storytold/vectorcraft/pull/1761) | merged | @euisuh / `e9fc3c57ae0c` | Malformed app JSON drops the desynchronized connection and reports uncertain completion; next explicit call reconnects. Thumb-worker test and unrelated docs excluded. |

Compatibility choices and deferred work:

- MCP command batches keep Vectnor's current per-step undo contract; newer upstream atomic-batch APIs are not imported. Both successful and failed retained replies share the cap. Envelope rules apply to MCP; the desktop control protocol retains its existing request shape.
- Field arithmetic adds a local 4,096-byte input cap and 64-level recursion cap. The field buffer also follows external value changes. The supplied Direct Selection Escape fix is fitted to existing state fields; mesh/corner/bracket and guide-specific cancellation remain separate work.
- Replacement staging is owner-only on Unix and keeps existing target permissions. Vectnor still preserves the old file on write/rename failure; the direct-overwrite save fallback is deferred. Windows ACL behavior needs platform verification.
- The newer native unpack module is absent here (#1783). Cross-importer nesting/cycle controls (#1852/#1863), document ID/batch/control deadline contracts (#1862), connection lifetime/input scheduling (#1797/#1815), newer open-path deletion (#1982) and guide Escape (#1922) require separate compatibility work. They remain explicitly deferred.
- Large renderer/cache, PDF backend, tracing, 3D, file-format, scripting and platform changes are metadata-screened candidates, not ported wholesale. Localization, mandatory font embedding, community/promotional integrations and publication changes are excluded or deferred by owner scope. No new assets, fonts or contributor profile entries.
- `ruzstd` alone moves from 0.9.0 to 0.9.1; the package checksum matches the pinned upstream patch. [RustSec confirms the decompression-bomb fix](https://rustsec.org/advisories/RUSTSEC-2026-0344.html). The application version remains 0.9.0.

Verification: **all eight CI gates pass — 5,183 tests passed, zero failed, four existing ignored**.
The affected-crate library run passes 3,297 tests. The actual browser app also compiles for WASM;
an existing dead-code warning in unchanged `dialogs/missing_links.rs` remains. Native control verification
checks real Direct Selection preview/Escape/release, filled Pencil Stroke None, invalid-hex refusal,
failed-save document preservation, Telugu edit/undo/redo and `.vcnr` save/reopen. Reopened node JSON and
artwork PNG SHA-256 are identical and the document is clean. Canvas/Preferences/About screenshots were
inspected: English UI, Indic text, Vectnor branding, Appaji Ambarisha and Contributors/Models remain.
The test app quit cleanly and `dist/Vectnor.app` is rebuilt from verified optimized-development binaries.

Performance remains **8/10 budgets before and after** (50k paths, optimized-development profile):
400% render 21.77 → 21.60 ms (16 ms limit); native open 354.48 → 321.04 ms (300 ms limit).
These are single runs with unknown load-average reporting; no speedup or release-performance claim.
The final after run follows completed CI/web/native checks; an earlier overlapping run is retained
separately and excluded. Evidence stays local under `/tmp/vectnor-upstream-2026-10-17/`.
Scores/estimates are unchanged. No source, build, PR or web app is uploaded.

## 2026-10-13 local update — M0.upstream-update

Reviewed 300 distinct public PR metadata records, including **190 updated since the last local change**
(`d4f8181`, 2026-10-12 12:58:51 UTC): 56 open, 125 merged and nine closed without merge. Adapted **20
updated PRs plus two older required alignment dependencies**. Open proposals are explicitly recorded as
open at the captured review; they are local reviewed adaptations, not described as upstream merged fixes.
All source postimages used to review the selected Rust diffs were verified against their immutable head
commits and Git blob IDs. [This review snapshot](upstream-audit-2026-10-13.json) preserves decisions and pins;
[the previous snapshot](upstream-audit.json) remains unchanged. No GitHub upload.

| PR | State at review | Local adaptation |
|---|---|---|
| [#1666](https://github.com/storytold/vectorcraft/pull/1666) | open | Direct and Group Selection stay within the isolated object. |
| [#1509](https://github.com/storytold/vectorcraft/pull/1509) | merged | Selected paths win overlapping anchor hit tests. |
| [#1633](https://github.com/storytold/vectorcraft/pull/1633) | open | Character-style Auto leading is explicit; resets propagate while local overrides survive. |
| [#1630](https://github.com/storytold/vectorcraft/pull/1630) | open | Shift-double-click enables adjoining Width-point adjustment. |
| [#1589](https://github.com/storytold/vectorcraft/pull/1589) | merged | Generated ICC tags are aligned to four-byte boundaries with checked byte ranges and bounded repacking. |
| [#1485](https://github.com/storytold/vectorcraft/pull/1485) | merged | Graph CSV preserves numeric precision instead of rounding data to display precision. |
| [#1549](https://github.com/storytold/vectorcraft/pull/1549) | merged | Duplicate is available in Keyboard Shortcuts. |
| [#1580](https://github.com/storytold/vectorcraft/pull/1580) | merged | Eligible commands outside menus can receive shortcuts. |
| [#1490](https://github.com/storytold/vectorcraft/pull/1490) | merged | Selected field text uses a readable foreground. |
| [#1506](https://github.com/storytold/vectorcraft/pull/1506) | merged | Slider rails are visible; template layers remain in Preview during Outline view. |
| [#1404](https://github.com/storytold/vectorcraft/pull/1404) | merged | A clipboard picture accompanied by blank text still pastes as an image. |
| [#1415](https://github.com/storytold/vectorcraft/pull/1415) | merged | A missing isolation target reports the missing object without changing eligible object kinds. |
| [#1493](https://github.com/storytold/vectorcraft/pull/1493) | merged | A single selected group supplies its shapes separately to Pathfinder. |
| [#1642](https://github.com/storytold/vectorcraft/pull/1642) | open | Pathfinder counts nested shapes after Expand; local counting avoids cloned node allocations. |
| [#1641](https://github.com/storytold/vectorcraft/pull/1641) | open | A single path's selected anchors follow Align To instead of being treated as one whole object. |
| [#1674](https://github.com/storytold/vectorcraft/pull/1674) | open | Delete removes original anchor indexes and whole selected objects together; surviving paths remain selected. |
| [#1625](https://github.com/storytold/vectorcraft/pull/1625) | merged | Apple ENOTSUP from a full flush falls back to plain fsync; other failures preserve error handling. |
| [#1663](https://github.com/storytold/vectorcraft/pull/1663) | open | Pen movement below three screen pixels remains a corner click. |
| [#1634](https://github.com/storytold/vectorcraft/pull/1634) | open | Character-style capture uses selection-start target keys; Apply keeps its selected range. |
| [#1437](https://github.com/storytold/vectorcraft/pull/1437) | open | Clean-room marker checks run in CI, preserving required notices and documented handwritten fixture headers. |
| [#1216](https://github.com/storytold/vectorcraft/pull/1216) | merged | Dependency: engine aligns selected anchors and their handles, preserving unselected points. |
| [#1276](https://github.com/storytold/vectorcraft/pull/1276) | merged | Dependency: repeated anchor alignment creates no extra undo entry. |

### Compatibility choices and remaining limits

- Anchor deletion is fitted to Vectnor's existing selection model and per-object removal API; newer segment-selection and batch-removal contracts are not imported. Selected guides go with the selected art in the same edit. New tests cover closed-path runs, mixed whole/partial selection, surviving-path selection and Undo/Redo.
- Anchor picking retains Vectnor's existing press-time selection; the newer deferred click/drag contract is not imported. Isolation and overlapping-point regressions verify the resulting selection.
- Alignment imports the required engine point/handle behavior as well as the UI fix. Key-anchor alignment remains absent; with partial anchors, Key Object falls back to Selection. General anchor transforms from #1427 remain separate.
- ICC repacking uses checked ranges and a 64 MiB output limit; no profile binaries are imported. Character-style captures retain documented target keys, Auto-leading resets and local overrides; Indic text support is unchanged.
- The macOS fsync fallback uses the already pinned `rustix 1.1.5`, Apple-only. No dependency package version is added or upgraded. Injected error tests cover unsupported full flush and failed fallback. An actual SMB mount has not been tested. The non-atomic overwrite proposal #1649 remains deferred.
- The clean-room gate checks stored marker bytes, with explicit notice/handwritten-header exceptions. Compressed streams and UTF-16 metadata are not decoded; this is an additional guard, not proof of every asset's origin. Existing required notices and the owner's permitted font baseline remain.
- Translation catalogs/language selectors, mandatory font bundling, old branding/community controls and public publication remain excluded. Larger 3D, tracing, font discovery, scripting, embedding and import/cache work remain candidates with their dependency/platform scope recorded in the snapshot.
- All eight CI gates pass: **5,153 tests passed, zero failed, four existing tests ignored**; WASM passes. Native checks verify point-only Align with no-op history, original-index Delete with a selected remainder and second Delete, grouped Minus Front, Auto-leading source/follower resets with a retained local override, and Shift-double-click Width options. Checked objects and PNG hashes are identical after `.vcnr` save/reopen. Preferences, Width dialog, canvas and About screenshots were inspected; English-only UI, Appaji Ambarisha, Contributors/Models and Telugu artwork remain.
- Optimized-development 50k-path performance is **8/10 budgets before and after**. The existing 400% render overrun is 22.79 → 22.46 ms (16 ms budget); native open is 314.75 → 310.32 ms (300 ms budget). No speedup or release-performance claim is made. Actual SMB, Windows/Linux native surfaces and browser clipboard UI still need platform verification.
- Logs and screenshots stay local under `/tmp/vectnor-upstream-2026-10-13/`. The native bundle is rebuilt as `dist/Vectnor.app`, version 0.9.0. Readiness scores and effort estimates are unchanged; no broad parity remeasurement is claimed.

## Earlier review — M0.upstream, 2026-10-12

Reviewed 57 open and 162 distinct merged pull-request summaries returned by GitHub searches of [storytold/vectorcraft](https://github.com/storytold/vectorcraft/pulls). The merged queries covered recent changes from 2026-10-10 onward. Searches were bounded; this is a recorded review snapshot, not a complete historical inventory. The source diff, upstream merge state, local applicability and synthetic tests were inspected for each of the nine selected backports.

The Vectnor baseline is `cdca9f872becb0c1b7baef107a2a9dda3035ddbf`. [Pinned revisions and source-author credits](../../VECTNOR_UPSTREAM.md) identify the exact inspected contributions. [The machine-readable snapshot](upstream-audit.json) retains the review list and code paths.

| Selected PR | Resulting behavior | Verification |
|---|---|---|
| [#1051](https://github.com/storytold/vectorcraft/pull/1051) | Generated font/credit files keep their timestamps when unchanged. | Repeated build and generated-file timestamp checks. |
| [#1058](https://github.com/storytold/vectorcraft/pull/1058) | Saved views reject malformed coordinates/rotation without adding history. | Saved-view command regression. |
| [#1062](https://github.com/storytold/vectorcraft/pull/1062) | EPS arc/arcn handle huge angles without a per-turn loop. | Timed original PostScript programs and endpoint checks. |
| [#1081](https://github.com/storytold/vectorcraft/pull/1081) | Shape Builder line grids merge without internal seams. | Shared-corner, face coverage and merged-outline tests. |
| [#1155](https://github.com/storytold/vectorcraft/pull/1155) | Extreme geometry/strokes cannot reach the rasterizer's fatal paths. | Synthetic pixel regression; local hidden/unpainted-stroke preservation test. |
| [#1230](https://github.com/storytold/vectorcraft/pull/1230) | Duplicate planar edges share ends; erasing a loose piece removes its own length. | Saved property seed, loose-end regression and 2,000-case pathops run. |
| [#1246](https://github.com/storytold/vectorcraft/pull/1246) | A lost agent reply never causes an edit to be sent twice. | Loopback server applies a real edit, drops its reply, and verifies one edit after explicit reconnect. |
| [#1256](https://github.com/storytold/vectorcraft/pull/1256) | Painting an unpainted zero-width stroke restores 1 pt. | Eyedropper/default/swap/appearance-item tests and full-document Undo/Redo comparisons. |
| [#1320](https://github.com/storytold/vectorcraft/pull/1320) | Closed curves export the DXF SPLINE closed flag. | Exported group-70 assertions. |

## Scope decisions

- Retain Vectnor's branding, developer identity, `.vcnr`/`vectnor` identity, native document compatibility, Welcome thumbnails, canvas scrollbars, and About Contributors/Models.
- Match Pagenor's provenance/audit/versioning organization and central workspace metadata. The existing Rust crate/host layering already has the same shape; keep vector-specific crates and Pagenor's publishing/composition crates appropriate to their applications.
- Open proposals remain candidates. New icon/branding PRs, public hosting/release workflows, upstream promotion and community controls are excluded. Fonts are not imported.
- Broad memory, docking, scripting, new-language and text-shaping changes need their own dependency/compatibility review. Transform Again #1303 depends on newer interaction/journal code; its patch is deferred rather than partially changing that contract.
- The planar fix does not claim every Shape Builder case is solved: upstream #1230 documents a remaining near-horizontal loose-end case. DXF circle/arc entity fitting is also outside #1320.
- Original MIT OR Apache-2.0 licence and copyright notices remain. Contributor preference entries and generated stats are not copied or edited.

## Local evidence

Pathops passes all 31 library tests with `PROPTEST_CASES=2000`, including the previously failing persisted seed. Full workspace CI passes all seven gates: 5,144 tests passed, no failures, 4 existing ignored. Native verification saves/reopens a `.vcnr` document with the restored 1 pt stroke, rejects a malformed saved view without mutation, and preserves Appaji Ambarisha’s About credit plus Contributors/Models. Running screenshots were inspected. Both generated-source build scripts were executed twice against the same output directory; file hashes and nanosecond modification times remain identical. No dependency or application-version changes. The rebuilt bundle is `dist/Vectnor.app`.

The before/after 50,000-path benchmarks use the same optimized development profile. Both pass eight of ten budgets; 400% render and open time exceed their budgets in both runs. Available load-average reporting is unknown, so these single runs do not establish a speedup or release-build performance.

| Budget | Before | After | Limit |
|---|---:|---:|---:|
| Fit-page render | 78.97 ms | 78.81 ms | 100 ms |
| Pan refresh | 79.75 ms | 78.43 ms | 100 ms |
| 400% render | 22.42 ms | 21.56 ms | 16 ms |
| Open | 302.32 ms | 309.47 ms | 300 ms |
| Pathfinder Unite, 1,000 paths | 56.91 ms | 58.16 ms | 150 ms |

Full CI logs: `/tmp/vectnor-upstream-ci-verified.log`. Native proof: `/tmp/vectnor-upstream-reopened.png`, `/tmp/vectnor-upstream-about.png`; `/tmp/vectnor-upstream-smoke.log` confirms a clean reopened document and 1 pt width. Performance logs: `/tmp/vectnor-upstream-perf-before.log`, `/tmp/vectnor-upstream-perf-after.log`. Generated-source proof: `/tmp/vectnor-upstream-generated-check.json`. Temporary verification files are not committed.

## Revision history

| Date | Change | Summary |
|---|---|---|
| 2026-10-22 | minor | Nineteen reviewed upstream scopes for ordered Indic/AltGr input, threaded text, brush references, clipping, EPS/CSS, exact-size shapes, Recent tabs and panel scrolling. Scores unchanged; no upload. [Audit](vectnor/UPSTREAM_AUDIT.md). |
| 2026-10-21 | minor | Thirty-two reviewed local upstream adaptations: Indic caret/joiners, style resets, selection, bounded mesh/EMF processing, CSV/SVG and raster-scale export correctness. Scores unchanged; no upload. [Audit](vectnor/UPSTREAM_AUDIT.md). |
| 2026-10-17 | minor | Eighteen reviewed upstream adaptations for save/export safety, bounded processing, automation framing and editing; scores unchanged. [Audit](upstream-audit-2026-10-17.json). No upload. |
| 2026-10-12 | minor | Nine merged upstream fixes selected, pinned, adapted and regression-tested; Pagenor structure comparison documented. |

## Revision history

| Date | Change | Summary |
|---|---|---|
| 2026-10-13 | minor | Captured the delta after d4f8181, pinned source blobs and recorded twenty updated adaptations plus two older alignment dependencies; local only. |
| 2026-10-12 | minor | Initial nine merged backports and Pagenor structure comparison. |
