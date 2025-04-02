use anyhow::{Ok, Result};
use app::App;
use winit::event_loop::EventLoop;
mod app;
fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let mut app = App::default();

    let event_loop = EventLoop::new()?;
    event_loop.run_app(&mut app)?;
    Ok(())
}
