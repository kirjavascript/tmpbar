use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct Signal<T> {
    ctx: egui::Context,
    state: Arc<Mutex<Vec<T>>>,
}

impl<T> Signal<T> {
    pub fn new(ctx: egui::Context) -> Self {
        Self {
            ctx,
            state: Arc::new(Mutex::new(vec![])),
        }
    }

    pub fn send(&self, state: T) {
        self.state.lock().unwrap().push(state);
        self.ctx.request_repaint();
    }

    pub fn read(&self) -> Vec<T> {
        std::mem::replace(&mut self.state.lock().unwrap(), vec![])
    }

    pub fn consume(&self) -> bool {
        let mut r#ref = self.state.lock().unwrap();
        let has = r#ref.len() > 0;
        r#ref.clear();
        has
    }
}
