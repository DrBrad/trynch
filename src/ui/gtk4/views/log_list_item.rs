use gtk4::{Builder, Label, ListBoxRow};

pub struct LogListItem {
    pub root: ListBoxRow,
    pub log_container: gtk4::Box,
    pub log: Label,
    pub time: Label
}

impl LogListItem {

    pub fn new(log: &str) -> Self {
        let builder = Builder::from_resource("/trynch/rust/res/ui/log_list_item.ui");

        let root: ListBoxRow = builder
            .object("root")
            .expect("Couldn't find 'root' in log_list_item.ui");

        let log_container: gtk4::Box = builder
            .object("log_container")
            .expect("Couldn't find 'log_container' in log_list_item.ui");

        let log_view: Label = builder
            .object("log")
            .expect("Couldn't find 'log' in log_list_item.ui");
        log_view.set_label(log);

        let time: Label = builder
            .object("time")
            .expect("Couldn't find 'time' in log_list_item.ui");
        time.set_label("10:05 AM");

        Self {
            root,
            log_container,
            log: log_view,
            time
        }
    }
}
