# piston_window
The official Piston convenience window wrapper for the Piston game engine

This window wrapper focuses on simple-to-get-started API that integrates well with the Piston libraries. The source can be used as example for making your own window back-end that fits your purposes.

- Events includes everything, application state, window and graphics resources
- Pass around the application state as a single parameter
- Nested game loops is possible everywhere, giving advanced flow logic control
- Convenient methods for rendering 2D and 3D
- Sets up Gfx (3D graphics) automatically

### Design

The motivation by having a convenience wrapper is to reduce the amount of boilerplate to set up a new project, and to use a design pattern that works nicely in Rust. The basic idea is to use one object for everything.

`PistonWindow` = window + event iterator + event + graphics resources + application state

For example, many games require some sort of global application state that stores all the resources shared by the different parts of the game logic. Piston Window uses `Rc<RefCell<T>>` to share the application state between parts where lifetimes might be inconvenient. You can swap out the application state with another, using the `.app(new_app_state)` method.

All controller libraries in Piston, such as drag controller or a camera controller, require `E: GenericEvent`. This is implemented by `PistonWindow` such that you can call `controller.event(e)`. `GenericEvent` is found in the [event](https://github.com/pistondevelopers/event) library that is reexported in the [piston](https://github.com/pistondevelopers/piston) crate. The event design in Piston is the simplest possible to abstract over the underlying structure, supporting future hardware and allow custom events. For example, you can write your own window wrapper that adds multiplayer networking and then wrap `PistonWindow` around it. All event traits are implemented for all types implementing `GenericEvent`. If you share the event traits in a separate library, other people will then be able to implement their own network library and controllers using your events, and it will work across all the window back-ends.

If you want to run a custom scene in your game, you can use `for e in e.clone() { ... }` to create a nested game loop. This allows you to swap out the game logic entirely, so you can write mini games as separate libraries targeting `PistonWindow` and then glue it together in a larger project.

Piston libraries do not depend on "piston_window" directly but uses traits either by the Piston core or other back-end abstractions such as Gfx (3D graphics). The purpose is to make it easier to share code across projects, even if they use different back-ends.

### Dependency graph

![Dependencies](./Cargo.png)
