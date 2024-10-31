use std::ptr::{null, null_mut};
use winapi::ctypes::c_int;
use winapi::shared::minwindef::{HINSTANCE, LPARAM, LRESULT, WPARAM};
use winapi::shared::windef::{HDC, HGDIOBJ, HPEN, HWND, LPPOINT};
use winapi::um::libloaderapi::GetModuleHandleW;
use winapi::um::wingdi::{CreatePen, DeleteObject, LineTo, MoveToEx, SelectObject, PS_SOLID, RGB};
use winapi::um::winuser::{CreateWindowExW, DefWindowProcW, DispatchMessageW, GetDC, GetMessageW, RegisterClassW, ShowWindow, TranslateMessage, UpdateWindow, MSG, SW_SHOW, WNDCLASSW, WS_OVERLAPPEDWINDOW};

unsafe extern "system"  fn window_proc(hwnd: HWND, msg: u32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    match msg {
        _ => DefWindowProcW(hwnd, msg, w_param, l_param),
    }
}
fn to_w_string(s: &str) -> Vec<u16> {
    let mut v: Vec<u16> = s.encode_utf16().collect();
    v.push(0);
    v
}
pub struct Point{
    pub x: i32,
    pub y: i32
}
pub struct Color{
    pub r: u8,
    pub g: u8,
    pub b: u8
}
pub struct Window{
    name:String,
    class_name:String,
    pos_x:u32,
    pos_y:u32,
    height:u32,
    width:u32,
    class:WNDCLASSW,
    h_instance:HINSTANCE,
    hwnd: HWND,
    hdc: HDC
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
        unsafe { RegisterClassW(&class) };
        let hwnd:HWND = unsafe { CreateWindowExW(0, to_w_string(&class_name).as_ptr(), to_w_string(&name).as_ptr(), WS_OVERLAPPEDWINDOW, pos_x as c_int, pos_y as c_int, width as c_int, height as c_int, null_mut(), null_mut(), h_instance, null_mut()) };
        let hdc = unsafe { GetDC(hwnd) };
        Self { name, class_name, pos_x, pos_y, height, width, class, h_instance, hwnd, hdc }
    }
    pub unsafe fn show(&self) {
        let _ = ShowWindow(self.hwnd, SW_SHOW);
        self.update();
    }

    pub fn update(&self) {
        unsafe{
            UpdateWindow(self.hwnd);
        }
    }
    pub unsafe fn draw_line(&self, start_point:&Point,end_point:&Point) {
        MoveToEx(self.hdc, start_point.x, start_point.y, 0 as LPPOINT);
        LineTo(self.hdc, end_point.x, end_point.y);
    }
    pub unsafe fn draw_rectangle(&self,start_point:Point,width:u32,height:u32){
        self.draw_line(&start_point, &Point { x: start_point.x + width as i32, y: start_point.y });
        self.draw_line(&start_point, &Point { x: start_point.x, y: start_point.y + height as i32 });
        self.draw_line(&Point { x: start_point.x + width as i32, y: start_point.y }, &Point{x:start_point.x + width as i32,y:start_point.y + height as i32});
        self.draw_line(&Point { x: start_point.x, y: start_point.y + height as i32},&Point{x:start_point.x + width as i32,y:start_point.y + height as i32});
    }
    pub unsafe fn draw_triangle(&self,p1: Point,p2: Point,p3: Point){
        self.draw_line(&p1, &p2);
        self.draw_line(&p1, &p3);
        self.draw_line(&p2, &p3);
    }
    pub unsafe fn change_pencil(&self, width:u32 ,color:Color){
        let pen:HPEN = CreatePen(PS_SOLID as c_int, width as c_int, RGB(color.r, color.g, color.b));
        let old_pen:HGDIOBJ = SelectObject(self.hdc,pen as _);
        DeleteObject(old_pen);
    }
    pub unsafe fn receive_message(&self) {
        let mut msg: MSG = std::mem::zeroed();
        let message_result = GetMessageW(&mut msg, self.hwnd, 0, 0) > 0;
        if message_result{
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }
}
pub unsafe fn receive_messages() {
    let mut msg: MSG = std::mem::zeroed();
    let message_result = GetMessageW(&mut msg,null_mut(), 0, 0) > 0;
    if message_result{
        TranslateMessage(&msg);
        DispatchMessageW(&msg);
    }
}
