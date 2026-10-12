# Interface language and document scripts

> **Last reviewed:** 2026-10-12 · **Last updated:** 2026-10-12 · **Change:** scope (English-only interface; Indic document text remains required) · **Target:** owner requirements; public specifications

Appaji Ambarisha requested an English-only Vectnor interface on 2026-10-12 and clarified that Indic
language support is still required. Interface translations and document text are separate concerns.

## Current interface scope

- English menus, panels, dialogs, tooltips and status/error messages on desktop and WASM.
- No interface language selector, `app.language` command, translation catalogs or system/browser locale probing.
- Old `language` and `interfaceLanguage` saved settings are ignored safely; other preferences remain.
- Shared `i18n` helpers are now English label/format/plural helpers, kept for widget API compatibility.
- Unicode names, file paths, artwork text, Contributors/Models and legal attribution remain intact.

The previous twelve-language score (54%, measured 2026-10-10) and 70–110 h translation estimate describe
an earlier scope. They are historical, not current requirements or evidence of English-interface readiness.
Headline readiness estimates have not been recalibrated; removing international variants does not improve
the historical full-product score.

## Indic text in artwork

Unicode text input and document storage remain unrestricted. HarfRust shapes text using the selected font;
font lookup/fallback and optional `CRAFT_FONTS_DIR` remain available. The new regression checks cover
Devanagari, Telugu, Bengali, Tamil, Kannada, Malayalam, Gujarati, Gurmukhi and Odia matras/conjunct clusters
and equal-style run splits when a suitable installed or craft-fonts font is present. Fonts are not added to
this repository. Font-dependent checks explicitly report skipped scripts where fonts are unavailable.

A font-independent command test retains text from all nine scripts through editing, undo/redo and native
save/load. Existing bidi and East Asian typography options remain available explicitly. New type uses
Roman-baseline defaults, independent of old interface-language preferences.

This is focused regression coverage, not a complete Indic parity assessment. Indic IME integration,
caret/selection inside complex clusters, regional hyphenation and editable export fidelity need separate
validation. Missing font coverage is reported by the existing font workflow.

## Evidence

- `crates/ui-egui/src/i18n/tests.rs`: English formatting, Unicode names and obsolete preference migration.
- `crates/ui-egui/src/tests_nativemenu.rs`: English app menu with no language controls; Appearance retained.
- `crates/text/src/tests_indic.rs`: source clusters, glyph coverage and shaping across equal-style run splits.
- `crates/engine/src/cmd/typecmd.rs`: Indic edit/undo/redo/native round trip; explicit East Asian alignment.
- [Type checklist](type-parity.md) and [development instructions](development.md#localisation).

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

## Revision history

| Date | Change | Summary |
|---|---|---|
| 2026-10-12 | scope | Removed international interface variants by owner request; kept Indic document support and added focused regression coverage. |
| 2026-10-12 | minor | Alt/Option switches to Smooth label added to all eleven existing non-English catalogs; baseline scores unchanged, native review still pending. |
| 2026-10-12 | minor | Pencil/Paintbrush Keep selected label added to all eleven existing non-English catalogs; complete-language coverage remains enforced. Baseline scores unchanged; these labels have not had native-speaker review. |
| 2026-10-10 | major | First per-language table, measured from the catalog row counts against the complete French catalog |
