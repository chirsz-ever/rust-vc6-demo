#![allow(non_camel_case_types)]

use core::ffi::{c_int, c_long, c_ulong, c_void};

type DWORD = c_ulong;
type HANDLE = *mut c_void;
type HRESULT = c_long;
type LPCWSTR = *const u16;

// need Rust 1.71 for `link_ordinal` stable
#[allow(dead_code)]
#[link(name = "shell32.dll", kind = "raw-dylib")]
extern "system" {
    #[link_ordinal(232)]
    pub fn SHSetFolderPathW(_0: c_int, _1: HANDLE, _2: DWORD, _3: LPCWSTR) -> HRESULT;
}

#[cfg(all(target_env = "msvc", target_arch = "x86"))]
std::arch::global_asm!(
    ".extern ___CxxFrameHandler",
    ".global ___CxxFrameHandler3",
    "___CxxFrameHandler3:",
    "jmp ___CxxFrameHandler",
);
