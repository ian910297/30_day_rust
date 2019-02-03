use gio::prelude::*;
use gtk::prelude::*;
use gtk::{
    Application, ApplicationWindow, Button, DrawingArea, Label, 
    Orientation, WindowPosition};
use std::{cell::RefCell, rc::Rc, collections::HashMap};
use std::sync::{Arc, Mutex};
use crate::config::*;
use crate::game::*;

pub fn setup_ui(app: &Application) {
    let mut game = Game::new();
    let window = ApplicationWindow::new(app);
    let config = Config::new();

    window.set_title(config.title);
    window.set_default_size(config.width, config.height);
    window.set_position(WindowPosition::Center);
    window.connect_delete_event(move |win, _| {
        win.destroy();
        Inhibit(false)
    });
    
    // Container
    let container = gtk::Box::new(Orientation::Vertical, 0);
    // Menu box
    let menu_box = gtk::Box::new(Orientation::Horizontal, 10);
    let intro_str = Label::new("GTK SNARK");
    let start_btn = Button::new_with_label("Start");
    let quit_btn = Button::new_with_label("Quit");
    menu_box.pack_start(&intro_str, true, true, 0);
    menu_box.pack_start(&start_btn, true, false, 0);
    menu_box.pack_start(&quit_btn, true, false, 0);

    // Score board
    let score_board_box = gtk::Box::new(Orientation::Horizontal, 10);
    let score_str = Label::new("Score: 0");
    score_board_box.pack_start(&score_str, true, true, 0);
    
    // Drawing Area
    let game_box = gtk::Box::new(Orientation::Vertical, 5);
    let canvas = DrawingArea::new();
    game_box.pack_start(&canvas, true, true, 0);

    container.pack_start(&menu_box, false, false, 0);
    container.pack_start(&score_board_box, false, false, 0);
    container.pack_start(&game_box, true, false, 0);
    window.add(&container);
    window.show_all();

    let tick = move || {
        let curr_score = game.increase_score();
        score_str.set_text(&curr_score);
        gtk::Continue(true)
    };
    gtk::timeout_add_seconds(1, tick);
}