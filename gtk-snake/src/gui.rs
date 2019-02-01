extern crate gdk;
extern crate gio;
extern crate gtk;

use gio::prelude::*;
use gtk::prelude::*;

use crate::config::*;

pub fn setup_ui(app: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(app);
    let config = Config::new();

    window.set_title(config.title);
    window.set_default_size(config.width, config.height);
    window.connect_delete_event(move |win, _| {
      win.destroy();
      Inhibit(false)
    });

    let label = gtk::Label::new("Hello");
    window.add(&label);
    window.show_all();
}