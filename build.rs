use std::env;
use std::path::PathBuf;

fn main() {
    let arch = std::env::var("ARCH").unwrap_or_default();

    match arch.as_str() {
        "x86_64" => {
            println!("cargo:rustc-cfg=arch_x86_64");
        }
        "aarch64" => {
            println!("cargo:rustc-cfg=arch_aarch64");
        }
        "rv64" => {
            println!("cargo:rustc-cfg=arch_rv64");
        }
        _ => {
            // default architecture
            println!("cargo:rustc-cfg=arch_x86_64");
        }
    }
}

#[allow(unused)]
fn compile_architecture_files(arch: &str) {
    /*
    let mut builder = cc::Build::new();

    let asm_path = format!("src/arch/{}/startup.s", arch);
    let c_path = format!("src/arch/{}/startup.c", arch);

    builder.filer(&asm_path).filer(&c_path).compile("startup");

    println!("cargo:rerun-if-changed={}", asm_path);
    println!("cargo:rerun-if-changed={}", c_path);
    */
}
