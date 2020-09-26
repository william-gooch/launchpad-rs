mod shortcuts;

use launchpad::*;
use launchpad::event::*;

use std::io::stdin;
use std::sync::{Arc, Mutex, MutexGuard};
use std::process::exit;

use shortcuts::*;

use lazy_static::lazy_static;
use enigo::*;

lazy_static! {
    static ref ENIGO: Arc<Mutex<Enigo>> = Arc::new(Mutex::new(Enigo::new()));
}

struct Application {
    launchpad: Arc<Mutex<Box<dyn Launchpad>>>,

    pub pages: ShortcutPages,
    pub hotbar: ShortcutHotbar,
    pub current_page: usize
}

impl Application {
    pub fn new() -> Arc<Mutex<Application>> {
        let launchpad = Arc::new(Mutex::new(create_launchpad().unwrap()));

        let lp_clone = launchpad.clone();

        let application = Arc::new(Mutex::new(Application {
            launchpad,
            pages: ShortcutPages::default(),
            hotbar: ShortcutHotbar::default(),
            current_page: 0
        }));

        lp_clone.lock().unwrap().set_event_handler(Box::new(ApplicationEventHandler { application: application.clone() }));

        application
    }

    fn render(&mut self) {
        let mut locked = self.launchpad.lock().unwrap();

        Application::clear(&mut locked);
        Application::render_tab_bar(&mut locked, self.current_page);
        Application::render_hot_bar(&mut locked, &self.hotbar);
        Application::render_page(&mut locked, &self.pages.pages[self.current_page]);
    }

    fn clear(launchpad: &mut MutexGuard<Box<dyn Launchpad>>) {
        launchpad.clear_grid();
    }

    fn render_tab_bar(launchpad: &mut MutexGuard<Box<dyn Launchpad>>, page: usize) {
        launchpad.set_box(0, 0, 8, 1, color::LaunchpadColor::BLUE);
        launchpad.set_light(page, 0, color::LaunchpadColor::RED);
    }

    fn render_hot_bar(launchpad: &mut MutexGuard<Box<dyn Launchpad>>, hotbar: &ShortcutHotbar) {
        for y in 0..8 {
            match &hotbar.shortcuts[y] {
                Some(shortcut) => {
                    launchpad.set_light(8, y+1, shortcut.color);
                },
                _ => ()
            }
        }
    }

    fn render_page(launchpad: &mut MutexGuard<Box<dyn Launchpad>>, page: &ShortcutPage) {
        for y in 0..8 {
            for x in 0..8 {
                match &page.shortcuts[y][x] {
                    Some(shortcut) => {
                        launchpad.set_light(x, y+1, shortcut.color);
                    }
                    _ => ()
                }
            }
        }
    }
}

struct ApplicationEventHandler {
    application: Arc<Mutex<Application>>
}

impl LaunchpadEventHandler for ApplicationEventHandler {
    fn notify(&self, args: &LaunchpadEventArgs) {
        match args {
            LaunchpadEventArgs::Pressed { x, y } => {
                if *y == 0 {
                    let mut locked = self.application.lock().unwrap();
                    locked.current_page = *x;
                    locked.render();
                } else if *x == 8 {
                    let locked = self.application.lock().unwrap();
                    match &locked.hotbar.shortcuts[*y-1] {
                        Some(shortcut) => {
                            shortcut.invoke();
                        },
                        None => ()
                    }
                } else {
                    let locked = self.application.lock().unwrap();
                    match &locked.pages.pages[locked.current_page].shortcuts[*x][*y-1] {
                        Some(shortcut) => {
                            shortcut.invoke();
                        },
                        None => ()
                    }
                }
            },
            _ => ()
        }
    }
}

fn wait() {
    let mut input = String::new();
    stdin().read_line(&mut input);
}

fn main() {
    let app = Application::new();
    {
        let mut locked = app.lock().unwrap();
        locked.pages.pages[0].shortcuts[0][0] = Some(Shortcut {
            callback: Box::new(|| {
                ENIGO.lock().unwrap().key_sequence("hello world");
            }),
            color: color::LaunchpadColor { red: 0, green: 87, blue: 107 }
        });
        locked.hotbar.shortcuts[7] = Some(Shortcut {
            callback: Box::new(|| {
                exit(0);
            }),
            color: color::LaunchpadColor::RED
        });
        locked.render();
    }
    wait();
}