use glob::glob;

fn main() {
    // TODO make this compile correctly without cuda as well as a build option
    cc::Build::new()
        .cuda(true)
        .cudart("static")
        .include("kernels/include")
        .flag("-t0")
        .files(
            glob("kernels/*.*")
                .expect("Failed to read kernel directory glob pattern")
                .map(|x| x.expect("Failed to read path from glob"))
        )
        .emit_rerun_if_env_changed(true)
        .compile("kernels");
    println!("cargo:rerun-if-changed=kernels/**/*.cu");
    println!("cargo:rerun-if-changed=kernels/**/*.c");
    println!("cargo:rerun-if-changed=kernels/**/*.h");
    println!("cargo:rerun-if-changed=kernels/**/*.cpp");
    println!("cargo:rerun-if-changed=kernels/**/*.hpp");
}
