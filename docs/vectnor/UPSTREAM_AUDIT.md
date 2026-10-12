# Upstream review — M0.upstream

> **Last reviewed:** 2026-10-12 · **Last updated:** 2026-10-12 · **Change:** minor (selected merged fixes adapted to Vectnor)

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
