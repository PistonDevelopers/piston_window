#![deny(missing_docs)]

//! The official Piston window wrapper for the Piston game engine
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
//! ```ignore
//! extern crate piston_window;
//!
//! use piston_window::*;
//!
//! fn main() {
//!     let window: PistonWindow =
//!         WindowSettings::new("Hello World!", [512; 2])
//!             .build().unwrap();
//!     for e in window {
//!         e.draw_2d(|c, g| {
//!             clear([0.5, 0.5, 0.5, 1.0], g);
//!             rectangle([1.0, 0.0, 0.0, 1.0], // red
//!                       [0.0, 0.0, 100.0, 100.0], // rectangle
//!                       c.transform, g);
//!         });
//!     }
//! }
//! ```
//!
//! ### Swap to another window back-end
//!
//! Change the second generic parameter to the window back-end you want to use.
//!
//! ```ignore
//! let window: PistonWindow<(), Sdl2Window> =
//!     WindowSettings::new("title", [512; 2])
//!         .build().unwrap();
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
//! This library is not meant to be depended on by other libraries.
//! Instead, libraries should depend on the lower abstractions,
//! such as the [Piston core](https://github.com/pistondevelopers/piston).

extern crate piston;
extern crate gfx;
extern crate gfx_device_gl;
extern crate gfx_graphics;
extern crate graphics;
extern crate shader_version;
extern crate glutin_window;

use glutin_window::GlutinWindow;
pub use shader_version::OpenGL;
pub use graphics::*;
pub use piston::window::*;
pub use piston::input::*;
pub use piston::event_loop::*;
pub use gfx_graphics::{ GlyphError, Texture, TextureSettings, Flip };

use std::cell::RefCell;
use std::rc::Rc;
use std::any::Any;

use gfx_graphics::{ Gfx2d, GfxGraphics };

/// Actual gfx::Stream implementation carried by the window.
pub type GfxEncoder = gfx::Encoder<gfx_device_gl::Resources,
    gfx_device_gl::CommandBuffer>;
/// Glyph cache.
pub type Glyphs = gfx_graphics::GlyphCache<gfx_device_gl::Resources,
    gfx_device_gl::Factory>;
/// 2D graphics.
pub type G2d<'a> = GfxGraphics<'a,
    gfx_device_gl::Resources,
    gfx_device_gl::CommandBuffer>;
/// Texture type compatible with `G2d`.
pub type G2dTexture<'a> = Texture<gfx_device_gl::Resources>;

/// Contains everything required for controlling window, graphics, event loop.
pub struct PistonWindow<T = (), W: Window = GlutinWindow> {
    /// The window.
    pub window: Rc<RefCell<W>>,
    /// GFX encoder.
    pub encoder: Rc<RefCell<GfxEncoder>>,
    /// GFX device.
    pub device: Rc<RefCell<gfx_device_gl::Device>>,
    /// Output frame buffer.
    pub output_color: Rc<gfx::handle::RenderTargetView<
        gfx_device_gl::Resources, gfx::format::Srgba8>>,
    /// Output stencil buffer.
    pub output_stencil: Rc<gfx::handle::DepthStencilView<
        gfx_device_gl::Resources, gfx::format::DepthStencil>>,
    /// Gfx2d.
    pub g2d: Rc<RefCell<Gfx2d<gfx_device_gl::Resources>>>,
    /// Application structure.
    pub app: Rc<RefCell<T>>,
    /// The factory that was created along with the device.
    pub factory: Rc<RefCell<gfx_device_gl::Factory>>,
}

impl<W> BuildFromWindowSettings for PistonWindow<(), W>
    where W: Window + OpenGLWindow + BuildFromWindowSettings,
          W::Event: GenericEvent
{
    fn build_from_window_settings(mut settings: WindowSettings)
    -> Result<PistonWindow<(), W>, String> {
        // Turn on sRGB.
        settings = settings.srgb(true);

        // Use OpenGL 3.2 by default, because this is what window backends
        // usually do.
        let opengl = settings.get_maybe_opengl().unwrap_or(OpenGL::V3_2);
        let samples = settings.get_samples();

        Ok(PistonWindow::new(opengl, samples,
            Rc::new(RefCell::new(try!(settings.build()))), empty_app()))
    }
}

impl<T, W> Clone for PistonWindow<T, W>
    where W: Window, W::Event: Clone
{
    fn clone(&self) -> Self {
        PistonWindow {
            window: self.window.clone(),
            encoder: self.encoder.clone(),
            device: self.device.clone(),
            output_color: self.output_color.clone(),
            output_stencil: self.output_stencil.clone(),
            g2d: self.g2d.clone(),
            app: self.app.clone(),
            factory: self.factory.clone(),
        }
    }
}

