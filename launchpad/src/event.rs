pub enum LaunchpadEventArgs {
    Released { x: usize, y: usize },
    Pressed { x: usize, y: usize },
}

#[derive(Default)]
pub struct LaunchpadEvent {
    callbacks: Vec<Box<dyn LaunchpadEventHandler>>
}

impl LaunchpadEvent {
    pub fn subscribe(&mut self, handler: Box<dyn LaunchpadEventHandler>) {
        self.callbacks.push(handler);
    }

    pub fn trigger(&self, args: LaunchpadEventArgs) {
        for handler in self.callbacks.iter() {
            handler.notify(&args);
        }
    }
}

pub trait LaunchpadEventHandler: Send + Sync {
    fn notify(&self, event: &LaunchpadEventArgs);
}
