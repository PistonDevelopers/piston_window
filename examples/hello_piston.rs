extern crate piston_window;

use piston_window::*;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Hello Piston!", [640, 480])
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|e| { panic!("Failed to build PistonWindow: {}", e) });
    let mut events: WindowEvents = window.events();
    println!("Press any button to enter inner loop");
    while let Some(e) =  events.next(&mut window) {
        window.draw_2d(&e, |_c, g| {
            clear([0.5, 1.0, 0.5, 1.0], g);
        });

        if let Some(button) = e.press_args() {
            println!("Pressed {:?}", button);
            println!("Press X to exit inner loop");
            while let Some(e) = events.next(&mut window) {
                window.draw_2d(&e, |_c, g| {
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
