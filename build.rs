use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=src/scanner/c_src/");

    let out_dir = env::var("OUT_DIR").unwrap();
    let out_path = PathBuf::from(&out_dir);

    let recv_obj = out_path.join("receive_packet.o");
    let send_obj = out_path.join("send_packet.o");
    let lib_path = out_path.join("libscanner.a");

    let status = Command::new("cc")
        .args(&[
            "-c", "src/scanner/c_src/receive_packet.c",
            "-o", recv_obj.to_str().unwrap(),
            "-Isrc/scanner/c_src",
        ])
        .status()
        .unwrap();
    assert!(status.success());

    let status = Command::new("cc")
        .args(&[
            "-c", "src/scanner/c_src/send_packet.c",
            "-o", send_obj.to_str().unwrap(),
            "-Isrc/scanner/c_src",
        ])
        .status()
        .unwrap();
    assert!(status.success());

    let status = Command::new("ar")
        .args(&[
            "rcs",
            lib_path.to_str().unwrap(),
            recv_obj.to_str().unwrap(),
            send_obj.to_str().unwrap(),
        ])
        .status()
        .unwrap();
    assert!(status.success());

    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=scanner");
}