#![allow(non_snake_case)]

use core::ffi::{c_int, c_long, c_ulong, c_void};

type DWORD = c_ulong;
type HANDLE = *mut c_void;
type HRESULT = c_long;
type LPCWSTR = *const u16;

// see https://learn.microsoft.com/en-us/windows/win32/api/shlobj_core/nf-shlobj_core-shsetfolderpathw
#[no_mangle]
extern "stdcall" fn SHSetFolderPathW(_0: c_int, _1: HANDLE, _2: DWORD, _3: LPCWSTR) -> HRESULT {
    0
}

#[no_mangle]
unsafe extern "cdecl" fn __CxxFrameHandler3() {
    std::process::exit(1);
}
