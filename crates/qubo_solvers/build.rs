use std::env::var;
use std::process::Command;
use glob::glob;

fn main() {
    let out_dir = var("OUT_DIR").unwrap();
    let is_debug = matches!(var("PROFILE").unwrap().as_str(), "debug");

    println!("cargo:rerun-if-changed=kernels/**/*");
    println!("cargo:rustc-link-search={}", out_dir);
    println!("cargo:rustc-link-lib=kernels");

    let output_object = format!("{}/libkernels.a", out_dir);

    let status = Command::new(concat!(env!("DPCPP_HOME"), "/bin/clang++"))
        .arg(format!("--target={}", var("TARGET").unwrap()).as_str())
        .args([
            "-fsycl",
            "-fsycl-unnamed-lambda",
            "-fsycl-targets=nvptx64-nvidia-cuda",
            "-fPIC"
        ])
        .arg(if is_debug { "-O0" } else { "-O3" })
        .args([
            "-shared",
            "-o",
            output_object.as_str()
        ])
        .arg("-Ikernels/include")
        .args(glob("kernels/*.*")
            .expect("Failed to read kernel directory glob pattern")
            .map(|x| x.expect("Failed to read path from glob")))
        .status()
        .expect("link command failed, IO error");
    if !status.success() {
        panic!("link command failed, exit status {}", status);
    }
}
