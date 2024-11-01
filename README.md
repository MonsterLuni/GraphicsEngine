# GraphicsEngine
This Library is the right thing for you 
if you want to make a rust application which needs easy control over one or multiple windows.
The API that I am writing will have the most important functions for drawing on a window.
## Usage
The most simple program would be a blank window which registers inputs that the user gives.
use GraphicsEngine::window::Window;

```
use GraphicsEngine::window::Window;

fn main() {
    let window: Window = Window::new("WindowName".to_string(), "WindowClass".to_string(), 0, 0, 500, 500);
    unsafe {
        window.show();
        loop {
            window.receive_message();
        }
    }
}
```
Or maybe something a little bit more elaborate, 
like a Window that automatically fills in the Background Red and has a Blue Rectangle in the left upper corner
```
use GraphicsEngine::window::{Color, Window};
use GraphicsEngine::window::Point;

fn main() {
    let window: Window = Window::new("WindowName".to_string(), "WindowClass".to_string(), 0, 0, 500, 500);
    unsafe {
        window.show();
        loop {
            window.change_pencil(0, Color { r: 255, g: 0, b: 0, });
            window.fill();
            window.change_pencil(0, Color { r: 0, g: 0, b: 255, });
            window.draw_rectangle(Point { x: 0, y: 0 }, 50, 50);
            window.receive_message();
        }
    }
}
```
Keep in mind that the code above has a flaw. There will be flickering occuring as soon as the user does something with the window.
To remove this nasty flaw there are two simple functions to call { create_buffer() and use_buffer() }
```
use GraphicsEngine::window::{Color, Window};
use GraphicsEngine::window::Point;

fn main() {
    let mut window: Window = Window::new("WindowName".to_string(), "WindowClass".to_string(), 0, 0, 500, 500);
    unsafe {
        window.show();
        loop {
            window.create_buffer();

            window.change_pencil(0, Color { r: 255, g: 0, b: 0, });
            window.fill();
            window.change_pencil(0, Color { r: 0, g: 0, b: 255, });
            window.draw_rectangle(Point { x: 0, y: 0 }, 50, 50);

            window.use_buffer();
            window.receive_message();
        }
    }
}
```
If you want to update the window you just have to call the { update() } function

## Things that will follow
I want to add...  <br>

..documentation to every function so that everybody has a better time using this library <br>
..a way to easily add key and mouse inputs to the window <br>

Any other thing I'm working on is visible as an Issue in this GitLab Project 