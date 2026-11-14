//! Nhúng các file của bản web đã build (index.html, *.js, *.wasm, *.svg) vào exe.
//! Thư mục web: biến môi trường `TRB_SITE_DIR`, mặc định là thư mục cha (gốc repo release).

use std::env;
use std::fmt::Write as _;
use std::fs;
use std::path::PathBuf;

fn mime(name: &str) -> &'static str {
    match name.rsplit('.').next().unwrap_or("") {
        "html" => "text/html; charset=utf-8",
        "js" => "text/javascript; charset=utf-8",
        "wasm" => "application/wasm",
        "svg" => "image/svg+xml",
        "css" => "text/css; charset=utf-8",
        "json" => "application/json",
        "png" => "image/png",
        "ico" => "image/x-icon",
        _ => "application/octet-stream",
    }
}

fn main() {
    // Icon của exe (chỉ khi build cho Windows; cần rc.exe của Windows SDK — có sẵn trên runner windows-latest)
    println!("cargo:rerun-if-changed=icon.ico");
    if env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows")
        && env::var("CARGO_CFG_TARGET_ENV").as_deref() == Ok("msvc")
        && cfg!(windows)
    {
        embed_icon();
    }
    let manifest = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    println!("cargo:rerun-if-env-changed=TRB_SITE_DIR");
    let site = env::var("TRB_SITE_DIR").map(PathBuf::from).unwrap_or_else(|_| manifest.join(".."));
    let site = site.canonicalize().unwrap_or_else(|e| panic!("Không tìm thấy thư mục web '{}': {e}", site.display()));
    println!("cargo:rerun-if-changed={}", site.display());

    let mut files: Vec<PathBuf> = fs::read_dir(&site)
        .expect("Không đọc được thư mục web")
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.is_file())
        .filter(|p| matches!(p.extension().and_then(|e| e.to_str()), Some("html" | "js" | "wasm" | "svg" | "css" | "png" | "ico")))
        .collect();
    files.sort();
    assert!(files.iter().any(|p| p.file_name().is_some_and(|n| n == "index.html")), "Thiếu index.html trong '{}'", site.display());

    let mut out = String::from("/// (đường dẫn URL, Content-Type, nội dung)\npub static FILES: &[(&str, &str, &[u8])] = &[\n");
    for p in &files {
        let name = p.file_name().unwrap().to_string_lossy();
        println!("cargo:rerun-if-changed={}", p.display());
        let _ = writeln!(out, "    (\"/{name}\", \"{}\", include_bytes!({:?})),", mime(&name), p.display().to_string());
    }
    out.push_str("];\n");
    fs::write(PathBuf::from(env::var("OUT_DIR").unwrap()).join("site.rs"), out).unwrap();
}

#[cfg(windows)]
fn embed_icon() {
    let mut res = winresource::WindowsResource::new();
    res.set_icon("icon.ico").set("ProductName", "TRB Helper").set("FileDescription", "TRB Helper");
    res.compile().expect("Không nhúng được icon.ico");
}

#[cfg(not(windows))]
fn embed_icon() {}
