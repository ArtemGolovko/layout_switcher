use core::ffi::c_void;

// Macros
macro_rules! unsafe_impl_default_zeroed {
    ($t:ty) => {
        impl Default for $t {
            #[inline]
            #[must_use]
            fn default() -> Self {
                unsafe { core::mem::zeroed() }
            }
        }
    };
}

// c types
pub type wchar_t = u16;
pub type c_int = i32;
pub type c_uint = u32;
pub type c_long = i64;
pub type c_ulong = u64;
pub type c_ushort = u16;

// Win API types
pub type BYTE = u8;
pub type WCHAR = wchar_t;
pub type INT = c_int;
pub type UINT = c_uint;
pub type LONG = c_long;
pub type WORD = c_ushort;
pub type DWORD = c_ulong;
pub type BOOL = INT;
pub type ATOM = WORD;

// Pointer types
pub type PVOID = *mut c_void;
pub type LPVOID = *mut c_void;
pub type PWSTR = *mut WCHAR;
pub type LPWSTR = *mut WCHAR;
pub type PCWSTR = *const WCHAR;
pub type LPCWSTR = *const WCHAR;
pub type LPDWORD = *mut DWORD;
pub type LPBYTE = *mut BYTE;
pub type LPMSG = *mut MSG;
pub type HANDLE = PVOID;
pub type UINT_PTR = usize;
pub type LONG_PTR = isize;

// Handle types
pub type HKL = HANDLE;
pub type HKEY = HANDLE;
pub type HINSTANCE = HANDLE;
pub type HICON = HANDLE;
pub type HBRUSH = HANDLE;
pub type HWND = HANDLE;
pub type HMENU = HANDLE;
pub type HCURSOR = HICON;
pub type PHKEY = *mut HKEY;

// Parameter types
pub type WPARAM = UINT_PTR;
pub type LPARAM = LONG_PTR;

// Status types
pub type REGSAM = DWORD;
pub type LSTATUS = LONG;
pub type HRESULT = LONG;
pub type LRESULT = LONG_PTR;

// Callback types
pub type WNDPROC = Option<
    unsafe extern "system" fn(
        hwnd: HWND,
        uMsg: UINT,
        wParam: WPARAM,
        lParam: LPARAM
    ) -> LRESULT
>;

// Structs
#[repr(C)]
pub struct WNDCLASSW {
    pub style: UINT,
    pub lpfnWndProc: WNDPROC,
    pub cbClsExtra: c_int,
    pub cbWndExtra: c_int,
    pub hInstance: HINSTANCE,
    pub hIcon: HICON,
    pub hCursor: HCURSOR,
    pub hbrBackground: HBRUSH,
    pub lpszMenuName: LPCWSTR,
    pub lpszClassName: LPCWSTR 
}
unsafe_impl_default_zeroed!(WNDCLASSW);

#[repr(C)]
pub struct POINT {
  x: LONG,
  y: LONG,
}
unsafe_impl_default_zeroed!(POINT);


#[repr(C)]
pub struct MSG {
  hwnd: HWND,
  message: UINT,
  wParam: WPARAM,
  lParam: LPARAM,
  time: DWORD,
  pt: POINT,
  lPrivate: DWORD,
}
unsafe_impl_default_zeroed!(MSG);

// Constants
pub const KL_NAMELENGTH: usize = 1024;
pub const KLF_SETFORPROCESS: UINT = 0x00000100;
pub const KLF_REORDER: UINT = 0x00000008;
pub const READ_KEY: DWORD = 0x20019;
pub const HKEY_LOCAL_MACHINE: HKEY = 0x80000002 as HKEY; //(( HKEY ) (ULONG_PTR)((LONG)0x80000002) )
pub const ERROR_SUCCESS: i64 = 0;
pub const BUFFER_LENGTH: usize = 2048;
// pub const REG_EXPAND_SZ: DWORD = 0x00000002 as DWORD;
// pub const REG_SZ: DWORD = 0x00000001 as DWORD; 
pub const RRF_RT_REG_EXPAND_SZ: DWORD = 0x00000004;
pub const RRF_RT_REG_SZ: DWORD = 0x00000002;
pub const S_OK: HRESULT = 0x00000000;
pub const WS_OVERLAPPED: u32 = 0x00000000;
pub const WS_CAPTION: u32 = 0x00C00000;
pub const WS_SYSMENU: u32 = 0x00080000;
pub const WS_THICKFRAME: u32 = 0x00040000;
pub const WS_MINIMIZEBOX: u32 = 0x00020000;
pub const WS_MAXIMIZEBOX: u32 = 0x00010000;
pub const WS_OVERLAPPEDWINDOW: u32 = WS_OVERLAPPED
  | WS_CAPTION
  | WS_SYSMENU
  | WS_THICKFRAME
  | WS_MINIMIZEBOX
  | WS_MAXIMIZEBOX;
pub const CW_USEDEFAULT: c_int = 0x80000000_u32 as c_int;
pub const SW_SHOW: c_int = 5;
pub const WM_CLOSE: u32 = 0x0010;
pub const WM_DESTROY: u32 = 0x0002;

// External functions definitions
#[link(name="Kernel32")]
extern "system" {
    pub fn GetCurrentThreadId() -> DWORD;
    pub fn GetModuleHandleW(lpModuleName: LPCWSTR) -> HINSTANCE;
    pub fn GetLastError() -> DWORD;
}

#[link(name="User32")]
extern "system" {
    pub fn DestroyWindow(hWnd: HWND) -> BOOL;

    pub fn PostQuitMessage(nExitCode: c_int);

    pub fn TranslateMessage(lpMsg: *const MSG) -> BOOL;

    pub fn DispatchMessageW(lpMsg: *const MSG) -> LRESULT;

    pub fn GetMessageW(
        lpMsg: LPMSG, hWnd: HWND, wMsgFilterMin: UINT, wMsgFilterMax: UINT,
    ) -> BOOL;

    pub fn DefWindowProcW(
        hWnd: HWND, Msg: UINT, wParam: WPARAM, lParam: LPARAM,
    ) -> LRESULT;

    pub fn ShowWindow(hWnd: HWND, nCmdShow: c_int) -> BOOL;

    pub fn CreateWindowExW(
        dwExStyle: DWORD,
        lpClassName: LPCWSTR,
        lpWindowName: LPCWSTR,
        dwStyle: DWORD,
        X: c_int,
        Y: c_int,
        nWidth: c_int,
        nHeight: c_int,
        hWndParent: HWND,
        hMenu: HMENU,
        hInstance: HINSTANCE,
        lpParam: LPVOID
    ) -> HWND;

    pub fn RegisterClassW(lpWndClass: *const WNDCLASSW) -> ATOM;

    pub fn GetKeyboardLayoutList(nBuff: INT, lpList: *mut HKL) -> INT;
    pub fn ActivateKeyboardLayout(hkl: HKL, flags: UINT) -> HKL;
    pub fn GetKeyboardLayoutNameW(pwszKLID: LPWSTR) -> BOOL;
    pub fn GetKeyboardLayout(idThread: DWORD) -> HKL;
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


