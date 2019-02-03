use gio::prelude::*;
use gtk::prelude::*;
use gtk::{
    Application, ApplicationWindow, Button, DrawingArea, Label, 
    Orientation, WindowPosition};
use std::{rc::Rc, cell::RefCell};
use crate::config::*;
use crate::game::*;

pub fn setup_ui(app: &Application) {
    let game = Rc::new(RefCell::new(Game::new()));
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
    let game_clone = game.clone();
    start_btn.connect_clicked(move |btn| {
        if game_clone.borrow().is_running() {
            btn.set_label("start");
            game_clone.borrow_mut().pause();
        } else {
            btn.set_label("pause");
            game_clone.borrow_mut().run();
        }
    });
    menu_box.pack_start(&intro_str, true, true, 0);
    menu_box.pack_start(&start_btn, true, false, 0);
    menu_box.pack_start(&quit_btn, true, false, 0);

    // Score board
    let score_board_box = gtk::Box::new(Orientation::Horizontal, 10);
    let score_str = Label::new("Score: 0");
    let time_str = Label::new("Time: 100");
    score_board_box.pack_start(&score_str, true, true, 0);
    score_board_box.pack_start(&time_str, true, true, 0);
    
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
        if game.borrow().is_running()  {
            let (curr_score, curr_time) = game.borrow_mut().update();
            score_str.set_text(&curr_score);
            time_str.set_text(&curr_time);
        } else {
            
        }

        gtk::Continue(true)
    };

    gtk::timeout_add_seconds(1, tick);
}