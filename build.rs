use std::env;
use std::process::Command;

fn main() {
    let target = env::var("TARGET").unwrap();
    if target == "i686-pc-windows-msvc" {
        let out_dir = env::var("OUT_DIR").unwrap();
        println!("cargo:rustc-link-search={out_dir}");

        // ntdll.dll stub
        {
            let src_file = "stub\\ntdll_stub.cpp";
            let obj_file = format!("{out_dir}\\ntdll_stub.obj");
            let status = Command::new("cl")
            .arg("/c")
            .arg(src_file)
            .arg(format!("/Fo{obj_file}"))
            .status()
            .unwrap();
            assert_eq!(status.code(), Some(0));

            let def_file = "stub\\ntdll_stub.def";
            let status = Command::new("link")
                .args(["/NOLOGO", "/DLL"])
                .arg(format!("/DEF:{def_file}"))
                .arg(format!("/OUT:{out_dir}\\ntdll.dll"))
                .arg(obj_file)
                .status()
                .unwrap();
            assert_eq!(status.code(), Some(0));

            println!("cargo:rerun-if-changed={src_file}");
            println!("cargo:rerun-if-changed={def_file}");
        }

        // link wrapper
        let link_wrapper_src = "link_wrapper\\link_wrapper.rs";
        let link_wrapper_exe = "link_wrapper\\link_wrapper.exe";
        let status = Command::new("rustc")
            .args([link_wrapper_src, "-o", link_wrapper_exe])
            .status()
            .unwrap();
        assert_eq!(status.code(), Some(0));

        println!("cargo:rerun-if-changed={link_wrapper_src}");
    }
}
