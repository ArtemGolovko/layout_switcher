#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![windows_subsystem = "windows"]

#![no_main]

use core::ptr::{null, null_mut};
use widestring::widecstr;
use winapi::*;

#[cfg(windows)]
mod winapi;

mod from_wide;
unsafe extern "system" fn window_procedure(
    hWnd: HWND, Msg: UINT, wParam: WPARAM, lParam: LPARAM
) -> LRESULT {
    match Msg {
        WM_CLOSE => drop(DestroyWindow(hWnd)),
        WM_DESTROY => PostQuitMessage(0),
        _ => return DefWindowProcW(hWnd, Msg, wParam, lParam),
    }
    0
}

#[no_mangle]
pub unsafe extern "system" fn wWinMain(
    hInstance: HINSTANCE,
    hPrevInstance: HINSTANCE,
    pCmdLine: PWSTR,
    nCmdShow: c_int
) -> c_int {
    let class_name = widecstr!("Layout Switcher");

    let mut wc = WNDCLASSW::default();
    wc.lpfnWndProc = Some(window_procedure);
    wc.hInstance = hInstance;
    wc.lpszClassName = class_name.as_ptr();

    let atom = unsafe { RegisterClassW(&wc) };
    if atom == 0 {
        let last_error = unsafe { GetLastError() };
        panic!("Could not register the window class, error code: {}", last_error);
    }

    let window_name = widecstr!("Layout Switcher");
    let hwnd = unsafe {
        CreateWindowExW(
            0,
            class_name.as_ptr(),
            window_name.as_ptr(),
            WS_OVERLAPPEDWINDOW as u64,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            null_mut(),
            null_mut(),
            hInstance,
            null_mut()
        )
    };

    if hwnd.is_null() {
        panic!("Failed to create a window.");
    }

    let _previously_visible = unsafe { ShowWindow(hwnd, SW_SHOW) };

    let mut msg = MSG::default();

    loop {
        let message_return = unsafe { GetMessageW(&mut msg, null_mut(), 0, 0) }; 
        if message_return == 0 {
            break;
        } else if message_return == -1 {
            let last_error = unsafe { GetLastError() };
            panic!("Error with `GetMessageW`, error code: {}", last_error);
        } else {
            unsafe {
                TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }
        }
    }

    return 0;
}


// fn main() {
//     let hInstance = unsafe { GetModuleHandleW(null()) };
//     let class_name = widecstr!("Layout Switcher");

//     let mut wc = WNDCLASSW::default();
//     wc.lpfnWndProc = Some(window_procedure);
//     wc.hInstance = hInstance;
//     wc.lpszClassName = class_name.as_ptr();

//     let atom = unsafe { RegisterClassW(&wc) };
//     if atom == 0 {
//         let last_error = unsafe { GetLastError() };
//         panic!("Could not register the window class, error code: {}", last_error);
//     }

//     let window_name = widecstr!("Layout Switcher");
//     let hwnd = unsafe {
//         CreateWindowExW(
//             0,
//             class_name.as_ptr(),
//             window_name.as_ptr(),
//             WS_OVERLAPPEDWINDOW as u64,
//             CW_USEDEFAULT,
//             CW_USEDEFAULT,
//             CW_USEDEFAULT,
//             CW_USEDEFAULT,
//             null_mut(),
//             null_mut(),
//             hInstance,
//             null_mut()
//         )
//     };

//     if hwnd.is_null() {
//         panic!("Failed to create a window.");
//     }

//     let _previously_visible = unsafe { ShowWindow(hwnd, SW_SHOW) };

//     let mut msg = MSG::default();

//     loop {
//         let message_return = unsafe { GetMessageW(&mut msg, null_mut(), 0, 0) }; 
//         if message_return == 0 {
//             break;
//         } else if message_return == -1 {
//             let last_error = unsafe { GetLastError() };
//             panic!("Error with `GetMessageW`, error code: {}", last_error);
//         } else {
//             unsafe {
//                 TranslateMessage(&msg);
//                 DispatchMessageW(&msg);
//             }
//         }
//     }
// }

/*
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

    String::from_wide(&output_buf).unwrap()
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
    println!("Layout Text: {layout_text}.\n");
}

fn read_num() -> usize {
    let mut buf = String::new();
    std::io::stdin().lock().read_line(&mut buf).unwrap();

    buf.trim().parse().unwrap()
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

    String::from_wide(&szBuffer).unwrap()
}

fn main() {
    let mut szKeyboard: [WCHAR; KL_NAMELENGTH] = [0; KL_NAMELENGTH];
    
    let keyboard_layout_name: String;
    let current_HKL = unsafe { GetKeyboardLayout(GetCurrentThreadId()) };
    unsafe {
        ActivateKeyboardLayout(current_HKL, 0);
        GetKeyboardLayoutNameW(szKeyboard.as_mut_ptr());
        keyboard_layout_name = String::from_wide(&szKeyboard).unwrap();
        
        println!("Current layout name: {keyboard_layout_name}.");
    }
    get_keyboard_layout_info(&keyboard_layout_name);
    

    let nBuff: INT = unsafe { GetKeyboardLayoutList(0, core::ptr::null_mut()) };
    // use phkl.set_len instead of core::mem:forget and shadowing
    let mut phkl: Vec<HKL> = Vec::with_capacity(nBuff as usize);

    unsafe { GetKeyboardLayoutList(nBuff, phkl.as_mut_ptr()) };
    let ptr = phkl.as_mut_ptr();

    core::mem::forget(phkl);

    let phkl = unsafe { Vec::from_raw_parts(ptr, nBuff as usize, nBuff as usize) };

    println!("Layout count: {nBuff}.\n");

    for klid in &phkl {
         unsafe {
            ActivateKeyboardLayout(*klid, 0);

            if GetKeyboardLayoutNameW(szKeyboard.as_mut_ptr()) == 0 {
                panic!("Error");
            }
        };
        let keyboard_layout_name = String::from_wide(&szKeyboard).unwrap();

        println!("Name: {keyboard_layout_name}.");
        get_keyboard_layout_info(&keyboard_layout_name);
    }

    let n = read_num();
    if !(0..nBuff as usize).contains(&n) {
        panic!("Error");
    }
    unsafe {
        ActivateKeyboardLayout(phkl[n], KLF_REORDER);
    }

    let mut buf = String::new();
    std::io::stdin().lock().read_line(&mut buf).unwrap();
    println!("You wrote: {buf}.");
}
*/
