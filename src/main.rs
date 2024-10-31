pub mod window;

use crate::window::{receive_messages, Color, Point, Window};

fn main() {
    let window: Window = Window::new("Heureka".to_string(), "Class".to_string(), 100, 100, 200, 200);
    let window2: Window = Window::new("Heureka".to_string(), "ClassToInt".to_string(), 100, 100, 200, 200);
    unsafe {
        window.show();
        window2.show();
        loop {
            gui(&window);
            gui(&window2);
            receive_messages();
        }
    }
}
unsafe fn gui(window:&Window) {
    window.draw_line(&Point { x: 10, y: 50 }, &Point { x: 10, y: 100 });
    window.change_pencil(2, Color { r: 0, g: 0, b: 255 });
    window.draw_rectangle(Point { x: 50, y: 50 }, 50, 50);
    window.change_pencil(2, Color { r: 255, g: 0, b: 0 });
    window.draw_triangle(Point { x: 100, y: 50 }, Point { x: 100, y: 100 }, Point { x: 150, y: 100 });
}