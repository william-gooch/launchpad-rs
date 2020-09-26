#![allow(dead_code)]

pub mod color;
pub mod state;
pub mod event;

pub mod launchpad_x;

use std::sync::{Arc, Mutex};

use launchpad_x::LaunchpadX;
use color::LaunchpadColor;
use state::LaunchpadState;
use event::*;

pub trait LaunchpadOutput {
    fn set_all_lights(&mut self, color: LaunchpadColor);
    fn set_light(&mut self, x: usize, y: usize, color: LaunchpadColor);
    fn set_state(&mut self, lights: LaunchpadState);
    fn clear_grid(&mut self);
    fn set_box(&mut self, x: usize, y: usize, width: usize, height: usize, color: LaunchpadColor);
}

pub trait Launchpad: LaunchpadOutput + Send {
    fn get_event(&self) -> &Arc<Mutex<LaunchpadEvent>>;
    fn set_event_handler(&self, handler: Box<dyn LaunchpadEventHandler>);
}

pub fn create_launchpad() -> Result<Box<dyn Launchpad>, Box<dyn std::error::Error>> {        
    let launchpad = LaunchpadX::init()?;

    Ok(launchpad)
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn color_constants() {
        assert_eq!(LaunchpadColor::BLACK.red, 0)
    }
}