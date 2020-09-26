pub use launchpad::{create_launchpad, Launchpad, LaunchpadOutput};
pub use launchpad::color::LaunchpadColor;
pub use launchpad::state::LaunchpadState;
pub use launchpad::event::*;

pub use std::io::stdin;
pub use std::sync::{Arc, Mutex};

pub struct MyEventHandler {
    launchpad: Arc<Mutex<Box<dyn Launchpad>>>
}

impl LaunchpadEventHandler for MyEventHandler {
    fn notify(&self, args: &LaunchpadEventArgs) {
        let mut launchpad = self.launchpad.lock().unwrap();

        match args {
            LaunchpadEventArgs::Pressed { x, y } => launchpad.set_light(*x, *y, LaunchpadColor::GREEN),
            LaunchpadEventArgs::Released { x, y } => launchpad.set_light(*x, *y, LaunchpadColor::BLACK)
        }
    }
}

fn main() {
    match create_launchpad() {
        Ok(launchpad) => {
            let launchpad = Arc::new(Mutex::new(launchpad));

            {
                let mut locked = launchpad.lock().unwrap();
                locked.set_event_handler(Box::new(MyEventHandler { launchpad: launchpad.clone() }));

                let mut state = LaunchpadState::new();

                state.clear_grid();
                state.set_light(2, 2, LaunchpadColor::RED);
                state.set_box(0, 0, 8, 1, LaunchpadColor::BLUE);
                state.set_light(4, 0, LaunchpadColor::GREEN);

                locked.set_state(state);
            }

            let mut input = String::new();
            stdin().read_line(&mut input);
        },
        Err(err) => println!("Error: {}", err)
    };
}
