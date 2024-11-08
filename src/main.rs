use winapi::um::winuser::VK_LBUTTON;
use GraphicsEngine::window::{receive_messages, Color, Point, Window};

fn main() {
    let mut window: Window = Window::new("WindowName".to_string(), "WindowClass".to_string(), 0, 0, 500, 500);

    let mut right = 0;
    unsafe {
        window.show();
        loop {
            window.create_buffer();
            window.change_pencil(1, Color { r: 0, g: 0, b: 0, });
            window.fill();
            let input_handler = window.get_input_handler();
            if input_handler.unwrap().mouse_down(VK_LBUTTON) {
                window.change_pencil(1, Color { r: 0, g: 255, b: 0, });
                right += 1;
            }else{
                window.change_pencil(1, Color { r: 0, g: 0, b: 255, });
            }
            window.draw_string("Hello World!", Point { x: 100, y: 100 }, Color { r: 255, g: 0, b: 0, });
            window.draw_circle(Point { x: 100, y: 100 }, 100);
            window.draw_ellipse(50, 80, 90, 0);
            window.draw_triangle(Point { x: 10 + right, y: 10 }, Point { x: 10 + right, y: 80 }, Point { x: 100 + right, y: 80 });

            window.use_buffer();
            window.update();
            receive_messages();
        }
    }
}