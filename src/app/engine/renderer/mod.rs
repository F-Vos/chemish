use std::sync::Arc;

use anyhow::{Ok, Result};
use winit::window::Window;

pub struct Renderer {}

impl Renderer {
    pub fn new(window: Arc<Window>) -> Result<Self> {
        Ok(Self {})
    }
}
