use std::env;
use std::process::Command;

fn main() {
    let target = env::var("TARGET").unwrap();
    if target == "i686-pc-windows-msvc" {
        let def_file = "shell32_stub.def";
        let out_dir = env::var("OUT_DIR").unwrap();
        Command::new("lib")
            .args(["/NOLOGO", "/MACHINE:IX86"])
            .arg(format!("/DEF:{def_file}"))
            .arg(format!("/OUT:{out_dir}\\shell32_stub.lib"))
            .status()
            .unwrap();

        let link_wrapper_src = "link_wrapper\\link_wrapper.rs";
        let link_wrapper_exe = "link_wrapper\\link_wrapper.exe";
        Command::new("rustc")
            .args([link_wrapper_src, "-o", link_wrapper_exe])
            .status()
            .unwrap();

        println!("cargo:rustc-link-lib=shell32_stub");
        println!("cargo:rustc-link-search={out_dir}");
        println!("cargo:rerun-if-changed={def_file}");
        println!("cargo:rerun-if-changed={link_wrapper_src}");
    }
}
