use std::cell::RefCell;
use gtk4::{gdk, style_context_add_provider_for_display, Builder, CssProvider, Label, ListBox, ScrolledWindow};
use crate::bus::event_bus::EventPropagation::Continue;
use crate::bus::event_bus::register_event;
use crate::bus::events::key_event::KeyEvent;
use crate::ui::gtk4::views::log_list_item::LogListItem;

pub struct LogsView {
    pub root: gtk4::Box,
    pub logs_list: ListBox
}

impl LogsView {

    pub fn new() -> Self {
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

        Self {
            root,
            logs_list
        }
    }
}
