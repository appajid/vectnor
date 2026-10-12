# Vectnor versioning

`[workspace.package].version` in the root `Cargo.toml` is the application version. Workspace packages inherit it; native About, executables and packaging use it. Vectnor owns its version sequence independently of upstream and Pagenor.

The reviewed upstream backports retain **0.9.0**. No release, tag, public build or hosted web app is created by this source checkpoint. Run the full quality gate before committing; actual signing, installation and native platform execution are separate checks.

The native document format version is independent of the application version. Version changes do not rename `.vcnr` documents or change the `vectnor` header. Format changes require compatibility and migration tests.

Keep the repository private. Source updates to the owner's private repository and publication of builds or a hosted web app are distinct actions; public publication requires explicit owner authorization. Report the actual checks performed and outstanding limits in release notes when a release is authorized.
