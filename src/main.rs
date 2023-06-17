#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::os::windows::prelude::OsStringExt;

type c_int = i32;
type c_uint = u32;
type UINT = c_uint;
type INT = c_int;
type HKL = HANDLE;
type HANDLE = PVOID;
type WCHAR = wchar_t;
type wchar_t = u16;
type PVOID = *mut core::ffi::c_void;
type LPWSTR = *mut WCHAR;
type BOOL = INT;
type c_ulong = u64;
type DWORD = c_ulong;

const KL_NAMELENGTH: usize = 1024;
const KLF_SETFORPROCESS: UINT = 0x00000100;

#[link(name="User32")]
extern "system" {
    pub fn GetKeyboardLayoutList(nBuff: INT, lpList: *mut HKL) -> INT;
    pub fn ActivateKeyboardLayout(hkl: HKL, flags: UINT) -> HKL;
    pub fn GetKeyboardLayoutNameW(pwszKLID: LPWSTR) -> BOOL;
    pub fn GetKeyboardLayout(idThread: DWORD) -> HKL;
}

#[link(name="Kernel32")]
extern "system" {
    pub fn GetCurrentThreadId() -> DWORD;
}

fn LPWSTR_to_String(lpwstr: *const WCHAR) -> String {
    unsafe {
        let length = (0..).take_while(|&i| *lpwstr.offset(i) != 0).count();
        let slice = core::slice::from_raw_parts(lpwstr, length);
        std::ffi::OsString::from_wide(slice).to_string_lossy().into_owned()
    }
}

fn main() {
    let mut szKeyboard: [WCHAR; KL_NAMELENGTH] = unsafe { core::mem::zeroed() };
    let current_HKL = unsafe { GetKeyboardLayout(GetCurrentThreadId()) };
    unsafe {
        ActivateKeyboardLayout(current_HKL, KLF_SETFORPROCESS);
        GetKeyboardLayoutNameW(szKeyboard.as_mut_ptr());
        let keyboard_layout_name = LPWSTR_to_String(szKeyboard.as_ptr());
        
        println!("Current layout name: {keyboard_layout_name}.");
    }

    let nBuff: INT = unsafe { GetKeyboardLayoutList(0, core::ptr::null_mut()) };
    let mut phkl: Vec<HKL> = Vec::with_capacity(nBuff as usize);

    unsafe { GetKeyboardLayoutList(nBuff, phkl.as_mut_ptr()) };
    let ptr = phkl.as_mut_ptr();

    core::mem::forget(phkl);

    let phkl = unsafe { Vec::from_raw_parts(ptr, nBuff as usize, nBuff as usize) };

    println!("Layout count: {nBuff}.");

    for nKeyboard in 0..(nBuff as usize) {
         unsafe {
            ActivateKeyboardLayout(phkl[nKeyboard], KLF_SETFORPROCESS);
            if GetKeyboardLayoutNameW(szKeyboard.as_mut_ptr()) == 0 {
                panic!("Error");
            }
            let keyboard_layout_name = LPWSTR_to_String(szKeyboard.as_ptr());
            
            println!("Name: {keyboard_layout_name}.");
        };
    }
}
