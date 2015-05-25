extern crate piston_window;

use piston_window::*;

fn main() {
    let window: PistonWindow = WindowSettings::new("Hello Piston!", [640, 480])
        .exit_on_esc(true)
        .into();
    println!("Press any button to enter inner loop");
    for e in window {
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
            println!("You have exited the inner loop");
        }
    }
}
