pub mod window{
    use std::collections::HashMap;
    use std::ptr::{null, null_mut};
    use winapi::ctypes::c_int;
    use winapi::shared::minwindef::{HINSTANCE, LPARAM, LRESULT, TRUE, WPARAM};
    use winapi::shared::windef::{HBITMAP, HDC, HGDIOBJ, HPEN, HWND, LPPOINT, LPRECT, RECT};
    use winapi::um::libloaderapi::GetModuleHandleW;
    use winapi::um::wingdi::{BitBlt, CreateCompatibleBitmap, CreateCompatibleDC, CreatePen, DeleteDC, DeleteObject, LineTo, MoveToEx, SelectObject, PS_SOLID, RGB, SRCCOPY};
    use winapi::um::winuser::{CreateWindowExW, DefWindowProcW, DispatchMessageW, GetClientRect, GetDC, GetKeyState, GetMessageW, GetWindowLongPtrW, InvalidateRect, RegisterClassW, SetWindowLongPtrW, ShowWindow, TranslateMessage, UpdateWindow, GWLP_USERDATA, MSG, SW_SHOW, VK_LBUTTON, VK_MBUTTON, VK_RBUTTON, WM_KEYDOWN, WM_KEYUP, WM_LBUTTONDOWN, WM_LBUTTONUP, WM_MBUTTONDOWN, WM_MBUTTONUP, WM_MOUSEMOVE, WM_RBUTTONDOWN, WM_RBUTTONUP, WNDCLASSW, WS_OVERLAPPEDWINDOW};

