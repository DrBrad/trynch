use gtk4::gio::SimpleAction;
use gtk4::prelude::{ActionMapExt, Cast, GtkWindowExt};
use gtk4::{AboutDialog, Window};
use gtk4::gdk::Texture;
use crate::ui::gtk4::windows::main_window::MainWindow;

pub fn register_window_actions(window: &MainWindow) {
    let action = SimpleAction::new("show-about-dialog", None);
    action.connect_activate({
        let window = window.window.clone();
        move |_, _| {
            open_about_dialog(&window.upcast_ref());
        }
    });
    window.window.add_action(&action);
}

fn open_about_dialog(window: &Window) {
    //let icon_paintable = Texture::from_resource("/trynch/rust/res/icons/ic_launcher.svg");

    let dialog = AboutDialog::builder()
        .transient_for(window)
        .modal(true)
        .program_name("Trynch")
        .version(format!("{}-{}-{}", env!("PROFILE"), env!("CARGO_PKG_VERSION"), "gtk4").as_str())
        .authors(vec!["DrBrad"])
        //.website_label("https://ethernaught.net")
        //.website("https://ethernaught.net")
        .comments("")
        .copyright("Copyright (c) 2026 Trynch")
        .license("Copyright (c) 2026 Trynch")
        //.logo(&icon_paintable)
        .build();

    dialog.present();
}
