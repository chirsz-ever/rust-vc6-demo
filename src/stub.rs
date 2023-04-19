// wait for `link_ordinal` stable in i686
// #[link(name = "shell32.dll", kind = "raw-dylib")]
// extern "system" {
//     #[link_ordinal(232)]
//     pub fn SHSetFolderPathW(_0: c_int, _1: HANDLE, _2: DWORD, _3: LPCWSTR) -> HRESULT;
// }

#[no_mangle]
unsafe extern "cdecl" fn __CxxFrameHandler3() {
    std::process::exit(1);
}
