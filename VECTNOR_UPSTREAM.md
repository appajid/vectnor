# Vectnor upstream provenance

> **Last reviewed:** 2026-10-31 · **Last updated:** 2026-10-31 · **Change:** minor (reviewed import and editing fixes; scores unchanged)

Vectnor derives from [storytold/vectorcraft](https://github.com/storytold/vectorcraft), under MIT OR Apache-2.0. Original copyright and third-party notices remain in LICENSE-MIT, LICENSE-APACHE, NOTICE and ASSETS.md. Upstream names here identify source provenance.

The supplied source was initially committed as `d8d143e72223590ca4b451b792ca4e52cf68d23d`. Its original upstream revision was not preserved as verified Git ancestry. Subsequent ports compare the actual source and tests instead of assuming that history. The review baseline is `cdca9f872becb0c1b7baef107a2a9dda3035ddbf`.

Developer and maintainer: **Appaji Ambarisha** (`appajid`). Vectnor keeps its own branding, `.vcnr` extension and `vectnor` format header. The GitHub repository is public following the owner's explicit authorization on 2026-10-22. Local source uploads and build/site publication still require an explicit request.

## Reviewed adaptations — M0.upstream, 2026-10-12

| PR | Source author | Inspected head | Adaptation |
|---|---|---|---|
| [#1051](https://github.com/storytold/vectorcraft/pull/1051) | @bobtherobot | `a785b53e7b237eb941307bebfd1631fe7f4df994` | Generated font/credit source files are rewritten only when their contents change. |
| [#1058](https://github.com/storytold/vectorcraft/pull/1058) | @Ni-zav | `68241453dd0d9378a5dfb8d93269c90f782c66d2` | Saved views require exactly two finite coordinates and a finite numeric rotation; rejected input leaves history unchanged. |
| [#1062](https://github.com/storytold/vectorcraft/pull/1062) | @dotCooCoo | `1da3c0c05286bda36fa3e16173b3e4088e93826f` | EPS arc/arcn normalize huge angles in constant time, with bounded synthetic import tests. |
| [#1081](https://github.com/storytold/vectorcraft/pull/1081) | @NoJuanNobody | `9896eda208bfcbcdac6090cb0d46be8ff248c8ff` | Shape Builder merges line-grid regions through their shared graph edges, removing internal seams. |
| [#1155](https://github.com/storytold/vectorcraft/pull/1155) | @bflatastic | `255fefe71f52a4dc7e0b93b48d67188f80ff5120` | Rendering skips geometry/strokes outside safe device-coordinate limits; a local regression preserves fills beside hidden or unpainted oversized strokes. |
| [#1230](https://github.com/storytold/vectorcraft/pull/1230) | @dotCooCoo | `138d9499a78401b0af841be1371103b4f500f108` | Near-coincident copies of planar edges share vertices; the saved cutting-property failure and loose-end regression pass. |
| [#1246](https://github.com/storytold/vectorcraft/pull/1246) | @KnottyDyes | `44f3141c3dc9a94262ea63eaedc59b89139e99ee` | A lost MCP reply reports uncertain completion and never automatically resends an edit; the next explicit call can reconnect. |
| [#1256](https://github.com/storytold/vectorcraft/pull/1256) | @haranishi | `836222393296435dd610f526e4b028da2bae7113` | Adding paint to an unpainted zero-width stroke restores 1 pt; deliberate painted zero-width strokes and other appearance items retain their widths. |
| [#1320](https://github.com/storytold/vectorcraft/pull/1320) | @rollroyces | `851edd530f933541b4c20fb7d6e93307aff1271d` | Closed DXF spline exports carry group-70 bit 0 alongside the planar bit; circle/arc entity fitting remains separate. |

All nine selected contributions were merged upstream when inspected. Only the reviewed code/test portions were ported; product identifiers were adapted to Vectnor. No upstream contributor preference entries, fonts, artwork, branding, community controls or publication workflows were copied. Source-author logins record provenance and do not modify the generated Contributors/Models registry.

[Pinned review and decisions](docs/vectnor/UPSTREAM_AUDIT.md) · [Machine-readable snapshot](docs/vectnor/upstream-audit.json) · [Shared structure with Pagenor](docs/vectnor/STRUCTURE.md).

## Local update — M0.upstream-update, 2026-10-13

Twenty updated upstream PRs and two older required alignment dependencies are adapted locally after the
last change `d4f8181`. Each source author's login, PR URL, immutable head, raw diff hash and verified Git
blob identity is recorded in [the new review snapshot](docs/vectnor/upstream-audit-2026-10-13.json).
[Adaptations and limits](docs/vectnor/UPSTREAM_AUDIT.md#2026-10-13-local-update--m0upstream-update)
distinguish open proposals from merged fixes and identify the portions fitted to Vectnor's earlier APIs.
Existing provenance and contributor registries remain intact. No source, build or web app is uploaded.

## Local update — M0.upstream-safety, 2026-10-17

Eighteen adaptations selected after `f6d40ed` are recorded in the
[dated source-author/head/blob snapshot](docs/vectnor/upstream-audit-2026-10-17.json).
The [review](docs/vectnor/UPSTREAM_AUDIT.md#2026-10-17-local-update--m0upstream-safety) distinguishes
551 metadata-screened updates from 27 candidate diff scopes and 18 reviewed local adaptations. Existing attribution,
contributor registries, branding and licenses remain intact. No upload.

## Local update — M0.upstream-indic, 2026-10-21

Screened 687 updated PRs since `a17ee43`; adapted selected scopes from 32 (25 merged, seven open
at capture). [Review and compatibility choices](docs/vectnor/UPSTREAM_AUDIT.md) and
[immutable source snapshot](docs/vectnor/upstream-audit-2026-10-21.json) record authors and provenance.
English interface, Indic document support, private repository policy and existing attribution remain.
No new fonts/assets or upload. Scores and version remain unchanged.

## Local update — M0.upstream-workflow, 2026-10-22

Screened 329 upstream PR updates since `fcfa46a`; adapted 19 selected scopes (18 merged, one open,
including one regression-only PR) after 41 Rust postimage blob checks. Six prior adaptations were
rechecked at rebased merged heads and are not counted again. [Review](docs/vectnor/UPSTREAM_AUDIT.md)
and [source snapshot](docs/vectnor/upstream-audit-2026-10-22.json) record exact scope and exclusions.
The two substantial CDR proposals remain deferred; this update adds no CDR support. Keep changes local.

## Local update — M0.upstream-ai-text, 2026-10-25

Screened 541 updated VectorCraft PRs since the October 22 capture; scoped 23 diffs and adapted eight
merged scopes for AI palette/Registration import, oversized-image fallback, closed-path text, font
lookup/concurrent publication and Convert To Point Type on threaded frames. The
[review](docs/vectnor/UPSTREAM_AUDIT.md) and
[immutable snapshot](docs/vectnor/upstream-audit-2026-10-25.json) distinguish adapted scopes from
metadata-only screening and deferred dependency chains. Existing attribution and AI original-frame
recovery remain. No new assets/dependencies, version change or upload.

## Local update — M0.upstream-import-editing, 2026-10-31

Screened 687 VectorCraft PR updates after the October 25 capture, scoped 24 candidate diffs and
adapted 14 merged scopes. Changes cover whole-cluster font fallback, available font styles,
AI capitalization/gradient metadata, SVG CSS parsing, bounded/malformed input handling, mask-layer
creation and subpixel canvas mapping. [Review](docs/vectnor/UPSTREAM_AUDIT.md) and
[pinned source snapshot](docs/vectnor/upstream-audit-2026-10-31.json) record exact authors, heads,
postimages, local compatibility bounds and exclusions. No font files, dependencies or publication.

## Revision history

| Date | Change | Summary |
|---|---|---|
| 2026-10-22 | minor | Nineteen reviewed upstream scopes for ordered Indic/AltGr input, threaded text, brush references, clipping, EPS/CSS, exact-size shapes, Recent tabs and panel scrolling. Scores unchanged; no upload. [Audit](docs/vectnor/UPSTREAM_AUDIT.md). |
| 2026-10-22 | policy | Owner explicitly authorized making appajid/vectnor public; GitHub visibility verified PUBLIC. No local commits pushed. |
| 2026-10-21 | minor | Thirty-two reviewed local upstream adaptations: Indic caret/joiners, style resets, selection, bounded mesh/EMF processing, CSV/SVG and raster-scale export correctness. Scores unchanged; no upload. [Audit](docs/vectnor/UPSTREAM_AUDIT.md). |
| 2026-10-17 | minor | Eighteen reviewed upstream adaptations for save/export safety, bounded processing, automation framing and editing; scores unchanged. [Audit](docs/vectnor/UPSTREAM_AUDIT.md). No upload. |
| 2026-10-13 | minor | Reviewed upstream adaptations and compatibility/verification notes; no upload. |
