# Vectnor and Pagenor structure

> **Last reviewed:** 2026-10-12 · **Last updated:** 2026-10-12 · **Change:** minor (shared workspace and integration conventions documented)

Vectnor and [Pagenor](https://github.com/appajid/pagenor) share a Rust workspace structure, a command-driven engine, a thin egui UI, native/CLI/web hosts and an MCP adapter. Their document models and feature crates serve different editing tasks.

| Responsibility | Vectnor | Pagenor |
|---|---|---|
| Shared package metadata | Root `Cargo.toml`; version, edition, licence, Rust requirement and repository | Root `Cargo.toml` |
| Foundation and document model | `crates/geom`, `color`, `doc` | `crates/geom`, `color`, `doc` |
| Typography | `crates/text` | `crates/fonts`, `compose` |
| Rendering and interchange | `render`, `svg`, `pdf`, `eps`, `cad`, `metafile`, `format` | `render`, `pdf`, `idml`, `indd`, `epub`, `pptx`, `format` |
| Interactive tools | `crates/tools` emits engine commands | `crates/tools` emits engine commands |
| Editing and undo | `crates/engine/src/cmd` | `crates/engine/src/cmd` |
| UI and UI commands | `crates/ui-egui`; `menus.rs`, `theme::Tokens` | `crates/ui-egui`; `menus.rs`, `theme::Tokens` |
| Agent adapter | `crates/mcp`; existing control protocol | `crates/mcp`; authenticated control protocol |
| Hosts | `apps/vectnor`, `vectnor-cli`, `vectnor-web` | `apps/pagenor`, `pagenor-cli`, `pagenor-web` |
| Test helpers and quality gates | `crates/testkit`, `xtask`; `cargo xtask ci` | `crates/testkit`, `xtask`; `cargo xtask ci` |
| Native document identity | `.vcnr`, `vectnor` header | `.pgnr`, Pagenor's native archive |
| Provenance and review records | `VECTNOR_UPSTREAM.md`, `docs/vectnor/UPSTREAM_AUDIT.md` | `PAGENOR_UPSTREAM.md`, `docs/pagenor/` audit records |

All Vectnor workspace packages inherit the root repository metadata and application version. Package ownership, native document identity and version sequence remain specific to each application. Existing import compatibility and the optional `CRAFT_FONTS_DIR` contract remain.

The shared integration workflow is: inspect upstream state and source revisions, compare with local behavior, adapt suitable code and original synthetic tests, verify native/CLI/WASM behavior, record provenance and limits, and commit. Keep generated contributor statistics unchanged; source-author credits belong in provenance records. Do not import upstream branding, community controls, new font files or publication workflows.

Vectnor's clean-room rule remains absolute: public documentation and original inputs only; never run or inspect Adobe applications. Differences in another repository's agent instructions do not change this rule. Vectnor remains private and retains its current control protocol. Authentication, publishing and large cross-app API changes require their own implementation and compatibility work.

The comparison used Pagenor's [workspace manifest](https://github.com/appajid/pagenor/blob/main/Cargo.toml), [README](https://github.com/appajid/pagenor/blob/main/README.md), [upstream provenance](https://github.com/appajid/pagenor/blob/main/PAGENOR_UPSTREAM.md), [version policy](https://github.com/appajid/pagenor/blob/main/docs/pagenor/VERSIONING.md) and [release audit](https://github.com/appajid/pagenor/blob/main/docs/pagenor/UPSTREAM_040_AUDIT.md), read on 2026-10-12.

## Revision history

| Date | Change | Summary |
|---|---|---|
| 2026-10-12 | minor | Shared workspace conventions and application-specific boundaries recorded; Vectnor repository metadata centralized. |
