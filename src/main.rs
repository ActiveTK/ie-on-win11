#![windows_subsystem = "windows"]
extern crate winapi;

use winapi::um::objbase::CoInitialize;
use winapi::um::shellapi::ShellExecuteW;
use winapi::um::winuser::SW_SHOWNORMAL;
use std::ptr;

fn main() {
    let url = "https://yahoo.co.jp/";
    unsafe {
        CoInitialize(ptr::null_mut());
        ShellExecuteW(
            ptr::null_mut(),
            "open\0".encode_utf16().collect::<Vec<u16>>().as_ptr(),
            "iexplore.exe\0".encode_utf16().collect::<Vec<u16>>().as_ptr(),
            format!("{} -embedding", url).encode_utf16().chain(Some(0)).collect::<Vec<u16>>().as_ptr(),
            ptr::null(),
            SW_SHOWNORMAL
        );
    }
}
