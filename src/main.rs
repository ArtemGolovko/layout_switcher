#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use widestring::{U16CStr, U16CString};

type c_int = i32;
type c_uint = u32;
type UINT = c_uint;
type INT = c_int;
type c_long = i64;
type LONG = c_long;
type HKL = HANDLE;
type HANDLE = PVOID;
type WCHAR = wchar_t;
type wchar_t = u16;
type PVOID = *mut core::ffi::c_void;
type LPWSTR = *mut WCHAR;
type PWSTR = *mut WCHAR;
type LPCWSTR = *const WCHAR;
type PCWSTR = *const WCHAR;
type BOOL = INT;
type c_ulong = u64;
type DWORD = c_ulong;
type HKEY = HANDLE;
type REGSAM = DWORD;
type PHKEY = *mut HKEY;
type LSTATUS = LONG;
type LPDWORD = *mut DWORD;
type LPBYTE = *mut BYTE;
type BYTE = u8;
type HRESULT = LONG;

const KL_NAMELENGTH: usize = 1024;
const KLF_SETFORPROCESS: UINT = 0x00000100;
const READ_KEY: DWORD = 0x20019;
const HKEY_LOCAL_MACHINE: HKEY = 0x80000002 as HKEY; //(( HKEY ) (ULONG_PTR)((LONG)0x80000002) )
const ERROR_SUCCESS: i64 = 0;
const BUFFER_LENGTH: usize = 2048;
// const REG_EXPAND_SZ: DWORD = 0x00000002 as DWORD;
// const REG_SZ: DWORD = 0x00000001 as DWORD; 
const RRF_RT_REG_EXPAND_SZ: DWORD = 0x00000004;
const RRF_RT_REG_SZ: DWORD = 0x00000002;
const S_OK: HRESULT = 0x00000000;

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

#[link(name="Advapi32")]
extern "system" {
    pub fn RegOpenKeyExW(hKey: HKEY, plSubKey: LPCWSTR, ulOptions: DWORD, samDesired: REGSAM, phkResult: PHKEY) -> LSTATUS;
    pub fn RegQueryValueExW(hkey: HKEY, lpValueName: LPCWSTR, lpReserved: LPDWORD, lpType: LPDWORD, lpData: LPBYTE, lpcbData: LPDWORD) -> LSTATUS ;

    pub fn RegGetValueW(hKey: HKEY, lpSubKey: LPCWSTR, lpValue: LPCWSTR, dwFlags: DWORD, pdwType: LPDWORD, pvData: PVOID, pcbData: LPDWORD) -> LSTATUS;
}

#[link(name="Shlwapi")]
extern "system" {
    pub fn SHLoadIndirectString(pszSource: PCWSTR, pszOutBuf: PWSTR, cchOutBuf: UINT, ppvReserved: *const *const core::ffi::c_void) -> HRESULT;
}

fn WStr_to_String(wstr: &[WCHAR]) -> String {
    U16CStr::from_slice_truncate(wstr).unwrap().to_string().unwrap()
}

fn load_indirect_string(input: &str) -> String {
    let mut output_buf: [WCHAR; BUFFER_LENGTH] = [0; BUFFER_LENGTH];
    let input_buf = U16CString::from_str(input).unwrap();

    let result = unsafe {
        SHLoadIndirectString(
            input_buf.as_ptr(),
            output_buf.as_mut_ptr(),
            BUFFER_LENGTH as u32,
            core::ptr::null()
        )
    };

    if result != S_OK {
        panic!("Error RegOpenKeyExW: {result}.");
    }

    WStr_to_String(&output_buf)
}

fn get_keyboard_layout_info(klid: &str) {
    let mut hKey: HKEY = core::ptr::null_mut();

    let lpSubKey = U16CString::from_str(
        format!("SYSTEM\\CurrentControlSet\\Control\\Keyboard Layouts\\{klid}")
    ).unwrap();
    

    let result = unsafe {
        RegOpenKeyExW(
            HKEY_LOCAL_MACHINE,
            lpSubKey.as_ptr(),
            0,
            READ_KEY,
            &mut hKey as *mut HKEY
        )
    };
    if result != ERROR_SUCCESS {
        panic!("Error RegOpenKeyExW: {result}.");
    }

    let layout_display_name = load_indirect_string(&get_string_reg_key(&hKey, "Layout Display Name"));
    let layout_text = get_string_reg_key(&hKey, "Layout Text");

    println!("Layout Display Name: {layout_display_name}.");
    println!("Layout Text: {layout_text}.");
}

fn get_string_reg_key(hKey: &HKEY, key: &str) -> String {
    let mut szBuffer: [WCHAR; BUFFER_LENGTH] = [0; BUFFER_LENGTH];
    let mut dwBufferSize: DWORD = (BUFFER_LENGTH * 2) as u64;

    let lpKey = U16CString::from_str(key).unwrap();
    let mut dwType: DWORD = 0x00000000;

    let result = unsafe {
        RegGetValueW(
            *hKey,
            core::ptr::null(),
            lpKey.as_ptr(),
            RRF_RT_REG_EXPAND_SZ | RRF_RT_REG_SZ,
            &mut dwType as *mut DWORD,
            szBuffer.as_mut_ptr() as PVOID,
            &mut dwBufferSize
        )
    };

    if result != ERROR_SUCCESS {
        panic!("Error RegGetValueW: {result}.");
    }

    WStr_to_String(&szBuffer)
}

fn main() {
    let mut szKeyboard: [WCHAR; KL_NAMELENGTH] = [0; KL_NAMELENGTH];
    let current_HKL = unsafe { GetKeyboardLayout(GetCurrentThreadId()) };
    let keyboard_layout_name: String;
    unsafe {
        ActivateKeyboardLayout(current_HKL, KLF_SETFORPROCESS);
        GetKeyboardLayoutNameW(szKeyboard.as_mut_ptr());
        keyboard_layout_name = WStr_to_String(&szKeyboard);
        
        println!("Current layout name: {keyboard_layout_name}.");
    }

    get_keyboard_layout_info(&keyboard_layout_name);

    let nBuff: INT = unsafe { GetKeyboardLayoutList(0, core::ptr::null_mut()) };
    let mut phkl: Vec<HKL> = Vec::with_capacity(nBuff as usize);

    unsafe { GetKeyboardLayoutList(nBuff, phkl.as_mut_ptr()) };
    let ptr = phkl.as_mut_ptr();

    core::mem::forget(phkl);

    let phkl = unsafe { Vec::from_raw_parts(ptr, nBuff as usize, nBuff as usize) };

    println!("Layout count: {nBuff}.");

    for klid in phkl {
         unsafe {
            ActivateKeyboardLayout(klid, KLF_SETFORPROCESS);

            if GetKeyboardLayoutNameW(szKeyboard.as_mut_ptr()) == 0 {
                panic!("Error");
            }
        };
        let keyboard_layout_name = WStr_to_String(&szKeyboard);
        
        println!("Name: {keyboard_layout_name}.");
        
    }
}
