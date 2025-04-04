mod engine;

use engine::Engine;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    window::{WindowAttributes, WindowId},
};

#[derive(Default)]
pub struct App {
    engine: Option<Engine>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.engine = Some(Engine::new(event_loop).unwrap());
        if let Some(engine) = self.engine.as_mut() {
            let _secondary_window = engine
                .create_window(
                    event_loop,
                    WindowAttributes::default().with_title("Sec window"),
                )
                .unwrap();
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        if let Some(engine) = self.engine.as_mut() {
            engine.window_event(event_loop, window_id, event);
        }
    }
    fn suspended(&mut self, event_loop: &ActiveEventLoop) {
        let _ = event_loop;
    }

    fn exiting(&mut self, event_loop: &ActiveEventLoop) {
        let _ = event_loop;
    }
}
