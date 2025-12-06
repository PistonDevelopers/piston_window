//! Piston Window module.

use piston::{
    AdvancedWindow,
    BuildFromWindowSettings,
    Event,
    Events,
    EventLoop,
    EventSettings,
    GenericEvent,
    Position,
    ResizeArgs,
    Size,
    Window,
    WindowSettings,
};
use graphics::{Context};

use wgpu_graphics::{TextureContext, Wgpu2d, WgpuGraphics};
use std::error::Error;
use std::time::Duration;
use std::sync::Arc;

/// Glyph cache.
pub type Glyphs<'a> = wgpu_graphics::GlyphCache<'a>;
/// Type for 2D graphics.
pub type G2d = wgpu_graphics::Wgpu2d;
/// Texture type compatible with `G2d`.
pub type G2dTexture = wgpu_graphics::Texture;

use winit_window::WinitWindow;
/// Contains everything required for controlling window, graphics, event loop.
pub struct PistonWindow {
    /// The window.
    pub window: WinitWindow,
    /// WGPU device.
    pub device: Arc<wgpu::Device>,
    /// WGPU Command buffer queue.
    pub queue: Arc<wgpu::Queue>,
    /// WGPU surface.
    pub surface: wgpu::Surface<'static>,
    /// WGPU surface config.
    pub surface_config: wgpu::SurfaceConfiguration,
    /// Wgpu2d.
    pub g2d: Wgpu2d,
    /// Event loop state.
    pub events: Events,
}

impl BuildFromWindowSettings for PistonWindow {
    fn build_from_window_settings(
        settings: &WindowSettings,
    ) -> Result<Self, Box<dyn Error>> {
        // Turn on sRGB.
        let settings = settings.clone().srgb(true);
        Ok(PistonWindow::new(settings.build()?))
    }
}

impl PistonWindow {
    /// Creates a new Piston window.
    pub fn new(window: WinitWindow) -> Self {
        use wgpu::{PresentMode, SurfaceConfiguration, TextureFormat};

        fn init_surface_config(window: &WinitWindow) -> SurfaceConfiguration {
            SurfaceConfiguration {
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                format: TextureFormat::Bgra8UnormSrgb,
                width: window.draw_size().width as u32,
                height: window.draw_size().height as u32,
                present_mode: PresentMode::Fifo,
                alpha_mode: wgpu::CompositeAlphaMode::PostMultiplied,
                view_formats: vec![TextureFormat::Bgra8UnormSrgb],
                desired_maximum_frame_latency: Default::default(),
            }
        }

        let instance = wgpu::Instance::new(&Default::default());
        let surface = instance.create_surface(window.get_window()).unwrap();
        let adapter =
            futures::executor::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
                compatible_surface: Some(&surface),
                ..Default::default()
            })).unwrap();

        let mut device_descriptor = wgpu::DeviceDescriptor::default();
        device_descriptor.required_features.set(wgpu::Features::DEPTH_CLIP_CONTROL, true);
        let (device, queue) = futures::executor::block_on(
            adapter.request_device(&device_descriptor),
        ).unwrap();
        let surface_config = init_surface_config(&window);
        surface.configure(&device, &surface_config);
        let device = Arc::new(device);
        let queue = Arc::new(queue);
        let g2d = Wgpu2d::new(device.clone(), &surface_config);
        let events = Events::new(EventSettings::new());
        PistonWindow {
            window,
            events,
            device,
            surface,
            surface_config,
            queue,
            g2d,
        }
    }
}

impl PistonWindow {
    /// Creates context used to create and update textures.
    pub fn create_texture_context(&self) -> TextureContext {
        TextureContext::from_parts(self.device.clone(), self.queue.clone())
    }

