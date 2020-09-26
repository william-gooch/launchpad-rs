use launchpad::color::LaunchpadColor;

pub struct Shortcut {
    pub color: LaunchpadColor,
    pub callback: Box<dyn Fn() + Sync + Send>
}

impl Shortcut {
    pub fn invoke(&self) {
        (self.callback)();
    }
}


#[derive(Default)]
pub struct ShortcutPage {
    pub shortcuts: [[Option<Shortcut>;8];8]
}

#[derive(Default)]
pub struct ShortcutHotbar {
    pub shortcuts: [Option<Shortcut>;8]
}

#[derive(Default)]
pub struct ShortcutPages {
    pub pages: [ShortcutPage;8]
}