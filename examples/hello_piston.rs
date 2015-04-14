extern crate piston_window;
extern crate glutin_window;
extern crate piston;
extern crate graphics;

use std::cell::RefCell;
use std::rc::Rc;
use glutin_window::{ OpenGL, GlutinWindow };
use piston_window::*;
use piston::window::{ WindowSettings, Size };

use piston::event::*;
use graphics::*;
use piston::input::*;

fn main() {
    let window = Rc::new(RefCell::new(GlutinWindow::new(
        OpenGL::_3_2,
        WindowSettings::new("Hello Piston!".to_string(),
            Size { width: 640, height: 480 })
            .exit_on_esc(true)
    )));
    println!("Press a button to enter inner loop");
    for e in PistonWindow::new(window, empty_app()) {
        e.draw_2d(|_c, g| {
            clear([0.5, 1.0, 0.5, 1.0], g);
        });

        if let Some(button) = e.press_args() {
            println!("Pressed {:?}", button);
            println!("Press X to exit inner loop");
            for e in e.clone() {
                e.draw_2d(|_c, g| {
                    clear([0.5, 0.5, 1.0, 1.0], g);
                });

                // Inner loop.
                if let Some(Button::Keyboard(Key::X)) = e.press_args() {
                    break;
                }
            }
            println!("You have exit the inner loop");
        }
    }
}
