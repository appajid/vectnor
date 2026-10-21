# Upstream review

> **Last reviewed:** 2026-10-21 · **Last updated:** 2026-10-21 · **Change:** minor (reviewed Indic, editing and export fixes; scores unchanged)

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
| 2026-10-21 | minor | Thirty-two reviewed local upstream adaptations: Indic caret/joiners, style resets, selection, bounded mesh/EMF processing, CSV/SVG and raster-scale export correctness. Scores unchanged; no upload. [Audit](vectnor/UPSTREAM_AUDIT.md). |
| 2026-10-17 | minor | Eighteen reviewed upstream adaptations for save/export safety, bounded processing, automation framing and editing; scores unchanged. [Audit](upstream-audit-2026-10-17.json). No upload. |
| 2026-10-12 | minor | Nine merged upstream fixes selected, pinned, adapted and regression-tested; Pagenor structure comparison documented. |

## Revision history

| Date | Change | Summary |
|---|---|---|
| 2026-10-13 | minor | Captured the delta after d4f8181, pinned source blobs and recorded twenty updated adaptations plus two older alignment dependencies; local only. |
| 2026-10-12 | minor | Initial nine merged backports and Pagenor structure comparison. |