    /// Loads font from a path.
    pub fn load_font<'a, P: AsRef<std::path::Path>>(
        &'a self,
        path: P,
    ) -> Result<Glyphs<'a>, std::io::Error> {
        Glyphs::new(
            path,
            self.create_texture_context(),
            texture::TextureSettings::new(),
        )
    }

    /// Renders 2D graphics.
    ///
    /// Calls the closure on render events.
    /// There is no need to filter events manually, and there is no overhead.
    pub fn draw_2d<E, F, U>(&mut self, e: &E, f: F) -> Option<U>
    where
        E: GenericEvent,
        F: FnOnce(Context, &mut WgpuGraphics, &wgpu::Device) -> U,
    {
        if let Some(args) = e.render_args() {
            let surface_texture = self.surface.get_current_texture().unwrap();
            let surface_view = surface_texture.texture
                .create_view(&wgpu::TextureViewDescriptor::default());

            let device = &self.device;
            let (res, command_buffer) = self.g2d.draw(
                &self.surface_config,
                &surface_view,
                args.viewport(),
                |c, g| f(c, g, device));
            self.queue.submit(std::iter::once(command_buffer));
            surface_texture.present();
            Some(res)
        } else {
            None
        }
    }

    /// Renders 3D graphics.
    ///
    /// Calls the closure on render events.
    /// There is no need to filter events manually, and there is no overhead.
    pub fn draw_3d<E, F, U>(&mut self, e: &E, f: F) -> Option<U>
    where
        E: GenericEvent,
        F: FnOnce(&mut Self) -> U,
    {
        if let Some(_) = e.render_args() {Some(f(self))} else {None}
    }

    /// Let window handle new event.
    /// Cleans up after rendering and resizes frame buffers.
    pub fn event<E: GenericEvent>(&mut self, e: &E) {
        e.resize(
            |&ResizeArgs {
                 draw_size: [width, height],
                 ..
             }| {
                self.surface_config = wgpu::SurfaceConfiguration {
                        width,
                        height,
                        ..self.surface_config.clone()
                    };
                    self.surface.configure(&self.device, &self.surface_config);
            },
        );
    }
}

impl Window for PistonWindow {
    fn should_close(&self) -> bool {
        self.window.should_close()
    }
    fn set_should_close(&mut self, value: bool) {
        self.window.set_should_close(value)
    }
    fn size(&self) -> Size {
        self.window.size()
    }
    fn draw_size(&self) -> Size {
        self.window.draw_size()
    }
    fn swap_buffers(&mut self) {
        // Wait for queued commends to finish,
        // so they get included in the frame render.
        let _ = self.device.poll(wgpu::PollType::wait_indefinitely());
        self.window.swap_buffers()
    }
    fn wait_event(&mut self) -> Event {
        Window::wait_event(&mut self.window)
    }
    fn wait_event_timeout(&mut self, timeout: Duration) -> Option<Event> {
        Window::wait_event_timeout(&mut self.window, timeout)
    }
    fn poll_event(&mut self) -> Option<Event> {
        Window::poll_event(&mut self.window)
    }
}

impl AdvancedWindow for PistonWindow {
    fn get_title(&self) -> String {
        self.window.get_title()
    }
    fn set_title(&mut self, title: String) {
        self.window.set_title(title)
    }
    fn get_automatic_close(&self) -> bool {
        self.window.get_automatic_close()
    }
    fn set_automatic_close(&mut self, value: bool) {
        self.window.set_automatic_close(value);
    }
    fn get_exit_on_esc(&self) -> bool {
        self.window.get_exit_on_esc()
    }
    fn set_exit_on_esc(&mut self, value: bool) {
        self.window.set_exit_on_esc(value)
    }
    fn set_capture_cursor(&mut self, value: bool) {
        self.window.set_capture_cursor(value)
    }
    fn show(&mut self) {
        self.window.show()
    }
    fn hide(&mut self) {
        self.window.hide()
    }
    fn get_position(&self) -> Option<Position> {
        self.window.get_position()
    }
    fn set_position<P: Into<Position>>(&mut self, pos: P) {
        self.window.set_position(pos)
    }
    fn set_size<S: Into<Size>>(&mut self, size: S) {
        self.window.set_size(size)
    }
}

impl EventLoop for PistonWindow {
    fn get_event_settings(&self) -> EventSettings {
        self.events.get_event_settings()
    }

    fn set_event_settings(&mut self, settings: EventSettings) {
        self.events.set_event_settings(settings);
    }
}

impl Iterator for PistonWindow {
    type Item = Event;

    /// Returns next event.
    /// Cleans up after rendering and resizes frame buffers.
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(e) = self.events.next(&mut self.window) {
            self.event(&e);
            Some(e)
        } else {
            None
        }
    }
}
