use crate::LaunchpadOutput;
use crate::color::LaunchpadColor;

pub struct LaunchpadState {
    lights: [[LaunchpadColor;9];9]
}

impl LaunchpadState {
    pub fn new() -> LaunchpadState {
        LaunchpadState { lights: [[LaunchpadColor::BLACK;9];9] }
    }

    pub fn get_lights(&self) -> &[[LaunchpadColor;9];9] {
        &self.lights
    }
}

impl LaunchpadOutput for LaunchpadState {
    fn set_all_lights(&mut self, color: LaunchpadColor) {
        for y in 0..9 {
            for x in 0..9 {
                self.lights[y][x] = color;
            }
        }
    }

    fn set_light(&mut self, x: usize, y: usize, color: LaunchpadColor) {
        self.lights[y][x] = color;
    }

    fn set_state(&mut self, state: LaunchpadState) {
        for y in 0..9 {
            for x in 0..9 {
                self.lights[y][x] = state.get_lights()[y][x];
            }
        }
    }

    fn clear_grid(&mut self) {
        self.set_all_lights(LaunchpadColor::BLACK);
    }

    fn set_box(&mut self, x: usize, y: usize, width: usize, height: usize, color: LaunchpadColor) {
        for y in y..y+height {
            for x in x..x+width {
                self.lights[y][x] = color;
            }
        }
    }
}