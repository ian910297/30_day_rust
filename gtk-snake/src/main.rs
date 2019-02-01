extern crate gdk;
extern crate gio;
extern crate gtk;

use gio::prelude::*;
use gtk::prelude::*;

pub mod gui;
pub mod config;

pub struct Game {
    score: i32
}

impl Game {
    pub fn new() -> Self {
        Game {
            score: 0
        }
    }
}

fn main() {
    const APP_ID: &str = "io.github.ian910297.gtk-snake";
    const WINDOW_WIDTH: i32 = 600;
    const WINDOW_HEIGHT: i32 = 800;
    const TILE: i32 = 10;
    const STYLE: &[u8] = include_bytes!("style.css");

    let game = Game::new();

    match gtk::Application::new(APP_ID, gio::ApplicationFlags::FLAGS_NONE) {
        Ok(app) => {
            app.connect_activate(|app| gui::setup_ui(app));
            app.run(&[]);
        },
        Err(_) => {
            println!("Failed to initialize GTK-SNAKE");
        }
    }
}