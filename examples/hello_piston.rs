extern crate piston_window;

use piston_window::*;

fn main() {
    let title = "Hello Piston! (press any key to enter inner loop)";
    let mut window: PistonWindow = WindowSettings::new(title, [640, 480])
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|e| { panic!("Failed to build PistonWindow: {}", e) });
    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g| {
            clear([0.5, 1.0, 0.5, 1.0], g);
            rectangle([1.0, 0.0, 0.0, 1.0], [50.0, 50.0, 100.0, 100.0], c.transform, g);
        });

        if e.press_args().is_some() {
            window.set_title("Inner loop (press X to exit inner loop)".into());
            while let Some(e) = window.next() {
                window.draw_2d(&e, |c, g| {
                    clear([0.5, 0.5, 1.0, 1.0], g);
                    ellipse([1.0, 0.0, 0.0, 1.0], [50.0, 50.0, 100.0, 100.0], c.transform, g);
                });

                // Inner loop.
                if let Some(Button::Keyboard(Key::X)) = e.press_args() {
                    break;
                }
            }
            window.set_title(title.into());
        }
    }
}
