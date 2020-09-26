#[derive(Copy, Clone)]
pub struct LaunchpadColor {
    pub red:   u8,
    pub green: u8,
    pub blue:  u8
}

impl LaunchpadColor {
    pub const BLACK: LaunchpadColor = LaunchpadColor { red: 0,   green: 0,   blue: 0   };
    pub const RED: LaunchpadColor = LaunchpadColor   { red: 127, green: 0,   blue: 0   };
    pub const GREEN: LaunchpadColor = LaunchpadColor { red: 0,   green: 127, blue: 0   };
    pub const BLUE: LaunchpadColor = LaunchpadColor  { red: 0,   green: 0,   blue: 127 };
}