use std::cell::RefCell;
use gtk4::{gdk, style_context_add_provider_for_display, ApplicationWindow, Builder, CssProvider, GestureClick, GridView, Label, ListItem, MultiSelection, NoSelection, Orientation, Paned, SignalListItemFactory, SingleSelection, StringObject, Widget, Window};
use gtk4::prelude::{BoxExt, Cast, EventControllerExt, GestureSingleExt, GtkWindowExt, ListItemExt, ListModelExt, SelectionModelExt, StaticType, WidgetExt};
use crate::bus::event_bus::EventPropagation::Continue;
use crate::bus::event_bus::{pause_event, register_event, unregister_event};
use crate::bus::events::key_event::KeyEvent;
use crate::ui::gtk4::views::inter::stackable::Stackable;
use crate::ui::gtk4::views::log_list_item::LogListItem;
use crate::ui::gtk4::views::logs_view::LogsView;
use crate::ui::gtk4::windows::main_window::MainWindow;

pub struct MainView {
    pub root: gtk4::Box,
    key_event_listener: Option<RefCell<u32>>
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

        let activity_pane: Paned = builder
            .object("activity_pane")
            .expect("Couldn't find 'activity_pane' in main_view.ui");

        let logs_view = LogsView::new();
        activity_pane.set_end_child(Some(&logs_view.root));



        let key_event_listener = Some(RefCell::new(register_event("key_event", {
            let logs_list = logs_view.logs_list;

            move |id, event| {
                let event = event.as_any().downcast_ref::<KeyEvent>().unwrap();

                let log = LogListItem::new(event.key.as_str());
                logs_list.append(&log.root);

                Continue
            }
        }, false)));


        Self {
            root,
            key_event_listener
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
        if let Some(key_event_listener) = &self.key_event_listener {
            pause_event("key_event", *key_event_listener.borrow());
        }
    }

    fn on_destroy(&self) {
        if let Some(key_event_listener) = &self.key_event_listener {
            unregister_event("key_event", *key_event_listener.borrow());
        }
    }
}
