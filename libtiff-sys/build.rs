use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    let configure = Path::new("./libtiff/configure");
    if !configure.exists() {
        Command::new("sh")
            .current_dir("./libtiff")
            .arg("autogen.sh")
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
    }

    let libport_config = Path::new("./libtiff/port/libport_config.h");
    if !libport_config.exists() {
        Command::new("sh")
            .current_dir("./libtiff")
            .arg("configure")
            .arg("--target=wasm32-unknown-emscripten")
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
    }

    // let bindings = Path::new("./src/bindings.rs");
    // if !bindings.exists() {
    //     Command::new("bindgen")
    //         .arg("./libtiff/tools/tiff2pdf.c")
    //         .arg("-o")
    //         .arg("./src/bindings.rs")
    //         .arg("--allowlist-function")
    //         .arg("t2p_init")
    //         .arg("--allowlist-function")
    //         .arg("t2p_free")
    //         .arg("--allowlist-function")
    //         .arg("t2p_write_pdf")
    //         .arg("--allowlist-function")
    //         .arg("TIFFClientOpen")
    //         .arg("--allowlist-function")
    //         .arg("TIFFClose")
    //         .arg("--")
    //         .arg("-I./libtiff/port")
    //         .arg("-I./libtiff/libtiff")
    //         .arg("-I/usr/include")
    //         .arg("-I/usr/include/x86_64-linux-gnu")
    //         .spawn()
    //         .unwrap()
    //         .wait()
    //         .unwrap();
    // }

    let p = fs::read_dir("./libtiff/libtiff").unwrap().filter_map(|e| {
        let e = e.ok()?;
        if e.file_type().ok()?.is_dir() {
            return None;
        }
        let file_name = e.file_name();
        let file_name = file_name.to_str()?;
        if !file_name.starts_with("tif_") {
            return None;
        }
        if !file_name.ends_with(".c") {
            return None;
        }
        if file_name.contains("win32") {
            return None;
        }
        Some(e.path())
    });

    cc::Build::new()
        .file("./libtiff/tools/tiff2pdf.c")
        .files(p)
        .flag("-Wno-macro-redefined")
        .flag("-Wno-unused-function")
        .flag("-Wno-sign-compare")
        .flag("-Wno-format")
        .include("./libtiff/port")
        .include("./libtiff/libtiff")
        .include("/usr/include")
        .include("/usr/include/x86_64-linux-gnu")
        .compile("tiff")
}
