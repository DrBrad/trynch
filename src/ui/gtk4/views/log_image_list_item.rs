use gtk4::{Builder, Image, Label, ListBoxRow};
use gtk4::gdk::Texture;
use gtk4::gio::File;

pub struct LogImageListItem {
    pub root: ListBoxRow,
    pub log_container: gtk4::Box,
    pub image: Image,
    pub time: Label
}

impl LogImageListItem {

    pub fn new(file: &str) -> Self {
        let builder = Builder::from_resource("/trynch/rust/res/ui/log_image_list_item.ui");

        let root: ListBoxRow = builder
            .object("root")
            .expect("Couldn't find 'root' in log_list_item.ui");

        let log_container: gtk4::Box = builder
            .object("log_container")
            .expect("Couldn't find 'log_container' in log_list_item.ui");

        let image: Image = builder
            .object("image")
            .expect("Couldn't find 'image' in log_list_item.ui");

        match Texture::from_file(&File::for_path(file)) {
            Ok(tex) => image.set_paintable(Some(&tex)),
            Err(err) => eprintln!("Failed to load JPEG: {err}"),
        }

        let time: Label = builder
            .object("time")
            .expect("Couldn't find 'time' in log_list_item.ui");
        time.set_label("10:05 AM");

        Self {
            root,
            log_container,
            image,
            time
        }
    }
}
