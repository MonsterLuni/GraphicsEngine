mod window;

use crate::window::Window;

fn main() {
    let window: Window = Window::new("myClass".to_string(), "myFenster".to_string(), 100, 100, 200, 200);
    window.show();
}