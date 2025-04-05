mod swapchain;

use anyhow::{Ok, Result};
use std::sync::Arc;
use swapchain::Swapchain;
use winit::window::Window;

use super::rendering_context::RenderingContext;

pub struct Renderer {
    swapchain: Swapchain,
    context: Arc<RenderingContext>,
}

impl Renderer {
    pub fn new(context: Arc<RenderingContext>, window: Arc<Window>) -> Result<Self> {
        let mut swapchain = Swapchain::new(context.clone(), window.clone())?;
        swapchain.resize()?;
        Ok(Self { context, swapchain })
    }

    pub fn resize(&mut self) -> Result<()> {
        self.swapchain.resize()
    }
}
