# Upstream review

> **Last reviewed:** 2026-10-13 · **Last updated:** 2026-10-13 · **Change:** minor (reviewed local correctness adaptations; English UI and Indic text retained)

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
| 2026-10-12 | minor | Nine merged upstream fixes selected, pinned, adapted and regression-tested; Pagenor structure comparison documented. |

## Revision history

| Date | Change | Summary |
|---|---|---|
| 2026-10-13 | minor | Captured the delta after d4f8181, pinned source blobs and recorded twenty updated adaptations plus two older alignment dependencies; local only. |
| 2026-10-12 | minor | Initial nine merged backports and Pagenor structure comparison. |
