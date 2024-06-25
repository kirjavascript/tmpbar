use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct Signal<T> {
    ctx: egui::Context,
    state: Arc<Mutex<Option<T>>>,
}

impl<T> Signal<T> {
    pub fn new(ctx: egui::Context) -> Self {
        Self {
            ctx,
            state: Arc::new(Mutex::new(None)),
        }
    }

    pub fn send(&self, state: T) {
        *self.state.lock().unwrap() = Some(state);
        self.ctx.request_repaint();
    }

    pub fn read(&self) -> Option<T> {
        self.state.lock().unwrap().take()
    }
}
