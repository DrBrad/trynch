use std::rc::Rc;
use gtk4::{gdk, style_context_add_provider_for_display, ApplicationWindow, Builder, CssProvider, GestureClick, GridView, Label, ListItem, MultiSelection, NoSelection, Orientation, SignalListItemFactory, SingleSelection, StringObject, Widget, Window};
use gtk4::gio::ListStore;
use gtk4::prelude::{BoxExt, Cast, EventControllerExt, GestureSingleExt, GtkWindowExt, ListItemExt, ListModelExt, SelectionModelExt, StaticType, WidgetExt};

use std::cell::{Cell, RefCell};
use std::collections::{HashMap, VecDeque};
use std::fmt::format;
use std::process::exit;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use rdev::{listen, EventType, Key};
use crate::ui::gtk4::views::inter::stackable::Stackable;
use crate::ui::gtk4::windows::main_window::MainWindow;

pub struct MainView {
    pub root: gtk4::Box
}

impl MainView {

    pub fn new(window: &MainWindow) -> Self {
        let builder = Builder::from_resource("/trynch/rust/res/ui/main_view.ui");

        let provider = CssProvider::new();
        provider.load_from_resource("/trynch/rust/res/ui/main_view.css");
        style_context_add_provider_for_display(&gdk::Display::default().unwrap(), &provider, gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION);

        let root: gtk4::Box = builder
            .object("root")
            .expect("Couldn't find 'root' in main_view.ui");

        Self {
            root
        }
    }
}

impl Stackable for MainView {

    fn get_name(&self) -> String {
        String::from("main_view")
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
        //if let Some(button_event_listener) = &self.button_event_listener {
        //    pause_event("button_event", *button_event_listener.borrow());
        //}
    }

    fn on_destroy(&self) {
        //if let Some(button_event_listener) = &self.button_event_listener {
        //    unregister_event("button_event", *button_event_listener.borrow());
        //}
    }
}