fn create_main_targets(dim: gfx::tex::Dimensions) ->
(Rc<gfx::handle::RenderTargetView<
    gfx_device_gl::Resources, gfx::format::Srgba8>>,
 Rc<gfx::handle::DepthStencilView<
    gfx_device_gl::Resources, gfx::format::DepthStencil>>) {
    use gfx::core::factory::Typed;
    use gfx::format::{DepthStencil, Format, Formatted, Srgba8};

    let color_format: Format = <Srgba8 as Formatted>::get_format();
    let depth_format: Format = <DepthStencil as Formatted>::get_format();
    let (output_color, output_stencil) =
        gfx_device_gl::create_main_targets_raw(dim,
                                               color_format.0,
                                               depth_format.0);
    let output_color = Typed::new(output_color);
    let output_stencil = Typed::new(output_stencil);
    (Rc::new(output_color), Rc::new(output_stencil))
}

impl<T, W> PistonWindow<T, W>
    where W: Window, W::Event: GenericEvent
{
    /// Creates a new piston window.
    pub fn new(
        opengl: OpenGL,
        samples: u8,
        window: Rc<RefCell<W>>,
        app: Rc<RefCell<T>>
    ) -> Self
        where W: OpenGLWindow
    {
        use piston::window::{ OpenGLWindow, Window };
        use gfx::core::factory::Typed;

        let (device, mut factory) =
            gfx_device_gl::create(|s|
                window.borrow_mut().get_proc_address(s) as *const _);

        let (output_color, output_stencil) = {
            let aa = samples as gfx::tex::NumSamples;
            let draw_size = window.borrow().draw_size();
            let dim = (draw_size.width as u16, draw_size.height as u16,
                       1, aa.into());
            create_main_targets(dim)
        };

        let g2d = Gfx2d::new(opengl, &mut factory);
        let encoder = factory.create_command_buffer().into();
        PistonWindow {
            window: window.clone(),
            encoder: Rc::new(RefCell::new(encoder)),
            device: Rc::new(RefCell::new(device)),
            output_color: output_color,
            output_stencil: output_stencil,
            g2d: Rc::new(RefCell::new(g2d)),
            app: app,
            factory: Rc::new(RefCell::new(factory)),
        }
    }

    /// Changes application structure.
    pub fn app<U>(self, app: Rc<RefCell<U>>) -> PistonWindow<U, W> {
        PistonWindow {
            window: self.window,
            encoder: self.encoder,
            device: self.device,
            output_color: self.output_color,
            output_stencil: self.output_stencil,
            g2d: self.g2d,
            app: app,
            factory: self.factory,
        }
    }

    /// Changes application structure.
    #[inline(always)]
    pub fn app_by_value<U>(self, app: U) -> PistonWindow<U, W> {
        self.app(Rc::new(RefCell::new(app)))
    }

    /// Renders 2D graphics.
    pub fn draw_2d<F>(&mut self, e: &Event, f: F) where
        F: FnOnce(Context, &mut G2d)
    {
        use piston::input::RenderEvent;

        if let Some(args) = e.render_args() {
            let mut encoder = self.encoder.borrow_mut();
            self.g2d.borrow_mut().draw(
                &mut encoder,
                &self.output_color,
                &self.output_stencil,
                args.viewport(),
                f
            );
            let mut device = self.device.borrow_mut();
            encoder.flush(&mut *device);
        }
    }

    /// Renders 3D graphics.
    pub fn draw_3d<F>(&mut self, e: &Event, f: F) where
        F: FnOnce(&mut GfxEncoder)
    {
        use piston::input::RenderEvent;

        if let Some(_) = e.render_args() {
            let mut encoder = self.encoder.borrow_mut();
            f(&mut *encoder);
            let mut device = self.device.borrow_mut();
            encoder.flush(&mut *device);
        }
    }
}

impl<T, W> Window for PistonWindow<T, W>
    where W: Window
{
    type Event = <W as Window>::Event;

    fn should_close(&self) -> bool { self.window.borrow().should_close() }
    fn set_should_close(&mut self, value: bool) {
        self.window.borrow_mut().set_should_close(value)
    }
    fn size(&self) -> Size { self.window.borrow().size() }
    fn draw_size(&self) -> Size { self.window.borrow().draw_size() }
    fn swap_buffers(&mut self) { self.window.borrow_mut().swap_buffers() }
    fn poll_event(&mut self) -> Option<Self::Event> {
        Window::poll_event(&mut *self.window.borrow_mut())
    }
}

impl<T, W> AdvancedWindow for PistonWindow<T, W>
    where W: AdvancedWindow
{
    fn get_title(&self) -> String { self.window.borrow().get_title() }
    fn set_title(&mut self, title: String) {
        self.window.borrow_mut().set_title(title)
    }
    fn get_exit_on_esc(&self) -> bool { self.window.borrow().get_exit_on_esc() }
    fn set_exit_on_esc(&mut self, value: bool) {
        self.window.borrow_mut().set_exit_on_esc(value)
    }
    fn set_capture_cursor(&mut self, value: bool) {
        self.window.borrow_mut().set_capture_cursor(value)
    }
}

/// Creates a new empty application.
pub fn empty_app() -> Rc<RefCell<()>> { Rc::new(RefCell::new(())) }
