//! Windows only: embed the app icon and version info (VERSIONINFO) into `vectnor.exe`.
//!
//! On every other target this does nothing. A missing resource compiler is a warning, so a
//! cross-compile from macOS or Linux still links, unless `VECTNOR_REQUIRE_WINRES=1` turns it into
//! an error (for release builds).

fn main() -> Result<(), String> {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=../../assets/app-icon/vectnor.ico");
    println!("cargo:rerun-if-env-changed=VECTNOR_REQUIRE_WINRES");
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return Ok(());
    }
    let mut res = winresource::WindowsResource::new();
    res.set_icon("../../assets/app-icon/vectnor.ico")
        .set("ProductName", "Vectnor")
        .set("FileDescription", "Vectnor vector illustration editor")
        .set("CompanyName", "Appaji Ambarisha")
        .set("LegalCopyright", "Copyright (c) the Vectnor authors. MIT OR Apache-2.0.")
        .set("OriginalFilename", "vectnor.exe")
        .set("InternalName", "vectnor");
    if let Err(e) = res.compile() {
        if std::env::var_os("VECTNOR_REQUIRE_WINRES").is_some() {
            return Err(format!("embedding Windows resources failed: {e}"));
        }
        println!("cargo:warning=vectnor.exe built without icon/version resources: {e}");
    }
    Ok(())
}
