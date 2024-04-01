use std::env::var;
use std::process::Command;
use glob::glob;

fn main() {
    let out_dir = var("OUT_DIR").unwrap();
    let is_debug = matches!(var("PROFILE").unwrap().as_str(), "debug");

    println!("cargo:rerun-if-changed=kernels/**/*");
    println!("cargo:rustc-link-search={}", out_dir);
    println!("cargo:rustc-link-lib=kernels");
    println!("cargo:rustc-link-search={}/lib", env!("DPCPP_HOME"));

    let status = Command::new(concat!(env!("DPCPP_HOME"), "/bin/clang"))
        .args([
            "-fsycl",
            "-fsycl-unnamed-lambda",
            "-fsycl-targets=nvptx64-nvidia-cuda",
            "-fPIC"
        ])
        .args([ if is_debug { "-O0" } else { "-O3" }])
        .args([
            "-shared",
            "-o",
            format!("{}/libkernels.so", out_dir).as_str()
        ])
        .args(["-Ikernels/include"])
        .args(glob("kernels/*.*")
            .expect("Failed to read kernel directory glob pattern")
            .map(|x| x.expect("Failed to read path from glob")))
        .status()
        .expect("link command failed, IO error");
    if !status.success() {
        panic!("link command failed, exit status {}", status);
    }
}
