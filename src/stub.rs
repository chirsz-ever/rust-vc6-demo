// wait for `link_ordinal` stable in i686
// #[link(name = "shell32.dll", kind = "raw-dylib")]
// extern "system" {
//     #[link_ordinal(232)]
//     pub fn SHSetFolderPathW(_0: c_int, _1: HANDLE, _2: DWORD, _3: LPCWSTR) -> HRESULT;
// }

#[cfg(all(target_env = "msvc", target_arch = "x86"))]
std::arch::global_asm!(
    ".extern ___CxxFrameHandler",
    ".global ___CxxFrameHandler3",
    "___CxxFrameHandler3:",
    "jmp ___CxxFrameHandler",
);
