use std::path::PathBuf;

use gtk::{ApplicationWindow, ContainerExt, DialogExt, FileChooserExt, FileFilterExt, Image, ImageExt, SeparatorToolItem, Toolbar, ToolButton, ToolButtonExt, WidgetExt, Continue, AdjustmentExt};
use gtk::{FileChooserAction, FileChooserDialog, FileFilter};
use gtk_sys::{GTK_RESPONSE_ACCEPT, GTK_RESPONSE_CANCEL};

use crate::App;
use crate::playlist::Playlist;

const PLAY_STOCK: &str = "gtk-media-play";
const PAUSE_STOCK: &str = "gtk-media-pause";

pub struct MusicToolbar {
    open_button: ToolButton,
    next_button: ToolButton,
    play_button: ToolButton,
    previous_button: ToolButton,
    quit_button: ToolButton,
    remove_button: ToolButton,
    stop_button: ToolButton,
    toolbar: Toolbar,
}

impl MusicToolbar {
    pub(crate) fn new() -> Self {
        let toolbar = Toolbar::new();
        let open_button = ToolButton::new_from_stock("gtk-open");
        toolbar.add(&open_button);
        toolbar.add(&SeparatorToolItem::new());

        let previous_button = ToolButton::new_from_stock("gtk-media-previous");
        toolbar.add(&previous_button);

        let play_button = ToolButton::new_from_stock(PLAY_STOCK);
        toolbar.add(&play_button);

        let stop_button = ToolButton::new_from_stock("gtk-media-stop");
        toolbar.add(&stop_button);

        let next_button = ToolButton::new_from_stock("gtk-media-next");
        toolbar.add(&next_button);

        toolbar.add(&SeparatorToolItem::new());

        let remove_button = ToolButton::new_from_stock("gtk-remove");
        toolbar.add(&remove_button);

        toolbar.add(&SeparatorToolItem::new());

        let quit_button = ToolButton::new_from_stock("gtk-quit");
        toolbar.add(&quit_button);

        MusicToolbar {
            open_button,
            next_button,
            play_button,
            previous_button,
            quit_button,
            remove_button,
            stop_button,
            toolbar,
        }
    }

    pub(crate) fn toolbar(&self) -> &Toolbar {
        &self.toolbar
    }
}

impl App {
    pub fn connect_events(&self) {
        let playlist = self.playlist.clone();
        let adjustment = self.adjustment.clone();
        let state = self.state.clone();
        // let play_image = self.toolbar.play_image.clone();

        gtk::timeout_add(100, move || {
            let state = state.lock().unwrap();
            if let Some(path) = playlist.path() {
                if let Some(&duration) = state.durations.get(&path)
                {
                    adjustment.set_upper(duration as f64);
                }
            }
            if state.stopped {
                // set_image_icon(&play_image, PLAY_ICON);
            } else {
                // set_image_icon(&play_image, PAUSE_ICON);
            }
            adjustment.set_value(state.current_time as f64);
            Continue(true)
        });
    }

    pub(crate) fn connect_toolbar_events(&self) {
        let window = self.window.clone();

        let _ = self.toolbar.quit_button.connect_clicked(move |_| {
            window.destroy();
        });

        let play_button = self.toolbar.play_button.clone();
        let playlist = self.playlist.clone();
        let cover = self.cover.clone();
        let _ = self.toolbar.play_button.connect_clicked(move |_| {
            if play_button.get_stock_id() == Some(PLAY_STOCK.to_string()) {
                play_button.set_stock_id(PAUSE_STOCK);
                set_cover(&cover, &playlist);
            } else {
                play_button.set_stock_id(PLAY_STOCK);
            }
        });

        let parent = self.window.clone();
        let playlist = self.playlist.clone();
        let _ = self.toolbar.open_button.connect_clicked(move |_| {
            let file = show_open_dialog(&parent);
            if let Some(file) = file {
                playlist.add(&file);
            }
        });

        let playlist = self.playlist.clone();
        let _ = self.toolbar.remove_button.connect_clicked(move |_| {
            playlist.remove_selection();
        });

        let playlist = self.playlist.clone();
        // let play_image = self.toolbar.play_image.clone();
        let cover = self.cover.clone();
        let state = self.state.clone();
        self.toolbar.play_button.connect_clicked(move |_| {
            if state.lock().unwrap().stopped {
                if playlist.play() {
                    // set_image_icon(&play_image, PAUSE_ICON);
                    set_cover(&cover, &playlist);
                }
            } else {
                playlist.pause();
                // set_image_icon(&play_image, PLAY_ICON);
            }
        });

        let playlist = self.playlist.clone();
        // let play_image = self.toolbar.play_image.clone();
        let cover = self.cover.clone();
        self.toolbar.stop_button.connect_clicked(move |_| {
            playlist.stop();
            cover.hide();
            // set_image_icon(&play_image, PLAY_ICON);
        });

        let playlist = self.playlist.clone();
        // let play_image = self.toolbar.play_image.clone();
        let cover = self.cover.clone();
        self.toolbar.next_button.connect_clicked(move |_| {
            if playlist.next() {
                // set_image_icon(&play_image, PAUSE_ICON);
                set_cover(&cover, &playlist);
            }
        });

        let playlist = self.playlist.clone();
        // let play_image = self.toolbar.play_image.clone();
        let cover = self.cover.clone();
        self.toolbar.previous_button.connect_clicked(move |_| {
            if playlist.previous() {
                // set_image_icon(&play_image, PAUSE_ICON);
                set_cover(&cover, &playlist);
            }
        });
    }
}


const RESPONSE_ACCEPT: i32 = GTK_RESPONSE_ACCEPT;
const RESPONSE_CANCEL: i32 = GTK_RESPONSE_CANCEL;

fn show_open_dialog(parent: &ApplicationWindow)
                    -> Option<PathBuf> {
    let mut file = None;
    let dialog = FileChooserDialog::new(Some("Select an MP3 audio
     file"),
                                        Some(parent), FileChooserAction::Open);
    {
        let filter = FileFilter::new();
        filter.add_mime_type("audio/mp3");
        filter.set_name("MP3 audio file");
        dialog.add_filter(&filter);
    }

    let _ = dialog.add_button("Cancel", RESPONSE_CANCEL);
    let _ = dialog.add_button("Accept", RESPONSE_ACCEPT);

    let result = dialog.run();
    if result == RESPONSE_ACCEPT {
        file = dialog.get_filename();
    }
    dialog.destroy();
    file
}

fn set_cover(cover: &Image, playlist: &Playlist) {
    cover.set_from_pixbuf(playlist.pixbuf().as_ref());
    cover.show();
}