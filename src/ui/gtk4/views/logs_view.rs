use std::cell::RefCell;
use gtk4::{gdk, style_context_add_provider_for_display, Builder, CssProvider, Label, ListBox, ScrolledWindow, Widget};
use gtk4::gio::SimpleAction;
use gtk4::prelude::{ActionMapExt, Cast, WidgetExt};
use crate::bus::event_bus::{pause_event, register_event, resume_event, unregister_event};
use crate::bus::event_bus::EventPropagation::Continue;
use crate::bus::events::log_event::LogEvent;
use crate::ui::gtk4::views::inter::stackable::Stackable;
use crate::ui::gtk4::views::log_image_list_item::LogImageListItem;
use crate::ui::gtk4::views::log_list_item::LogListItem;
use crate::ui::gtk4::windows::main_window::MainWindow;
use crate::utils::detections::Detections;

pub struct LogsView {
    pub root: gtk4::Box,
    pub logs_list: ListBox,
    log_event_listener: Option<RefCell<u32>>
}

impl LogsView {

    pub fn new(window: &MainWindow) -> Self {
        let builder = Builder::from_resource("/trynch/rust/res/ui/logs_view.ui");

        let provider = CssProvider::new();
        provider.load_from_resource("/trynch/rust/res/ui/logs_view.css");
        style_context_add_provider_for_display(&gdk::Display::default().unwrap(), &provider, gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION);

        let root: gtk4::Box = builder
            .object("root")
            .expect("Couldn't find 'root' in logs_view.ui");

        let logs_list: ListBox = builder
            .object("logs_list")
            .expect("Couldn't find 'logs_list' in logs_view.ui");


        let logs_layout: ScrolledWindow = builder
            .object("logs_layout")
            .expect("Couldn't find 'logs_layout' in logs_view.ui");
        //let adj = logs_view.ui.vadjustment();
        //adj.set_value(adj.upper() - adj.page_size());






        let log_event_listener = Some(RefCell::new(register_event("log_event", {
            let logs_list = logs_list.clone();

            move |id, event| {
                let event = event.as_any().downcast_ref::<LogEvent>().unwrap();

                match event.detection {
                    Detections::Keyboard | Detections::Usb => logs_list.append(&LogListItem::new(event.log.as_str(), event.detection, event.severity, event.time).root),
                    Detections::Capture | Detections::Motion => logs_list.append(&LogImageListItem::new(event.log.as_str(), event.detection, event.severity, event.time).root),
                    Detections::Mic => {}
                }

                Continue
            }
        }, true)));


        let action = SimpleAction::new("start", None);
        action.connect_activate({
            let logs_list = logs_list.clone();
            let show_capture_bar = window.show_capture_bar.clone();
            let log_event_listener = log_event_listener.clone();

            move |_, _| {
                show_capture_bar.borrow()(true);
                if let Some(log_event_listener) = &log_event_listener {
                    while let Some(child) = logs_list.first_child() {
                        logs_list.remove(&child);
                    }

                    resume_event("log_event", *log_event_listener.borrow());
                }
            }
        });
        window.window.add_action(&action);

        let action = SimpleAction::new("stop", None);
        action.connect_activate({
            let show_capture_bar = window.show_capture_bar.clone();
            let log_event_listener = log_event_listener.clone();

            move |_, _| {
                show_capture_bar.borrow()(false);
                if let Some(log_event_listener) = &log_event_listener {
                    pause_event("log_event", *log_event_listener.borrow());
                }
            }
        });
        window.window.add_action(&action);

        //let log = LogImageListItem::new("motion_20260301_170134.985.png", Detections::Capture, Severities::Severe);
        //logs_list.append(&log.root);

        Self {
            root,
            logs_list,
            log_event_listener
        }
    }
}

impl Stackable for LogsView {

    fn get_name(&self) -> String {
        String::from("logs_view")
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
        if let Some(log_event_listener) = &self.log_event_listener {
            pause_event("log_event", *log_event_listener.borrow());
        }
    }

    fn on_destroy(&self) {
        if let Some(log_event_listener) = &self.log_event_listener {
            unregister_event("log_event", *log_event_listener.borrow());
        }
    }
}
