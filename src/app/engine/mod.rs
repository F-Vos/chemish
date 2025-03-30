mod renderer;

use std::{collections::HashMap, sync::Arc};

use anyhow::Result;
use renderer::Renderer;
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowId};

pub struct Engine {
    renderers: HashMap<WindowId, Renderer>,
    windows: HashMap<WindowId, Arc<Window>>,
    primary_window_id: WindowId,
}

impl Engine {
    pub fn new(event_loop: &ActiveEventLoop) -> Result<Self> {
        let raw_window = event_loop.create_window(Default::default())?;
        let primary_window = Arc::new(raw_window);
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
}
