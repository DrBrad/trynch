use gtk4::{Builder};

#[derive(Clone)]
pub struct TopBar {
    pub root: gtk4::Box
}

impl TopBar {

    pub fn new() -> Self {
        let builder = Builder::from_resource("/trynch/rust/res/ui/top_bar.ui");


        let root: gtk4::Box = builder
            .object("root")
            .expect("Couldn't find 'root' in top_bar.ui");

        Self {
            root
        }
    }
}
