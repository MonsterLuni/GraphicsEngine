mod window;

use crate::window::{Point, Window};

fn main() {
    let window: Window = Window::new("Heureka".to_string(), "Class".to_string(), 100, 100, 200, 200);


    window.show();
    window.draw_line(&Point{x: 20,y: 50},Point{x: 50,y: 50});
    window.draw_rectangle(&Point{x: 50,y: 50},50,50);

    window.get_input();
}