use std::sync::Arc;

use winit::{dpi::PhysicalSize, event::{Event, WindowEvent}, event_loop::EventLoop, window::{Window, WindowBuilder}};

use self::renderer::Renderer;

mod renderer;
mod event_handler;

pub struct Client {
    pub event_loop: EventLoop<()>,
    pub window: Arc<Window>,
    pub renderer: Renderer
}

impl Client {
    pub fn new() -> Self {
        let event_loop = EventLoop::new().unwrap();
        let window_builder = WindowBuilder::new()
            .with_inner_size(PhysicalSize::new(1600, 900));

        let window = window_builder.build(&event_loop).unwrap();
        let window = Arc::new(window);

        let renderer = Renderer::new(window.clone());

        Self {
            event_loop,
            window,
            renderer
        }
    }

    pub fn run(self) {
        self.event_loop.run(|event, target| {
            if let Event::WindowEvent { event: WindowEvent::CloseRequested, .. } = event {
                return target.exit();
            }
        })
        .unwrap();
    }
}
