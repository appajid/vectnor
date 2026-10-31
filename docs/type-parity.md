# Type parity

> **Last reviewed:** 2026-10-31 · **Last updated:** 2026-10-31 · **Change:** minor (reviewed upstream import and text fixes; scores unchanged) · **Target:** public documentation and owner requirements

Type is Illustrator's largest single area by use (weight 13 of 105 across "Type core" and "Type advanced" in
[target-app-parity.md](target-app-parity.md)). This checklist lists the features one by one. Status is from the
code and tests (`crates/text`, `crates/engine/src/cmd/type*.rs`, `textstyles.rs`, `crates/tools/src/text.rs`)
against Illustrator's public documentation; **estimated** unless noted.

## Summary

| Part | % | Remaining (h) |
|---|---:|---:|
| Type core (point, area, path type, Character/Paragraph, styles, threading, OpenType, fonts) | 80% | 14–19 |
| Type advanced (CJK composition, Middle Eastern, spelling, Touch Type, Retype, variable fonts) | 50% | 20–28 |

## Checklist

| Feature | Status | Notes |
|---|---|---|
| Indic Unicode text and shaping | partial, regression tested | English-only interface preserves document text. Nine-script matra/conjunct and equal-style run-split checks run with suitable installed/craft-fonts fonts; edit/undo/redo/native round trip is font-independent. Devanagari hit testing, ZWNJ fallback, zero-advance/wrapped bidi stops and grapheme-safe Left/Right/Delete have focused regressions. Comprehensive IME, selection and editable export fidelity remain unassessed. No new font assets. |
| Point, area and type-on-a-path type; vertical variants | done / partial | vertical point, area and path type have initial support |
| Area Type Options (rows, columns, inset, first baseline, Auto Size, Shrink Text to Fit) | done | |
| Threaded text, Fit Headline, Text Wrap | done | |
| Type on a Path effects and options; brackets | done | #429 |
| Character and Paragraph panels, styles with overrides | done | Capture includes supported default-valued attributes; empty text keeps its insertion style. Auto leading is explicit, redefinition propagates resets while keeping local overrides, with UI/history/native serialization tests. Mixed-value display for differing paragraphs remains missing. |
| OpenType panel, Glyphs panel, alternates | done | Highlight Alternate Glyphs preference unused (#394) |
| Font menu: samples, live preview, filters, favourites; font matching by any name; Find Font | done | #339 |
| Variable fonts | partial | named instances as styles; axis sliders missing |
| Colour fonts (OpenType SVG / COLR) | partial | |
| Every-line and Single-line composers | done | tuned by eye, never compared side by side |
| Hyphenation… and Justification… options | missing | |
| Optical Margin Alignment | missing | menu stub |
| Tab stops and leaders; Tabs panel | partial | Tabs panel and stops with leaders in the model (`text.tabs.set`); leader drawing not verified against the docs |
| Hidden characters (Show Hidden Characters) | done | `type.hiddenCharacters` |
| Spell check (Check Spelling, Auto Spell Check, custom dictionary) | missing | menu stubs; needs an openly licensed dictionary |
| Smart Punctuation, Change Case, Find & Replace | done | |
| Touch Type tool | missing | listed in the toolbar, not implemented |
| Retype (identify fonts in outlines/images) | missing | Illustrator's uses a cloud model |
| Snap to Glyph | missing | menu stub |
| Middle Eastern: bidirectional layout, Paragraph Direction | done | |
| Middle Eastern: Character Direction, digit types, kashidas, Middle Eastern composers, split caret | missing | |
| CJK: input methods (marked text, candidate window) | done | |
| CJK: kinsoku (Hard, Soft, None), Line-end Punctuation Half Width, tate-chu-yoko (auto) | done / partial | custom kinsoku sets, the Mojikumi Settings dialog and the other mojikumi sets missing |
| CJK: ruby, proportional vertical metrics (`vpal`/`palt`), manual tate-chu-yoko | missing | #966, #633 |
| Inline symbols in text | beyond Illustrator | not scored |

## Local verification — 2026-10-12

All seven `cargo xtask ci` gates pass: 5,123 tests passed, none failed, four existing tests ignored;
WASM checks pass. All nine Indic shaping cases ran locally with suitable installed fonts (no font binaries
added). The native app was launched with `VECTNOR_LOCALE=fr-FR` and `VECTNOR_NO_PREFS=1`: Preferences
remained English with no language field. Native canvas screenshots show all nine scripts, and real
Telugu `ui.text` input is preserved. Saving and reopening `.vcnr` retains every source string and produces
an identical PNG render (SHA-256 comparison). The reopened window stopped presenting frames, so the
artwork was verified through `ui.render`; the earlier preferences/canvas screenshots were inspected.
Evidence stays local under `/tmp/vectnor-english-*`; no GitHub upload. This remains focused regression
coverage, not a claim of complete Indic IME/editor/export parity.

## Local input and threading follow-up — 2026-10-22

Bengali Backspace and replacement text in one frame, typed text/editing keys/Enter/IME ordering, AltGr typing without command activation and middle-frame thread release receive focused coverage. Half-form tests build positive/negative fixtures in memory from existing OFL bytes and require actual shaping capability. No new font file or interface translation. Comprehensive Indic IME/export parity remains unassessed. [Pinned audit](vectnor/UPSTREAM_AUDIT.md). Scores unchanged.

## AI original text frames

The AI fallback importer can recover an original multiline area/point text frame and its
styled runs, rather than retaining one object per printed line or fragment. Distinct source
frames remain distinct. Complete text, orientation, context and unique ownership must agree;
uncertain cases remain unchanged. [Evidence and limits](vectnor/AI_IMPORT.md#original-text-frames--2026-10-22).
This is an import improvement; type-core/advanced scores and remaining hours are unchanged.

The supplied multi-artboard case also retains both original linked-frame pairs. Translated
area paths, default identity point matrices and superscript/subscript positions are read.
Missing fonts can still change wrapping and leave text overset; unsupported affine area transforms remain conservative.

AI/AIT import notes now report overflow in the final visible text layout after linked
stories reflow, retaining all text and original frame dimensions. Fitting/hidden/point text
and native payload restores avoid spurious notes; bounded checks report incomplete review.
[Evidence](vectnor/AI_IMPORT.md#general-ai-overflow-reporting--2026-10-25). Scores unchanged.

## Revision history

| Date | Change | Summary |
|---|---|---|
| 2026-10-31 | minor | Reviewed whole-grapheme fallback and available font-style selection. Synthetic Indic tests check common-face selection for base/vowel signs; existing shaping/editing tests retained. AI All Caps and Small Caps preserve original characters; custom source Small Caps size remains unread. No new font files; see docs/vectnor/UPSTREAM_AUDIT.md. |
| 2026-10-25 | minor | Reviewed upstream #3755/#3861/#3921/#3735/#3827: split PostScript font names resolve after installation, concurrent font lookups publish complete families and retry loaded fallback faces, collection enumeration is bounded, and Convert To Point Type removes a linked frame with its own text once. No font files added; missing fonts remain a document limitation. See docs/vectnor/UPSTREAM_AUDIT.md. |
| 2026-10-25 | minor | General AI text-overflow notes after linked-story reflow; contents and original frame sizes retained. Scores unchanged. [Evidence](vectnor/AI_IMPORT.md#general-ai-overflow-reporting--2026-10-25). |
| 2026-10-22 | minor | Supplied two-artboard AI: recover translated and linked original text frames, identity point matrices, saved first baselines and script positions; 143 fragments become 10 text objects locally. Scores unchanged. [Evidence](vectnor/AI_IMPORT.md#multi-artboard-linked-frames--2026-10-22). |
| 2026-10-22 | minor | Recover original AI text frames during artwork fallback: multiline and mixed-style text stays together; ambiguous ownership stays unchanged. Scores unchanged. [Evidence](vectnor/AI_IMPORT.md#original-text-frames--2026-10-22). |
| 2026-10-22 | minor | Nineteen reviewed upstream scopes for ordered Indic/AltGr input, threaded text, brush references, clipping, EPS/CSS, exact-size shapes, Recent tabs and panel scrolling. Scores unchanged; no upload. [Audit](vectnor/UPSTREAM_AUDIT.md). |
| 2026-10-21 | minor | Thirty-two reviewed local upstream adaptations: Indic caret/joiners, style resets, selection, bounded mesh/EMF processing, CSV/SVG and raster-scale export correctness. Scores unchanged; no upload. [Audit](vectnor/UPSTREAM_AUDIT.md). |
| 2026-10-13 | minor | Reviewed upstream correctness pass: twenty updated adaptations and two alignment dependencies; English interface/Indic text retained; scores unchanged. See the [audit](vectnor/UPSTREAM_AUDIT.md). |
| 2026-10-12 | scope | English-only interface by owner request; Indic document text retained with nine-script shaping and edit/undo/redo/native round-trip checks. Historical readiness estimates unchanged pending reassessment. |
| 2026-10-10 | major | First checklist, split out of the parity table's Type core and Type advanced rows |
