#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

#![warn(anonymous_parameters)]
// not useful anymore #![warn(box_pointers)]
//#![warn(missing_docs)]
#![warn(trivial_casts, trivial_numeric_casts)]
#![warn(unused_results)]
#![warn(unused_qualifications)]
#![warn(variant_size_differences)]
#![warn(clippy::cast_possible_truncation, clippy::cast_possible_wrap,
clippy::cast_precision_loss, clippy::cast_sign_loss, clippy::integer_arithmetic)]
#![warn(clippy::fallible_impl_from)]
#![warn(clippy::filter_map, clippy::filter_map_next)]
#![warn(clippy::if_not_else, clippy::nonminimal_bool, clippy::single_match_else)]
#![warn(clippy::int_plus_one)]
#![warn(clippy::similar_names)]
#![warn(clippy::mutex_integer)]
//#![warn(clippy::print_stdout,clippy::use_debug)]
#![warn(clippy::unwrap_used, clippy::map_unwrap_or)]
//#![warn(clippy::unwrap_in_result)]

extern crate gdk_pixbuf;
extern crate gio;
extern crate gtk;
extern crate gtk_sys;
extern crate id3;

use std::env;
use std::rc::Rc;

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

use crate::playlist::Playlist;
use crate::toolbar::MusicToolbar;

mod playlist;
mod toolbar;

struct App {
    adjustment: Adjustment,
    cover: Image,
    toolbar: MusicToolbar,
    window: ApplicationWindow,
    application: Application,
    playlist: Rc<Playlist>,
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
        // cover.set_from_file("cover.jpg");
        vbox.add(&cover);

        let playlist = Rc::new(Playlist::new());
        vbox.add(playlist.view());

        let adjustment = Adjustment::new(0.0, 0.0, 10.0, 0.0, 0.0, 0.0);
        let scale = Scale::new(Horizontal, &adjustment);
        scale.set_draw_value(false);
        vbox.add(&scale);


        window.show_all();

        let app = App {
            adjustment,
            cover,
            playlist,
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

    let _ = application.connect_startup(|application| {
        let _ = App::new(application.clone());
    });
    let _ = application.connect_activate(|_| {});


    let _ = application.run(&env::args().collect::<Vec<_>>());
}
