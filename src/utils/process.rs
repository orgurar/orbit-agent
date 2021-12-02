use std::ptr;
use winapi::um::wincon::GetConsoleWindow;
use winapi::um::winuser::{ShowWindow, SW_HIDE};

// credit: https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindow
pub fn hide_console_window() {
    let window = unsafe { GetConsoleWindow() };
    if window != ptr::null_mut() {
        unsafe {
            // set windows show property to hide
            ShowWindow(window, SW_HIDE);
        }
    }
}
