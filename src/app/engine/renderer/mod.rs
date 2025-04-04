use anyhow::{Ok, Result};
use std::sync::Arc;
use winit::window::Window;

use super::rendering_context::RenderingContext;

pub struct Renderer {
    context: Arc<RenderingContext>,
}

impl Renderer {
    pub fn new(context: Arc<RenderingContext>, window: Arc<Window>) -> Result<Self> {
        Ok(Self { context })
    }
}
