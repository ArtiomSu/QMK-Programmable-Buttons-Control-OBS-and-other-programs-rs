use std::sync::Mutex;
use winapi::shared::minwindef::{WPARAM, LPARAM, LRESULT, DWORD};
use winapi::shared::windef::*;
use winapi::um::winuser::*;
use winapi::um::libloaderapi::GetModuleHandleW;
use std::ptr::{self};
use std::sync::Arc;

use crate::programmable_keys::programmable_keys::ProgrammableKeys;

#[macro_export]
/// Convert regular expression to a native string, to be passable as an argument in WinAPI
macro_rules! native_str {
    ($str: expr) => {
        format!("{}\0", $str).as_ptr() as *const u16
    };
}

fn handle_hid(raw_input: &RAWINPUT) {
    unsafe { 
        let raw_keyboard_input = raw_input.data.keyboard();

        println!("flags {:?}, extra info {:?}, makeCoke {:?}, message {:?}" , raw_keyboard_input.Flags, raw_keyboard_input.ExtraInformation, raw_keyboard_input.MakeCode, raw_keyboard_input.Message);

        if raw_keyboard_input.Flags == 0 && raw_keyboard_input.MakeCode == 5 && raw_keyboard_input.Message != 5 {
            let prog_key = ProgrammableKeys::from_u32(raw_keyboard_input.Message);
            match prog_key {
                ProgrammableKeys::MACROUNKOWN => {
                    eprintln!("MACROUNKOWN PRESSED");
                },
                _ => {
                    let queue_temp = KEY_QUEUE.clone();
                    match queue_temp{
                        Some(queue) => {
                            match queue.lock() {
                                Ok(mut borrowed_queue) => {
                                    borrowed_queue.push(prog_key);
                                },
                                Err(e) => {
                                    eprintln!("Error locking queue: {:?}", e);
                                }
                            }
                        },
                        None => {
                            eprintln!("KEY_QUEUE is not initialized");
                        }
                    }
                }
            }
        }
    }
}

unsafe extern "system" fn wnd_proc(
    hwnd: HWND,
    msg: u32,
    w_param: WPARAM,
    l_param: LPARAM,
) -> LRESULT {
    match msg {
        WM_INPUT => {
            let mut dwsize: u32 = std::mem::size_of::<RAWINPUT>() as u32;
            let mut raw_input: RAWINPUT = std::mem::MaybeUninit::uninit().assume_init();

            GetRawInputData(
                l_param as *mut _,
                RID_INPUT,
                &mut raw_input as *mut _ as *mut winapi::ctypes::c_void,
                &mut dwsize as *mut _,
               std::mem::size_of::<RAWINPUTHEADER>() as u32
            );

            handle_hid(&raw_input);

            0
        },
        _ => {
            println!("Unknown message: {}", msg);
            DefWindowProcW(hwnd, msg, w_param, l_param)
        },
    }
}

fn create_window() -> HWND {
    unsafe {
        let h_instance = GetModuleHandleW(ptr::null_mut());
        let class_name = native_str!("winky::shadow");
        let win = WNDCLASSW {
            hInstance: h_instance,
            lpfnWndProc: Some(wnd_proc),
            lpszClassName: class_name,
            style: 0,
            cbClsExtra: 0,
            cbWndExtra: 0,
            hbrBackground: ptr::null_mut(),
            hCursor: ptr::null_mut(),
            hIcon: ptr::null_mut(),
            lpszMenuName: ptr::null_mut(),
        };

        assert!(RegisterClassW(&win) != 0);

        let hwnd = CreateWindowExW(
            WS_EX_CLIENTEDGE,
            class_name,
            class_name,
            WS_OVERLAPPEDWINDOW,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            ptr::null_mut(),
            ptr::null_mut(),
            h_instance,
            ptr::null_mut());

        assert!(hwnd != ptr::null_mut());

        return hwnd;
    }
}

fn attach(hwnd: HWND) {
    // let mouse = RAWINPUTDEVICE {
	//     usUsagePage: 1,
	//     usUsage: 2,	// Mice
	//     dwFlags: RIDEV_INPUTSINK,
	//     hwndTarget: hwnd,
    // };

    // let keyboard = RAWINPUTDEVICE {
	//     usUsagePage: 1,
	//     usUsage: 6,	// Keyboards
	//     dwFlags: RIDEV_INPUTSINK,
	//     hwndTarget: hwnd,
    // };

    // https://learn.microsoft.com/en-us/windows/win32/inputdev/using-raw-input

    // let hid = RAWINPUTDEVICE {
	//     usUsagePage: 0x0B, 
	//     usUsage: 0x7,
	//     dwFlags: GIDC_ARRIVAL,
	//     hwndTarget: hwnd,
    // };

    let hid = RAWINPUTDEVICE {
	    usUsagePage: 0x000C, 
	    usUsage: 0x0001,
	    dwFlags: RIDEV_INPUTSINK,
	    hwndTarget: hwnd,
    };

    unsafe { 
        RegisterRawInputDevices(&hid, 1, std::mem::size_of::<RAWINPUTDEVICE>() as u32);
    }
}

fn message_loop(hwnd: HWND) {
    let mut msg = MSG {
        hwnd : hwnd,
        message : 0 as u32,
        wParam : 0 as WPARAM,
        lParam : 0 as LPARAM,
        time : 0 as DWORD,
        pt : POINT { x: 0, y: 0, },
    };
    unsafe {
        while GetMessageW(&mut msg, hwnd as HWND, WM_INPUT, WM_INPUT) == 1 {
            DispatchMessageW(&msg);
        }
        CloseWindow(hwnd);
    }
}
 
static mut KEY_QUEUE: Option<Arc<Mutex<Vec<ProgrammableKeys>>>> = None; 

pub fn windows_start(queue: &Arc<Mutex<Vec<ProgrammableKeys>>>){
    let temp = queue.clone();
    unsafe {
        KEY_QUEUE = Some(temp);
    }
    let hwnd = create_window();
    attach(hwnd);
    message_loop(hwnd);   
}