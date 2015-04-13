#![deny(missing_docs)]

//! The official Piston window back-end for the Piston game engine

extern crate glutin_window;
extern crate piston;

use std::cell::RefCell;
use std::rc::Rc;
use std::any::Any;

use glutin_window::GlutinWindow;
use piston::{ event, window };

/// The type of event emitted from event loop.
pub type PistonEvent = event::Event<<GlutinWindow as window::Window>::Event>;

/// Contains everything required for controlling window, graphics, event loop.
#[derive(Clone)]
pub struct PistonWindow<T = ()> {
    /// The window.
    pub window: Rc<RefCell<GlutinWindow>>,
    /// The event loop.
    pub events: Rc<RefCell<event::events::Events<GlutinWindow, PistonEvent>>>,
    /// The event.
    pub event: Option<PistonEvent>,
    /// Application structure.
    pub app: Rc<RefCell<T>>,
}

impl<T> PistonWindow<T> {
    /// Creates a new piston object.
    pub fn new(window: Rc<RefCell<GlutinWindow>>, app: Rc<RefCell<T>>) -> Self {
        use piston::event::Events;

        PistonWindow {
            window: window.clone(),
            events: Rc::new(RefCell::new(window.events())),
            event: None,
            app: app,
        }
    }
}

impl Iterator for PistonWindow {
    type Item = PistonWindow;

    fn next(&mut self) -> Option<PistonWindow> {
        if let Some(e) = self.events.borrow_mut().next() {
            Some(PistonWindow {
                window: self.window.clone(),
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
