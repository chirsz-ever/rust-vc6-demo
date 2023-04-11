#![allow(non_snake_case)]
#![allow(dead_code)]

use core::ffi::{c_int, c_long, c_ulong, c_void};

type BOOL = c_int;
type DWORD = c_ulong;
type HANDLE = *mut c_void;
type HRESULT = c_long;
type LPWSTR = *mut u16;
type LPCWSTR = *const u16;
type LPDWORD = *mut DWORD;

// see https://learn.microsoft.com/en-us/windows/win32/api/userenv/nf-userenv-getuserprofiledirectoryw
#[no_mangle]
pub extern "stdcall" fn _imp__GetUserProfileDirectoryW(
    _0: HANDLE,
    _1: LPWSTR,
    _2: LPDWORD,
) -> BOOL {
    0
}

// see https://learn.microsoft.com/en-us/windows/win32/api/shlobj_core/nf-shlobj_core-shsetfolderpathw
#[no_mangle]
pub extern "stdcall" fn SHSetFolderPathW(_0: c_int, _1: HANDLE, _2: DWORD, _3: LPCWSTR) -> HRESULT {
    0
}

#[no_mangle]
pub unsafe extern "cdecl" fn __CxxFrameHandler3() {
    std::process::exit(1);
}
