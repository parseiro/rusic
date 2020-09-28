extern crate gio;
extern crate gtk;

use std::env;

use gio::{ApplicationExt, ApplicationExtManual, ApplicationFlags};
use gtk::{
    Application,
    ApplicationWindow,
    WidgetExt,
    GtkWindowExt,
};

fn main() {
    let application = Application::new(
        "com.vilelapinheiro.jogos",
        ApplicationFlags::empty())
        .expect("Application initialization failed");

    application.connect_startup(|app| {
        let window = ApplicationWindow::new(&app);
        window.set_title("Rusic");
        window.show();
    });

    application.connect_activate(|_| {});
    application.run(&env::args().collect::<Vec<_>>());
}
