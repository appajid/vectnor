# Vectnor upstream provenance

Vectnor derives from [storytold/vectorcraft](https://github.com/storytold/vectorcraft), under MIT OR Apache-2.0. Original copyright and third-party notices remain in LICENSE-MIT, LICENSE-APACHE, NOTICE and ASSETS.md. Upstream names here identify source provenance.

The supplied source was initially committed as `d8d143e72223590ca4b451b792ca4e52cf68d23d`. Its original upstream revision was not preserved as verified Git ancestry. Subsequent ports compare the actual source and tests instead of assuming that history. The review baseline is `cdca9f872becb0c1b7baef107a2a9dda3035ddbf`.

Developer and maintainer: **Appaji Ambarisha** (`appajid`). Vectnor keeps its own branding, `.vcnr` extension and `vectnor` format header. The repository remains private. Public source/build/site publication requires the owner's explicit authorization.

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
