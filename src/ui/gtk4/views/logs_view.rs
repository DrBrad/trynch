use std::cell::RefCell;
use gtk4::{gdk, style_context_add_provider_for_display, Builder, CssProvider, Label, ListBox, ScrolledWindow, Widget};
use gtk4::prelude::Cast;
use crate::bus::event_bus::{pause_event, register_event, unregister_event};
use crate::bus::event_bus::EventPropagation::Continue;
use crate::bus::events::camera_event::CameraEvent;
use crate::bus::events::key_event::KeyEvent;
use crate::bus::events::usb_event::UsbEvent;
use crate::ui::gtk4::views::inter::stackable::Stackable;
use crate::ui::gtk4::views::log_image_list_item::LogImageListItem;
use crate::ui::gtk4::views::log_list_item::LogListItem;
use crate::ui::gtk4::windows::main_window::MainWindow;

pub struct LogsView {
    pub root: gtk4::Box,
    pub logs_list: ListBox,
    key_event_listener: Option<RefCell<u32>>,
    usb_event_listener: Option<RefCell<u32>>,
    camera_event_listener: Option<RefCell<u32>>
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

        /*
        let mut logs: Vec<LogListItem> = Vec::new();

        for i in 0..20 {
            let log = LogListItem::new();
            logs_list.append(&log.root);
            logs.push(log);
        }
        */



        let logs_layout: ScrolledWindow = builder
            .object("logs_layout")
            .expect("Couldn't find 'logs_layout' in logs_view.ui");
        //let adj = logs_view.ui.vadjustment();
        //adj.set_value(adj.upper() - adj.page_size());






        let key_event_listener = Some(RefCell::new(register_event("key_event", {
            let logs_list = logs_list.clone();

            move |id, event| {
                let event = event.as_any().downcast_ref::<KeyEvent>().unwrap();

                let log = LogListItem::new(event.key.as_str());
                logs_list.append(&log.root);

                Continue
            }
        }, false)));


        let usb_event_listener = Some(RefCell::new(register_event("usb_event", {
            let logs_list = logs_list.clone();

            move |id, event| {
                let event = event.as_any().downcast_ref::<UsbEvent>().unwrap();

                let log = LogListItem::new(event.name.as_str());
                logs_list.append(&log.root);

                Continue
            }
        }, false)));


        let camera_event_listener = Some(RefCell::new(register_event("camera_event", {
            let logs_list = logs_list.clone();

            move |id, event| {
                let event = event.as_any().downcast_ref::<CameraEvent>().unwrap();

                let log = LogImageListItem::new(event.file.as_str());
                logs_list.append(&log.root);

                Continue
            }
        }, false)));


        let log = LogImageListItem::new("motion_20260301_163902.981.png");
        logs_list.append(&log.root);

        Self {
            root,
            logs_list,
            key_event_listener,
            usb_event_listener,
            camera_event_listener
        }
    }
}

impl Stackable for LogsView {

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

        if let Some(usb_event_listener) = &self.usb_event_listener {
            pause_event("usb_event", *usb_event_listener.borrow());
        }

        if let Some(camera_event_listener) = &self.camera_event_listener {
            pause_event("camera_event", *camera_event_listener.borrow());
        }
    }

    fn on_destroy(&self) {
        if let Some(key_event_listener) = &self.key_event_listener {
            unregister_event("key_event", *key_event_listener.borrow());
        }

        if let Some(usb_event_listener) = &self.usb_event_listener {
            unregister_event("usb_event", *usb_event_listener.borrow());
        }

        if let Some(camera_event_listener) = &self.camera_event_listener {
            unregister_event("camera_event", *camera_event_listener.borrow());
        }
    }
}
