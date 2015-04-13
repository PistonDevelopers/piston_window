extern crate piston_window;
extern crate glutin_window;
extern crate piston;

use std::cell::RefCell;
use std::rc::Rc;
use glutin_window::{ OpenGL, GlutinWindow };
use piston_window::*;
use piston::window::{ WindowSettings, Size };

fn main() {
    let window = Rc::new(RefCell::new(GlutinWindow::new(
        OpenGL::_3_2,
        WindowSettings::new("Loading...".to_string(),
            Size { width: 640, height: 480 })
            .exit_on_esc(true)
    )));
    for mut e in PistonWindow::new(window, empty_app()) {
        use piston::event::*;
        use piston::window::*;

        e.set_title("Hello Piston!".to_string());
        if let Some(button) = e.press_args() {
            use piston::input::{ Button, Key };

            println!("Pressed {:?}", button);
            println!("Press X to exit inner loop");
            for e in e {
                // Inner loop.
                if let Some(Button::Keyboard(Key::X)) = e.press_args() {
                    break;
                }
            }
            println!("You have exit the inner loop");
        }
    }
}
