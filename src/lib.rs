#![deny(missing_docs)]

//! The official Piston window wrapper for the Piston game engine
//!
//! Sets up [Gfx](https://github.com/gfx-rs/gfx) with an OpenGL back-end.  
//! Uses [gfx_graphics](https://github.com/pistondevelopers/gfx_graphics)
//! for 2D rendering.  
//! Uses [glutin_window](https://github.com/pistondevelopers/glutin_window)
//! as default window back-end, but this can be swapped.  
//!
//! sRGB is turned on because it is required by gfx_graphics.
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
//! ### Do not depend on this library
//!
//! This library is not meant to be depended on by other libraries.  
//! Instead, libraries should depend on the lower abstractions,
//! such as the [Piston core](https://github.com/pistondevelopers/piston).  
//! The only purpose of this library is to provide a convenient way to get started.  

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

use gfx::traits::*;
use gfx_graphics::{ Gfx2d, GfxGraphics };

/// Actual gfx::Stream implementation carried by the window.
pub type GfxEncoder = gfx::Encoder<gfx_device_gl::Resources,
    gfx_device_gl::command::CommandBuffer>;
/// Glyph cache.
pub type Glyphs = gfx_graphics::GlyphCache<gfx_device_gl::Resources,
    gfx_device_gl::Factory>;
/// 2D graphics.
pub type G2d<'a> = GfxGraphics<'a,
    gfx_device_gl::Resources,
    gfx_device_gl::command::CommandBuffer>;

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
        gfx_device_gl::Resources, gfx::format::Srgb8>>,
    /// Output stencil buffer.
    pub output_stencil: Rc<gfx::handle::DepthStencilView<
        gfx_device_gl::Resources, gfx::format::DepthStencil>>,
    /// Gfx2d.
    pub g2d: Rc<RefCell<Gfx2d<gfx_device_gl::Resources>>>,
    /// The event loop.
    pub events: Rc<RefCell<WindowEvents>>,
    /// The event.
    pub event: Option<Event<W::Event>>,
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
            events: self.events.clone(),
            event: self.event.clone(),
            app: self.app.clone(),
            factory: self.factory.clone(),
        }
    }
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
        use piston::event_loop::Events;
        use piston::window::{ OpenGLWindow, Window };

        let (device, mut factory) =
            gfx_device_gl::create(|s|
                window.borrow_mut().get_proc_address(s) as *const _);

        let draw_size = window.borrow().draw_size();
        let aa = samples as gfx::tex::NumSamples;
        let dim = (draw_size.width as u16, draw_size.height as u16,
                   1, aa.into());
        let (output_color, output_stencil) =
            gfx_device_gl::create_main_targets(dim);
        let g2d = Gfx2d::new(opengl, &mut factory);
        let encoder = factory.create_encoder();
        PistonWindow {
            window: window.clone(),
            encoder: Rc::new(RefCell::new(encoder)),
            device: Rc::new(RefCell::new(device)),
            output_color: Rc::new(output_color),
            output_stencil: Rc::new(output_stencil),
            g2d: Rc::new(RefCell::new(g2d)),
            events: Rc::new(RefCell::new(window.borrow().events())),
            event: None,
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
            events: self.events,
            event: self.event,
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
    pub fn draw_2d<F>(&self, f: F) where
        F: FnOnce(Context, &mut G2d)
    {
        use piston::input::RenderEvent;

        if let Some(ref e) = self.event {
            if let Some(args) = e.render_args() {
                let mut encoder = self.encoder.borrow_mut();
                encoder.reset();
                self.g2d.borrow_mut().draw(
                    &mut encoder,
                    &self.output_color,
                    &self.output_stencil,
                    args.viewport(),
                    f
                );
                self.device.borrow_mut().submit(encoder.as_buffer());
            }
        }
    }

    /// Renders 3D graphics.
    pub fn draw_3d<F>(&self, f: F) where
        F: FnOnce(&mut GfxEncoder)
    {
        use piston::input::RenderEvent;

        if let Some(ref e) = self.event {
            if let Some(_) = e.render_args() {
                let mut encoder = self.encoder.borrow_mut();
                encoder.reset();
                f(&mut *encoder);
                self.device.borrow_mut().submit(encoder.as_buffer());
            }
        }
    }
}

