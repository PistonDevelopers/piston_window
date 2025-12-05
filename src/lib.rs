#![deny(missing_docs)]
#![doc = include_str!("../README.md")]

/// Exports all of the types exposed by this module, *except* for `graphics`.
///
/// The `graphics` module contains a module and function called `image`,
/// which is very likely to conflict with the `image` crate.
///
/// Using the name "prelude" also suppresses the wildcard import warning from clippy.
pub mod prelude;

pub mod piston_window;

pub use prelude::*;

pub use graphics;
pub use piston;
pub use texture;
pub use wgpu;
pub use wgpu_graphics;

// Reexport commonly used libraries.
#[cfg(feature = "batteries")]
pub use ai_behavior;
#[cfg(feature = "batteries")]
pub use bevy;
#[cfg(feature = "batteries")]
pub use camera_controllers;
#[cfg(feature = "batteries")]
pub use collada;
#[cfg(feature = "batteries")]
pub use dyon;
#[cfg(feature = "batteries")]
pub use gltf;
#[cfg(feature = "batteries")]
pub use image;
#[cfg(feature = "batteries")]
pub use kira;
#[cfg(feature = "batteries")]
pub use nalgebra;
#[cfg(feature = "batteries")]
pub use piston_meta;
#[cfg(feature = "batteries")]
pub use sprite;
#[cfg(feature = "batteries")]
pub use rapier2d;
#[cfg(feature = "batteries")]
pub use rapier3d;
#[cfg(feature = "batteries")]
pub use texture_packer;
#[cfg(feature = "batteries")]
pub use vecmath;
#[cfg(feature = "batteries")]
pub use wavefront_obj;
#[cfg(feature = "batteries")]
pub use winit;
#[cfg(feature = "batteries")]
pub use winit_window;
