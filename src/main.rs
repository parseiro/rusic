extern crate gio;
extern crate gtk;
extern crate gdk_pixbuf;
extern crate id3;

use std::env;

use gio::{ApplicationExt, ApplicationExtManual, ApplicationFlags};
use gtk::{
    Adjustment,
    Image,
    ImageExt,
    Scale,
    ScaleExt,
};
use gtk::{Application, ApplicationWindow, GtkWindowExt, WidgetExt};
use gtk::{
    ContainerExt,
    SeparatorToolItem,
    Toolbar,
    ToolButton,
};
use gtk::Orientation::{Horizontal, Vertical};

use crate::toolbar::MusicToolbar;


mod toolbar;

struct App {
    adjustment: Adjustment,
    cover: Image,
    toolbar: MusicToolbar,
    window: ApplicationWindow,
    application: Application,
}

impl App {
    fn new(application: Application) -> Self {
        let window = ApplicationWindow::new(&application);
        window.set_title("Rusic");

        let vbox = gtk::Box::new(Vertical, 0);
        window.add(&vbox);

        let toolbar = MusicToolbar::new();
        vbox.add(toolbar.toolbar());

        let cover = Image::new();
        cover.set_from_file("cover.jpg");
        vbox.add(&cover);

        let adjustment = Adjustment::new(0.0, 0.0, 10.0, 0.0, 0.0, 0.0);
        let scale = Scale::new(Horizontal, &adjustment);
        scale.set_draw_value(false);
        vbox.add(&scale);

        window.show_all();

        let app = App {
            adjustment,
            cover,
            toolbar,
            window,
            application,
        };

        app.connect_events();
        //app.connect_toolbar_events();

        app
    }
}

fn main() {
    let application = Application::new(
        "com.vilelapinheiro.jogos",
        ApplicationFlags::empty())
        .expect("Application initialization failed");

    application.connect_startup(|application| {
        App::new(application.clone());
    });
    application.connect_activate(|_| {});


    application.run(&env::args().collect::<Vec<_>>());
}
