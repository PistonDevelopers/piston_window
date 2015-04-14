#![deny(missing_docs)]

//! The official Piston window back-end for the Piston game engine

extern crate glutin_window;
extern crate piston;
extern crate gfx;
extern crate gfx_device_gl;
extern crate gfx_graphics;
extern crate graphics;

use std::cell::RefCell;
use std::rc::Rc;
use std::any::Any;

use glutin_window::GlutinWindow;
use piston::{ event, window };
use gfx::traits::*;
use gfx_graphics::{ Gfx2d, GfxGraphics };
use graphics::Context;

/// The type of event emitted from event loop.
pub type PistonEvent = event::Event<<GlutinWindow as window::Window>::Event>;

/// Contains everything required for controlling window, graphics, event loop.
#[derive(Clone)]
pub struct PistonWindow<T = ()> {
    /// The window.
    pub window: Rc<RefCell<GlutinWindow>>,
    /// The gfx data.
    pub gfx: Rc<RefCell<Gfx>>,
    /// The event loop.
    pub events: Rc<RefCell<event::events::Events<GlutinWindow, PistonEvent>>>,
    /// The event.
    pub event: Option<PistonEvent>,
    /// Application structure.
    pub app: Rc<RefCell<T>>,
}

/// Contains Gfx data.
pub struct Gfx {
    /// The device.
    pub device: gfx_device_gl::Device,
    /// The output.
    pub output: gfx_device_gl::Output,
    /// The factory.
    pub factory: gfx_device_gl::Factory,
    /// Renderer.
    pub renderer: gfx::render::Renderer<gfx_device_gl::Resources,
        gfx_device_gl::CommandBuffer>,
    /// Gfx2d.
    pub g2d: Gfx2d<gfx_device_gl::Resources>,
}

impl<T> PistonWindow<T> {
    /// Creates a new piston object.
    pub fn new(window: Rc<RefCell<GlutinWindow>>, app: Rc<RefCell<T>>) -> Self {
        use piston::event::Events;
        use piston::window::{ OpenGLWindow, Window };

        let (mut device, mut factory) = gfx_device_gl::create(|s| window.borrow_mut().get_proc_address(s));
        let size = window.borrow().size();
        let output = factory.make_fake_output(size.width as u16, size.height as u16);
        let renderer = factory.create_renderer();
        let g2d = Gfx2d::new(&mut device, &mut factory);

        PistonWindow {
            window: window.clone(),
            gfx: Rc::new(RefCell::new(Gfx {
                device: device,
                output: output,
                factory: factory,
                renderer: renderer,
                g2d: g2d,
            })),
            events: Rc::new(RefCell::new(window.events())),
            event: None,
            app: app,
        }
    }

    /// Changes application structure.
    pub fn app<U>(self, app: Rc<RefCell<U>>) -> PistonWindow<U> {
        PistonWindow {
            window: self.window,
            gfx: self.gfx,
            events: self.events,
            event: self.event,
            app: app,
        }
    }

    /// Renders 2D graphics.
    pub fn draw_2d<F>(&self, f: F)
        where F: FnMut(Context, &mut GfxGraphics<
            gfx_device_gl::Resources, gfx_device_gl::CommandBuffer,
            gfx_device_gl::Output>)
    {
        use piston::event::RenderEvent;

        if let Some(ref e) = self.event {
            if let Some(args) = e.render_args() {
                let &mut Gfx { ref mut device, ref mut renderer, ref mut output,
                    ref mut g2d, .. } = &mut *self.gfx.borrow_mut();
                g2d.draw(renderer, output, args.viewport(), f);
                device.submit(renderer.as_buffer());
                renderer.reset();
            }
        }
    }
}

impl Iterator for PistonWindow {
    type Item = PistonWindow;

    fn next(&mut self) -> Option<PistonWindow> {
        use piston::event::AfterRenderEvent;

        if let Some(e) = self.events.borrow_mut().next() {
            if let Some(_) = e.after_render_args() {
                // After swapping buffers.
                let &mut Gfx {
                    ref mut device,
                    ref mut factory,
                    ..
                } = &mut *self.gfx.borrow_mut();
                device.after_frame();
                factory.cleanup();
            }

            Some(PistonWindow {
                window: self.window.clone(),
                gfx: self.gfx.clone(),
                events: self.events.clone(),
                event: Some(e),
                app: self.app.clone(),
            })
        } else { None }
    }
}

impl event::GenericEvent for PistonWindow {
    fn event_id(&self) -> event::EventId {
        match self.event {
            Some(ref e) => e.event_id(),
            None => event::EventId("")
        }
    }

    fn with_args<'a, F, U>(&'a self, f: F) -> U
       where F: FnMut(&Any) -> U
    {
        self.event.as_ref().unwrap().with_args(f)
    }

    fn from_args(event_id: event::EventId, any: &Any, old_event: &Self) -> Option<Self> {
        if let Some(ref e) = old_event.event {
            match event::GenericEvent::from_args(event_id, any, e) {
                Some(e) => {
                    Some(PistonWindow {
                        window: old_event.window.clone(),
                        gfx: old_event.gfx.clone(),
                        events: old_event.events.clone(),
                        event: Some(e),
                        app: old_event.app.clone(),
                    })
                }
                None => None
            }
        } else { None }
    }
}

impl window::Window for PistonWindow {
    type Event = <GlutinWindow as window::Window>::Event;

    fn should_close(&self) -> bool { self.window.borrow().should_close() }
    fn size(&self) -> window::Size { self.window.borrow().size() }
    fn draw_size(&self) -> window::Size { self.window.borrow().draw_size() }
    fn swap_buffers(&mut self) { self.window.borrow_mut().swap_buffers() }
    fn poll_event(&mut self) -> Option<Self::Event> {
        window::Window::poll_event(&mut *self.window.borrow_mut())
    }
}

impl window::AdvancedWindow for PistonWindow {
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