    unsafe extern "system"  fn window_proc(hwnd: HWND, msg: u32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
        let user_data_ptr = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut UserData;
        if !user_data_ptr.is_null() {
            let user_data = &mut *user_data_ptr;
            match msg{
                WM_MOUSEMOVE => {
                    user_data.input_handler.mouse_pos = Point{x:(l_param & 0xFFFF) as i16 as i32,y:((l_param >> 16) & 0xFFFF) as i16 as i32};
                }
                WM_KEYUP => {
                    user_data.input_handler.register_key(w_param as i32, false);
                }
                WM_KEYDOWN => {
                    user_data.input_handler.register_key(w_param as i32, true);
                }
                // Mouse
                WM_LBUTTONDOWN => {
                    user_data.input_handler.mouse_state.insert(VK_LBUTTON,true);
                }
                WM_LBUTTONUP => {
                    user_data.input_handler.mouse_state.insert(VK_LBUTTON,false);
                }
                WM_RBUTTONDOWN => {
                    user_data.input_handler.mouse_state.insert(VK_RBUTTON,true);
                }
                WM_RBUTTONUP => {
                    user_data.input_handler.mouse_state.insert(VK_RBUTTON,false);
                }
                WM_MBUTTONDOWN => {
                    user_data.input_handler.mouse_state.insert(VK_MBUTTON,true);
                }
                WM_MBUTTONUP => {
                    user_data.input_handler.mouse_state.insert(VK_MBUTTON,false);
                }
                _ => {}
            }
        }
        match msg{
            _ => {DefWindowProcW(hwnd, msg, w_param, l_param)}
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
        class:WNDCLASSW,
        h_instance:HINSTANCE,
        hwnd: HWND,
        hdc: HDC,
        buffer_hdc: Buffer,
    }
    struct UserData {
        input_handler: InputHandler
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
            let user_data = Box::new(UserData {
                input_handler: InputHandler::new(),
            });
            let user_data_ptr = Box::into_raw(user_data) as isize;
            unsafe { SetWindowLongPtrW(hwnd, GWLP_USERDATA, user_data_ptr) };
            Self { name, class_name, pos_x, pos_y, class, h_instance, hwnd, hdc, buffer_hdc: Buffer::None }
        }
        pub unsafe fn show(&self) {
            let _ = ShowWindow(self.hwnd, SW_SHOW);
        }
        pub unsafe fn update(&self) {
            InvalidateRect(self.hwnd,null_mut(),TRUE);
            UpdateWindow(self.hwnd);
        }
        pub unsafe fn create_buffer(&mut self){
            match self.buffer_hdc {
                Buffer::Some(_) => {
                    println!("There's already a buffer running");
                }
                Buffer::None => {
                    let size:(u32,u32) = self.get_size();
                    let hdc = CreateCompatibleDC(self.hdc);
                    let h_bitmap:HBITMAP = CreateCompatibleBitmap(self.hdc, size.0 as c_int, size.1 as c_int);
                    SelectObject(hdc, h_bitmap as _);
                    self.buffer_hdc = Buffer::Some(hdc);
                }
            }
        }
        pub unsafe fn use_buffer(&mut self){
            match self.buffer_hdc {
                Buffer::Some(hdc) => {
                    let size:(u32,u32) = self.get_size();
                    BitBlt(self.hdc, 0, 0, size.0 as c_int, size.1 as c_int, hdc, 0, 0, SRCCOPY);
                    let old_bitmap: HGDIOBJ = SelectObject(hdc, null_mut());
                    DeleteObject(old_bitmap);
                    DeleteDC(hdc);
                    self.buffer_hdc = Buffer::None;
                }
                Buffer::None => {
                    println!("There's no buffer running");
                }
            }

        }
        pub unsafe fn get_input_handler(&self) -> Option<&InputHandler>{
            let user_data_ptr = GetWindowLongPtrW(self.hwnd, GWLP_USERDATA) as *mut UserData;
            if !user_data_ptr.is_null() {
                let user_data = &mut *user_data_ptr;
                Some(&user_data.input_handler)
            }else{
                None
            }
        }
        pub unsafe fn draw_line(&self, start_point:&Point,end_point:&Point) {
            match self.buffer_hdc {
                Buffer::Some(hdc) => {
                    MoveToEx(hdc, start_point.x, start_point.y, 0 as LPPOINT);
                    LineTo(hdc, end_point.x, end_point.y);
                }
                Buffer::None => {
                    MoveToEx(self.hdc, start_point.x, start_point.y, 0 as LPPOINT);
                    LineTo(self.hdc, end_point.x, end_point.y);
                }
            }
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
            match self.buffer_hdc {
                Buffer::Some(hdc) => {
                    let pen:HPEN = CreatePen(PS_SOLID as c_int, width as c_int, RGB(color.r, color.g, color.b));
                    let old_pen:HGDIOBJ = SelectObject(hdc,pen as _);
                    DeleteObject(old_pen);
                }
                Buffer::None => {
                    let pen:HPEN = CreatePen(PS_SOLID as c_int, width as c_int, RGB(color.r, color.g, color.b));
                    let old_pen:HGDIOBJ = SelectObject(self.hdc,pen as _);
                    DeleteObject(old_pen);
                }
            }
        }
        pub unsafe fn fill(&self) {
            let mut i:u32 = 0;
            let size:(u32,u32) = self.get_size();
            while i < size.0{
                self.draw_line(&Point { x: i as i32, y: 0 }, &Point { x: i as i32, y: size.1 as i32 });
                i += 1;
            };
        }
        pub unsafe fn get_size(&self) -> (u32, u32){
            let mut rect = RECT{left: 0, top: 0, right: 0, bottom: 0};
            let lprect:LPRECT = &mut rect;
            GetClientRect(self.hwnd, lprect);
            (rect.right as u32, rect.bottom as u32)
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
    pub enum Buffer {
        Some(HDC),
        None,
    }
    pub struct InputHandler {
        pub key_states: HashMap<c_int,bool>,
        pub mouse_state: HashMap<c_int,bool>,
        pub mouse_pos: Point
    }
    impl InputHandler{
        pub fn new() -> Self{ InputHandler{ key_states: HashMap::new(), mouse_state: HashMap::new(), mouse_pos: Point { x: -1, y: -1 } } }
        fn register_key(&mut self, key:c_int, value:bool){
            self.key_states.insert(key, value);
        }
        pub fn key_down(&self, key:c_int) -> bool{
            match self.key_states.get(&key) {
                Some(value) => {
                    *value
                }
                None => {false}
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
}