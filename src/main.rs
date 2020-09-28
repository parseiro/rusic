extern crate gio;
extern crate gtk;

use std::env;

use gio::{ApplicationExt, ApplicationExtManual, ApplicationFlags};
use gtk::{Application, ApplicationWindow, GtkWindowExt, WidgetExt};
use gtk::{
    ContainerExt,
    SeparatorToolItem,
    Toolbar,
    ToolButton,
};

use crate::toolbar::MusicToolbar;

mod toolbar;

struct App {
    toolbar: MusicToolbar,
    window: ApplicationWindow,
}

impl App {
    fn new(application: Application) -> Self {
        let window = ApplicationWindow::new(&application);
        window.set_title("Rusic");

        let toolbar = MusicToolbar::new();
        window.add(toolbar.toolbar());
        window.show_all();

        let app = App {
            toolbar,
            window,
        };

        app.connect_events();

        app

    }

}

fn main() {
    let application = Application::new(
        "com.vilelapinheiro.jogos",
        ApplicationFlags::empty())
        .expect("Application initialization failed");

    application.connect_startup(|application| {});

    application.connect_activate(|_| {});
    application.run(&env::args().collect::<Vec<_>>());
}
