mod window;

use crate::window::{Color, Point, Window};

fn main() {
    let window: Window = Window::new("Heureka".to_string(), "Class".to_string(), 100, 100, 200, 200);
    window.show();

    window.draw_line(&Point{x: 10,y: 50},Point{x: 10,y: 100});
    window.change_pencil(2,Color{r: 255,g: 50,b: 20});
    window.draw_rectangle(&Point{x: 50,y: 50},50,50);

    window.get_input();
}