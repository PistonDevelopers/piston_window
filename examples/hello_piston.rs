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
            window = inner_event_loop(window.app(InnerApp {
                title: "Inner loop (press X to exit inner loop)",
                exit_button: Button::Keyboard(Key::X),
            }));
            window.set_title(title.into());
        }
    }
}

/// Stores application state of inner event loop.
pub struct InnerApp {
    pub title: &'static str,
    pub exit_button: Button,
}

fn inner_event_loop(mut window: PistonWindow<InnerApp>) -> PistonWindow {
    window.set_title(window.app.title.into());
    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g| {
            clear([0.5, 0.5, 1.0, 1.0], g);
            ellipse([1.0, 0.0, 0.0, 1.0], [50.0, 50.0, 100.0, 100.0], c.transform, g);
        });
        if let Some(button) = e.press_args() {
            if button == window.app.exit_button {
                break;
            }
        }
    }

    // Go back to default app state.
    window.app(())
}
