use std::env::var;
use std::{fs, io};
use std::path::Path;
use std::process::Command;
use glob::glob;

fn is_output_file_outdated<P1, P2>(input_paths: impl IntoIterator<Item = P1>, output: P2) -> io::Result<bool>
    where
        P1: AsRef<Path>,
        P2: AsRef<Path>,
{
    let out_meta = fs::metadata(output);

    if let Ok(meta) = out_meta {
        let output_mtime = meta.modified()?;

        // if input file is more recent than our output, we are outdated
        for input in input_paths {
            let input_meta = fs::metadata(input)?;
            let input_mtime = input_meta.modified()?;

            if input_mtime > output_mtime {
                return Ok(true)
            }
        }

        Ok(false)
    } else {
        // output file not found, we are outdated
        Ok(true)
    }

}

fn main() {
    let out_dir = var("OUT_DIR").unwrap();
    let is_debug = matches!(var("PROFILE").unwrap().as_str(), "debug");

    println!("cargo:rerun-if-changed=kernels/**/*");
    println!("cargo:rustc-link-search={}", out_dir);
    println!("cargo:rustc-link-lib=kernels");

    let output_object = format!("{}/libkernels.a", out_dir);

    let input_files: Vec<_> = glob("kernels/*.*")
        .expect("Failed to read kernel directory glob pattern")
        .map(|x| x.expect("Failed to read path from glob"))
        .collect();

    if is_output_file_outdated(&input_files[..], &output_object).expect("Failed to check if output file is outdated") {
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
            .args(input_files)
            .status()
            .expect("link command failed, IO error");
        if !status.success() {
            panic!("link command failed, exit status {}", status);
        }
    }
}
