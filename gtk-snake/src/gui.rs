use gio::prelude::*;
use gtk::prelude::*;
use gtk::{
    Application, ApplicationWindow, Button, DrawingArea, Label, 
    Orientation, WindowPosition};
use gdk::ModifierType;
use gdk::enums::key;
use std::sync::{Arc, Mutex};
use crate::config::*;
use crate::game::*;

pub fn setup_ui(app: &Application) {
    let game = Arc::new(Mutex::new(Game::new()));
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
    let restart_btn = Button::new_with_label("Restart");
    
    menu_box.pack_start(&intro_str, true, true, 0);
    menu_box.pack_start(&start_btn, true, false, 0);
    menu_box.pack_start(&restart_btn, true, false, 0);

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

    {
        // Keyboard input
        // Control directions and speedup
        let game = game.clone();
        
        window.connect_key_press_event(move |_, gdk| {
            let mut game = game.lock().unwrap();

            let speedup = gdk.get_state().contains(ModifierType::CONTROL_MASK);
            if speedup {
                game.speedup = true;
            } else {
                game.speedup = false;
            }

            let direction = match gdk.get_keyval() {
                65362 | 119 | 87 => Some(Direction::Up),
                65364 | 115 | 83 => Some(Direction::Down),
                65361 | 97 | 56 => Some(Direction::Left),
                65363 | 100 | 68 => Some(Direction::Right),
                _ => None,
            };
            if direction.is_some() {
                game.direction = direction.unwrap();
            }

            Inhibit(false)
        });
    }
    
    {
        // Draw map: fruit, snake, map
        canvas.connect_draw(move |_, ctx| {
            Inhibit(false)
        });
    }

    {
        // Start button event
        let game = game.clone();

        start_btn.clone().connect_clicked(move |btn| {
            let mut game = game.lock().unwrap();
            if (game.state == State::Init) ||
               (game.state == State::Pause) 
            {
                btn.set_label("pause");
                game.run();
            } else if game.state == State::Running {
                btn.set_label("continue");
                game.pause();
            }
        });
    }
    
    {
        // Restart button event
        let game = game.clone();
        let start_btn = start_btn.clone();
        let score_str = score_str.clone();
        let time_str = time_str.clone();

        restart_btn.connect_clicked(move |btn| {
            let mut game = game.lock().unwrap();
            start_btn.set_label("Start");
            game.reset();
            score_str.set_text("Score: 0");
            time_str.set_text("Time: 100");
        });
    }

    {
        // Loop round
        let game = game.clone();
        let score_str = score_str.clone();
        let time_str = time_str.clone();

        let tick = move || {
            let mut game = game.lock().unwrap();

            if game.state == State::Running {
                let (curr_score, curr_time) = game.update();
                score_str.set_text(&curr_score);
                time_str.set_text(&curr_time);
            } else {
                
            }

            gtk::Continue(true)
        };

        gtk::timeout_add_seconds(1, tick);
    }
}