mod renderer;

use anyhow::Result;
use renderer::Renderer;
use std::{collections::HashMap, sync::Arc};
use tracing::event;
use winit::{
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    window::{Window, WindowAttributes, WindowId},
};

pub struct Engine {
    renderers: HashMap<WindowId, Renderer>,
    windows: HashMap<WindowId, Arc<Window>>,
    primary_window_id: WindowId,
}

impl Engine {
    pub fn new(event_loop: &ActiveEventLoop) -> Result<Self> {
        let primary_window = Arc::new(event_loop.create_window(Default::default())?);
        let primary_window_id = primary_window.id();
        let windows = HashMap::from([(primary_window_id, primary_window)]);

        let renderers = windows
            .iter()
            .map(|(id, window)| {
                let renderer = Renderer::new(window.clone()).unwrap();
                (*id, renderer)
            })
            .collect::<HashMap<_, _>>();
        Ok(Self {
            renderers,
            windows,
            primary_window_id,
        })
    }

    pub fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                if window_id == self.primary_window_id {
                    event_loop.exit();
                } else {
                    self.windows.remove(&window_id);
                    self.renderers.remove(&window_id);
                }
            }
            _ => {}
        }
    }

    pub fn create_window(
        &mut self,
        event_loop: &ActiveEventLoop,
        attributes: WindowAttributes,
    ) -> Result<WindowId> {
        let window = Arc::new(event_loop.create_window(attributes)?);
        let window_id = window.id();
        self.windows.insert(window_id, window.clone());

        let renderer = Renderer::new(window)?;
        self.renderers.insert(window_id, renderer);

        Ok(window_id)
    }
}
