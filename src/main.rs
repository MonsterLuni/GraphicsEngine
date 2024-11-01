use winapi::um::winuser::{VK_UP};
use GraphicsEngine::window::{receive_messages, Color, Point, Window};

fn main() {
    let mut window:Window = Window::new("BestWindow".to_string(), "Class".to_string(), 0, 0, 400, 400);

    unsafe {
        window.show();
        loop {
            window.create_buffer();
            window.change_pencil(1, Color { r: 0, g: 255, b: 0, });
            window.draw_rectangle(Point { x: 200, y: 200 }, 80, 80);

            let ih = window.get_input_handler();
            match ih {
                Some(ih) => {
                    if ih.key_down(VK_UP) {
                        window.change_pencil(2, Color {
                            r: 0,
                            g: 0,
                            b: 255,
                        });
                        window.draw_triangle(Point { x: 20, y: 100 }, Point { x: 20, y: 50 }, Point { x: 45, y: 75 })
                    }
                    if ih.mouse_pos.x >= 200 && ih.mouse_pos.x <= 280 && ih.mouse_pos.y >= 200 && ih.mouse_pos.y <= 280 {
                        window.change_pencil(1, Color { r: 255, g: 255, b: 0, });
                        window.draw_rectangle(Point { x: 200, y: 200 }, 80, 80);
                    }
                    println!("{:?}", ih.mouse_state)
                },
                None => {
                    println!("No input handler found");
                    break;
                }
            }
            window.use_buffer();
            window.update();
            receive_messages();
        }
    }
}