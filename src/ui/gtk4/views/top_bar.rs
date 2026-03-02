use gtk4::{Builder, Button};

#[derive(Clone)]
pub struct TopBar {
    pub root: gtk4::Box,
    pub app_options: gtk4::Box,
    pub start: Button,
    pub stop: Button
}

impl TopBar {

    pub fn new() -> Self {
        let builder = Builder::from_resource("/trynch/rust/res/ui/top_bar.ui");

        let root: gtk4::Box = builder
            .object("root")
            .expect("Couldn't find 'root' in top_bar.ui");

        let app_options: gtk4::Box = builder
            .object("app_options")
            .expect("Couldn't find 'app_options' in title_bar.ui");

        let start: Button = builder
            .object("start")
            .expect("Couldn't find 'start' in title_bar.ui");

        let stop: Button = builder
            .object("stop")
            .expect("Couldn't find 'stop' in title_bar.ui");

        Self {
            root,
            app_options,
            start,
            stop
        }
    }
}
