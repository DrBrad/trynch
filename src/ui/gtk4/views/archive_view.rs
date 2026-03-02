use gtk4::{gdk, style_context_add_provider_for_display, Builder, CssProvider, Label, ListBox, ScrolledWindow, Widget};
use gtk4::prelude::{ActionMapExt, Cast};
use crate::bus::event_bus::{pause_event, register_event, resume_event, unregister_event};
use crate::ui::gtk4::views::inter::stackable::Stackable;
use crate::ui::gtk4::windows::main_window::MainWindow;

pub struct LogsView {
    pub root: gtk4::Box
}

impl LogsView {

    pub fn new(window: &MainWindow) -> Self {
        let builder = Builder::from_resource("/trynch/rust/res/ui/archive_view.ui");

        let provider = CssProvider::new();
        provider.load_from_resource("/trynch/rust/res/ui/archive_view.css");
        style_context_add_provider_for_display(&gdk::Display::default().unwrap(), &provider, gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION);

        let root: gtk4::Box = builder
            .object("root")
            .expect("Couldn't find 'root' in archive_view.ui");

        Self {
            root
        }
    }
}

impl Stackable for LogsView {

    fn get_name(&self) -> String {
        String::from("archive_view")
    }

    fn get_root(&self) -> &Widget {
        self.root.upcast_ref()
    }

    fn on_create(&self) {
        //(self.show_title_bar)(true);
    }

    fn on_resume(&self) {
        //(self.show_title_bar)(true);
    }

    fn on_pause(&self) {
    }

    fn on_destroy(&self) {
    }
}