impl<T, W> Iterator for PistonWindow<T, W>
    where W: Window, W::Event: GenericEvent
{
    type Item = PistonWindow<T, W>;

    fn next(&mut self) -> Option<PistonWindow<T, W>> {
        use piston::input::*;
        use gfx::core::factory::Phantom;

        let window = &mut *self.window.borrow_mut();
        if let Some(e) = self.events.borrow_mut().next(window) {
            if let Some(_) = e.after_render_args() {
                // After swapping buffers.
                self.device.borrow_mut().cleanup();
            }

            // Check whether window has resized and update the output.
            let dim = self.output_color.raw().get_dimensions();
            let (w, h) = (dim.0, dim.1);
            let draw_size = window.draw_size();
            let (output_color, output_stencil) =
                if w != draw_size.width as u16 || h != draw_size.height as u16 {
                    let dim = (draw_size.width as u16,
                               draw_size.height as u16,
                               dim.2, dim.3);
                    let (output_color, output_stencil) =
                        gfx_device_gl::create_main_targets(dim);
                    (Rc::new(output_color), Rc::new(output_stencil))
                } else {
                    (self.output_color.clone(), self.output_stencil.clone())
                };

            Some(PistonWindow {
                window: self.window.clone(),
                encoder: self.encoder.clone(),
                device: self.device.clone(),
                output_color: output_color,
                output_stencil: output_stencil,
                g2d: self.g2d.clone(),
                events: self.events.clone(),
                event: Some(e),
                app: self.app.clone(),
                factory: self.factory.clone(),
            })
        } else { None }
    }
}

impl<T, W> GenericEvent for PistonWindow<T, W>
    where W: Window, W::Event: GenericEvent
{
    fn event_id(&self) -> EventId {
        match self.event {
            Some(ref e) => e.event_id(),
            None => EventId("")
        }
    }

    fn with_args<'a, F, U>(&'a self, f: F) -> U
       where F: FnMut(&Any) -> U
    {
        self.event.as_ref().unwrap().with_args(f)
    }

    fn from_args(
        event_id: EventId,
        any: &Any,
        old_event: &Self
    ) -> Option<Self> {
        if let Some(ref e) = old_event.event {
            match GenericEvent::from_args(event_id, any, e) {
                Some(e) => {
                    Some(PistonWindow {
                        window: old_event.window.clone(),
                        encoder: old_event.encoder.clone(),
                        device: old_event.device.clone(),
                        output_color: old_event.output_color.clone(),
                        output_stencil: old_event.output_stencil.clone(),
                        g2d: old_event.g2d.clone(),
                        events: old_event.events.clone(),
                        event: Some(e),
                        app: old_event.app.clone(),
                        factory: old_event.factory.clone(),
                    })
                }
                None => None
            }
        } else { None }
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

impl<T, W> EventLoop for PistonWindow<T, W>
    where W: Window
{
    fn set_ups(&mut self, frames: u64) {
        self.events.borrow_mut().set_ups(frames);
    }

    fn set_max_fps(&mut self, frames: u64) {
        self.events.borrow_mut().set_max_fps(frames);
    }

    fn set_swap_buffers(&mut self, enable: bool) {
        self.events.borrow_mut().set_swap_buffers(enable);
    }

    fn set_bench_mode(&mut self, enable: bool) {
        self.events.borrow_mut().set_bench_mode(enable);
    }
}

/// Creates a new empty application.
pub fn empty_app() -> Rc<RefCell<()>> { Rc::new(RefCell::new(())) }
