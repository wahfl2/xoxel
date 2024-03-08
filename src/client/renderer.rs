use std::sync::Arc;

use winit::window::Window;

use self::backend::Backend;

mod backend;

pub struct Renderer {
    pub backend: Backend,
}

impl Renderer {
    pub fn new(window: Arc<Window>) -> Self {
        let backend = Backend::new(window);
        
        Self { backend }
    }
}