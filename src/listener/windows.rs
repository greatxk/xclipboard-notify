use std::ffi::OsStr;
use std::io::Error;
use std::iter::once;
use std::mem;
use std::ptr::null_mut;
use winapi;

use winapi::shared::windef::HWND;
use winapi::um::libloaderapi::GetModuleHandleW;
use winapi::um::winuser::{
    AddClipboardFormatListener, CreateWindowExW, DefWindowProcW, DispatchMessageW, GetMessageW,
    RegisterClassW, TranslateMessage, CS_HREDRAW, CS_OWNDC, CS_VREDRAW, MSG, WM_CLIPBOARDUPDATE,
    WNDCLASSW,
};

use clipboard_win::{formats, get_clipboard};

pub struct ClipboardNotify {
    hwnd: HWND,
}


impl ClipboardNotify {
    pub fn new() -> ClipboardNotify {
        let notify = ClipboardNotify {
            hwnd: create_window("xwindow", "xwindow-clipboard-notify").unwrap(),
        };
        notify
    }

    pub fn listen<F>(&self, handle: F)
    where
        F: Fn(String),
    {
        loop {
            unsafe {
                let mut message: MSG = mem::MaybeUninit::uninit().assume_init();

                if GetMessageW(&mut message as *mut MSG, self.hwnd, 0, 0) > 0 {
                    if message.message == WM_CLIPBOARDUPDATE {
                        let text = get_clipboard(formats::Unicode).unwrap_or("".to_owned());
                        handle(text);
                    }
                    TranslateMessage(&message as *const MSG);
                    DispatchMessageW(&message as *const MSG);
                } else {
                    break;
                }
            }
        }
    }
}



fn win32_string(value: &str) -> Vec<u16> {
    use std::os::windows::ffi::OsStrExt;
    OsStr::new(value).encode_wide().chain(once(0)).collect()
}

fn create_window(name: &str, title: &str) -> Result<HWND, Error> {
    let name = win32_string(name);
    let title = win32_string(title);

    unsafe {
        let hinstance = GetModuleHandleW(null_mut());
        let wnd_class = WNDCLASSW {
            style: CS_OWNDC | CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(DefWindowProcW),
            hInstance: hinstance,
            lpszClassName: name.as_ptr(),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hIcon: null_mut(),
            hCursor: null_mut(),
            hbrBackground: null_mut(),
            lpszMenuName: null_mut(),
        };
        RegisterClassW(&wnd_class);
        let handle = CreateWindowExW(
            0,
            name.as_ptr(),
            title.as_ptr(),
            0,
            0,
            0,
            0,
            0,
            null_mut(),
            null_mut(),
            hinstance,
            null_mut(),
        );

        if handle.is_null() {
            Err(Error::last_os_error())
        } else {
            AddClipboardFormatListener(handle);
            Ok(handle)
        }
    }
}
