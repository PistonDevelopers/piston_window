# piston_window [![Crates.io](https://img.shields.io/crates/v/piston_window.svg)](https://crates.io/crates/piston_window) [![Crates.io](https://img.shields.io/crates/l/piston_window.svg)](https://github.com/PistonDevelopers/piston_window/blob/master/LICENSE)
The official Piston Window for the Piston game engine

### Example

```rust no_run
use piston_window::*;

fn main() {
    let mut window: PistonWindow =
        WindowSettings::new("Hello World!", [512; 2])
            .build().unwrap();
    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g, _| {
            use graphics::*;
            clear([0.5, 0.5, 0.5, 1.0], g);
            rectangle([1.0, 0.0, 0.0, 1.0], // red
                      [0.0, 0.0, 100.0, 100.0], // rectangle
                      c.transform, g);
        });
    }
}
```

**If you want to dive into the world of Piston, then you can [start here](https://github.com/PistonDevelopers/piston).**

### Design

The purpose of this library is to provide an easy-to-use,
simple-to-get-started and convenient-for-applications API for Piston.

Sets up:

- [piston](https://github.com/PistonDevelopers/piston.git) for the Piston framework
- [piston-texture](https://github.com/pistondevelopers/texture.git) for generic textures
- [piston2d-graphics](https://github.com/pistondevelopers/graphics.git) for 2D graphics
- [piston2d-wgpu_graphics](https://github.com/pistondevelopers/wgpu_graphics)
for 2D rendering
- [wgpu](https://github.com/gfx-rs/wgpu) for 3D rendering

With the Cargo feature "batteries" turned on:

- [bevy](https://github.com/bevyengine/bevy) for Entity-Component-System (ECS) paradigm
- [camera_controllers](https://github.com/PistonDevelopers/camera_controllers.git) for 3D camera
- [collada](https://github.com/PistonDevelopers/piston_collada) for a popular 3D asset format
- [dyon](https://github.com/pistondevelopers/dyon.git) for scripting
- [find_folder](https://github.com/pistondevelopers/find_folder) for searching for a folder from current directory
- [gltf](https://github.com/gltf-rs/gltf) for a popular and efficient 3D asset format for game engines
- [image](https://github.com/image-rs/image) for reading and saving image formats
- [kira](https://github.com/tesselode/kira) for Audio
- [nalgebra](https://github.com/dimforge/nalgebra) for Linear Algebra
- [piston-ai_behavior](https://github.com/pistondevelopers/ai_behavior.git) for AI behavior trees
- [piston-button_controller](https://github.com/pistondevelopers/button_controller) for UI button logic
- [pistoncore-winit_window](https://github.com/pistondevelopers/winit_window)
as window back-end
- [piston_meta](https://github.com/pistondevelopers/meta.git) for Meta-Parsing (human readable documents)
- [piston2d-deform_grid](https://github.com/PistonDevelopers/deform_grid) for deforming textures in 2D
- [piston2d-drag_controller](https://github.com/PistonDevelopers/drag_controller) for mouse drag logic
- [piston2d-sprite](https://github.com/pistondevelopers/sprite.git) for 2D sprite animation
- [rand](https://github.com/rust-random/rand) for generating random numbers
- [rapier2d](https://github.com/dimforge/rapier) for 2D physics
- [rapier3d](https://github.com/dimforge/rapier) for 3D physics
- [texture_packer](https://github.com/PistonDevelopers/texture_packer) for texture packing
- [turbine_scene3d-wgpu](https://github.com/PistonDevelopers/turbine/tree/master/scene3d/backends/wgpu) for WGPU backend for Turbine-Scene3D
- [turbine](https://github.com/PistonDevelopers/turbine) for 3D content production
- [vecmath](https://github.com/pistondevelopers/vecmath.git) for vector math
- [wavefront_obj](https://github.com/PistonDevelopers/wavefront_obj.git) for a popular 3D asset format
- [winit](https://github.com/rust-windowing/winit) for platform window features

To enable the "batteries" feature, you add the following in "Cargo.toml":

```text
piston_window = { version = "*", features = ["batteries"] }
```

These libraries are simply reexported under "piston_window::*", e.g. "piston_window::dyon". This means you can add one dependency in your project
and get a reasonable set of libraries to build your game.

Notice that some of these libraries have turned off certain features to
reduce the number of dependencies. Consult the Cargo documentation to learn
how to include more features per dependency.


### sRGB

The impl of `BuildFromWindowSettings` in this library turns on
`WindowSettings::srgb`, because it is more backward compatible.

Most images such as those found on the internet use sRGB,
that has a non-linear gamma corrected space.
When rendering 3D, make sure textures and colors are in linear gamma space
to get correct color transformation.
Consult the WGPU documentation to learn more about how to do this properly.

For more information about sRGB, see
https://github.com/PistonDevelopers/piston/issues/1014

### Library dependencies

This library is meant to be used in applications only.
It is not meant to be depended on by generic libraries.
Instead, libraries should depend on the lower abstractions,
such as the [Piston core](https://github.com/pistondevelopers/piston).

### Feedback

Ideas or feedback? Open up an issue [here](https://github.com/pistondevelopers/piston_window/issues).
