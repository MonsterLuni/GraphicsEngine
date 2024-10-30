use std::ptr::{null, null_mut};
use std::thread;
use std::time::Duration;
use winapi::ctypes::c_int;
use winapi::um::winuser::{CreateWindowExW, DefWindowProcW, DispatchMessageW, GetDC, GetMessageW, RegisterClassW, ShowWindow, TranslateMessage, UpdateWindow, MSG, SW_SHOW, WNDCLASSW, WS_OVERLAPPEDWINDOW};
use winapi::um::libloaderapi::GetModuleHandleW;
use winapi::shared::windef::{HWND};
use winapi::um::wingdi::{LineTo, Rectangle};
use winapi::shared::minwindef::{LRESULT, WPARAM, LPARAM, HINSTANCE};
extern "system" fn window_proc(hwnd: HWND, msg: u32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    match msg{
        _ => unsafe { DefWindowProcW(hwnd, msg, w_param, l_param) }
    }
}
fn to_w_string(s: &str) -> Vec<u16> {
    let mut v: Vec<u16> = s.encode_utf16().collect();
    v.push(0); // Null-Terminierung
    v
}
pub struct Window{
    pub name:String,
    pub class_name:String,
    pub pos_x:u32,
    pub pos_y:u32,
    pub height:u32,
    pub width:u32,
    class:WNDCLASSW,
    h_instance:HINSTANCE,
    hwnd: HWND
}
impl Window {
    pub fn new(name: String, class_name: String, pos_x: u32, pos_y: u32, height: u32, width: u32) -> Self {
        let h_instance = unsafe { GetModuleHandleW(null()) };
        let class: WNDCLASSW = WNDCLASSW {
            style: 0,
            lpfnWndProc: Some(window_proc),
            cbClsExtra: 0,
            lpszClassName: to_w_string(&class_name).as_ptr(),
            hInstance: h_instance,
            hIcon: null_mut(),
            hCursor: null_mut(),
            hbrBackground: null_mut(),
            cbWndExtra: 0,
            lpszMenuName: null_mut(),
        };
        Self { name, class_name, pos_x, pos_y, height, width, class, h_instance }
    }
    pub fn show(&self) {
        unsafe { RegisterClassW(&self.class) };
        let hwnd:HWND = unsafe { CreateWindowExW(0, to_w_string(&self.class_name).as_ptr(), to_w_string(&self.name).as_ptr(), WS_OVERLAPPEDWINDOW, self.pos_x as c_int, self.pos_y as c_int, self.width as c_int, self.height as c_int, null_mut(), null_mut(), self.h_instance, null_mut()) };
        unsafe {
            ShowWindow(hwnd, SW_SHOW);
            UpdateWindow(hwnd);
        };
        let mut msg: MSG = unsafe { std::mem::zeroed() };
        while unsafe { GetMessageW(&mut msg, null_mut(), 0, 0) } > 0 {
            unsafe {
                TranslateMessage(&msg);
                UpdateWindow(hwnd);
                DispatchMessageW(&msg);
            };
            thread::sleep(Duration::from_millis(1));
        };
    }
    pub fn draw(&self){
        unsafe{
            let hdc = GetDC(self);
        }
    }
}