#![deny(missing_docs)]

//! The official Piston window wrapper for the Piston game engine
//!
//! **Notice! If this is your first time visiting Piston, [start here](https://github.com/PistonDevelopers/piston).**
//!
//! The purpose of this library is to provide an easy-to-use,
//! simple-to-get-started and convenient-for-applications API for Piston.
//!
//! Sets up:
//!
//! - [Gfx](https://github.com/gfx-rs/gfx) with an OpenGL back-end.
//! - [gfx_graphics](https://github.com/pistondevelopers/gfx_graphics)
//! for 2D rendering.
//! - [glutin_window](https://github.com/pistondevelopers/glutin_window)
//! as default window back-end, but this can be swapped (see below).
//!
//! ### Example
//!
//! ```no_run
//! extern crate piston_window;
//!
//! use piston_window::*;
//!
//! fn main() {
//!     let mut window: PistonWindow =
//!         WindowSettings::new("Hello World!", [512; 2])
//!             .build().unwrap();
//!     while let Some(e) = window.next() {
//!         window.draw_2d(&e, |c, g, _| {
//!             clear([0.5, 0.5, 0.5, 1.0], g);
//!             rectangle([1.0, 0.0, 0.0, 1.0], // red
//!                       [0.0, 0.0, 100.0, 100.0], // rectangle
//!                       c.transform, g);
//!         });
//!     }
//! }
//! ```
//!
//! The `draw_2d` function calls the closure on render events.
//! There is no need to filter events manually, and there is no overhead.
//!
//! ### Swap to another window back-end
//!
//! Change the generic parameter to the window back-end you want to use.
//!
//! ```ignore
//! extern crate piston_window;
//! extern crate sdl2_window;
//!
//! use piston_window::*;
//! use sdl2_window::Sdl2Window;
//!
//! # fn main() {
//!
//! let window: PistonWindow<Sdl2Window> =
//!     WindowSettings::new("title", [512; 2])
//!         .build().unwrap();
//!
//! # }
//! ```
//!
//! ### sRGB
//!
//! The impl of `BuildFromWindowSettings` in this library turns on
//! `WindowSettings::srgb`, because it is required by gfx_graphics.
//!
//! Most images such as those found on the internet uses sRGB,
//! that has a non-linear gamma corrected space.
//! When rendering 3D, make sure textures and colors are in linear gamma space.
//! Alternative is to use `Srgb8` and `Srgba8` formats for textures.
//!
//! For more information about sRGB, see
//! https://github.com/PistonDevelopers/piston/issues/1014
//!
//! ### Library dependencies
//!
//! This library is meant to be used in applications only.
//! It is not meant to be depended on by generic libraries.
//! Instead, libraries should depend on the lower abstractions,
//! such as the [Piston core](https://github.com/pistondevelopers/piston).

extern crate gfx;
extern crate gfx_device_gl;
extern crate gfx_graphics;
extern crate piston;
extern crate shader_version;

pub extern crate graphics;
pub extern crate texture;

/// Exports all of the types exposed by this module, *except* for `graphics`.
///
/// The `graphics` module contains a module and function called `image`,
/// which is very likely to conflict with the `image` crate.
///
/// Using the name "prelude" also suppresses the wildcard import warning from clippy.
pub mod prelude;

pub use graphics::*;
pub use prelude::*;
