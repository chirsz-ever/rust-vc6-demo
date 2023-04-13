use std::io::{self, prelude::*};

const UNSUPPORTED_LIBS: &[&str] = &["legacy_stdio_definitions.lib", "bcrypt.lib"];

fn main() -> std::process::ExitCode {
    let mut new_args = Vec::new();
    for arg in std::env::args() {
        if let Some(response_file) = arg.strip_prefix('@') {
            // VC6 only support read response file in ANSI encoding
            let response_file_utf16 = read_utf16_file(response_file).unwrap();
            let response_file_content = String::from_utf16(&response_file_utf16).unwrap();
            for a in response_file_content.split_ascii_whitespace() {
                let a = &a[1..a.len() - 1];
                if !UNSUPPORTED_LIBS.contains(&a) {
                    new_args.push(a.to_owned());
                }
            }
        } else if !UNSUPPORTED_LIBS.contains(&arg.as_str()) {
            new_args.push(arg.to_owned());
        }
    }
    let output = std::process::Command::new("LINK.EXE")
        .args(&new_args[1..])
        .output()
        .unwrap();
    std::io::stdout().write_all(&output.stdout).unwrap();
    std::io::stderr().write_all(&output.stderr).unwrap();
    std::process::ExitCode::from(output.status.code().unwrap_or(1) as u8)
}

fn read_utf16_file(path: &str) -> io::Result<Vec<u16>> {
    let mut f = std::fs::File::open(path)?.bytes();
    let bom = [f.next().unwrap()?, f.next().unwrap()?];
    let mut content = vec![];
    if bom == [0xFF, 0xFE] {
        // little endian
        while let (Some(Ok(b0)), Some(Ok(b1))) = (f.next(), f.next()) {
            content.push(u16::from_le_bytes([b0, b1]))
        }
    } else {
        // big endian
        assert_eq!(bom, [0xFE, 0xFF]);
        while let (Some(Ok(b0)), Some(Ok(b1))) = (f.next(), f.next()) {
            content.push(u16::from_be_bytes([b0, b1]))
        }
    }
    Ok(content)
}
