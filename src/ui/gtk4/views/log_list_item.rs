use gtk4::{Builder, Image, Label, ListBoxRow};
use gtk4::prelude::{StyleContextExt, WidgetExt};
use crate::utils::detections::Detections;
use crate::utils::severities::Severities;

pub struct LogListItem {
    pub root: ListBoxRow,
    pub log_container: gtk4::Box,
    pub log: Label,
    pub time: Label
}

impl LogListItem {

    pub fn new(message: &str, detection: Detections, severity: Severities) -> Self {
        let builder = Builder::from_resource("/trynch/rust/res/ui/log_list_item.ui");

        let root: ListBoxRow = builder
            .object("root")
            .expect("Couldn't find 'root' in log_list_item.ui");
        root.style_context().add_class(&severity.to_string());

        let icon: Image = builder
            .object("icon")
            .expect("Couldn't find 'icon' in log_list_item.ui");
        icon.set_resource(Some("/trynch/rust/res/icons/ic_keyboard.svg"));

        match detection {
            Detections::Keyboard => icon.set_resource(Some("/trynch/rust/res/icons/ic_keyboard.svg")),
            Detections::Usb => icon.set_resource(Some("/trynch/rust/res/icons/ic_usb.svg")),
            _ => {}
        }

        let log_container: gtk4::Box = builder
            .object("log_container")
            .expect("Couldn't find 'log_container' in log_list_item.ui");

        let log: Label = builder
            .object("log")
            .expect("Couldn't find 'log' in log_list_item.ui");
        log.set_label(message);

        let time: Label = builder
            .object("time")
            .expect("Couldn't find 'time' in log_list_item.ui");
        time.set_label("10:05 AM");

        Self {
            root,
            log_container,
            log,
            time
        }
    }
}
