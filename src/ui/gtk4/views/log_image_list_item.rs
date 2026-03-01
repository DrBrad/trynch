use std::path::PathBuf;
use gtk4::{Builder, Image, Label, ListBoxRow, Overlay, Picture};
use gtk4::cairo::Path;
use gtk4::gdk::Texture;
use gtk4::gio::File;
use crate::ui::gtk4::widgets::rounded_picture::RoundedPicture;

pub struct LogImageListItem {
    pub root: ListBoxRow,
    pub log_image_container: Overlay,
    pub image: RoundedPicture,
    pub time: Label
}

impl LogImageListItem {

    pub fn new(file: &str) -> Self {
        let builder = Builder::from_resource("/trynch/rust/res/ui/log_image_list_item.ui");

        let root: ListBoxRow = builder
            .object("root")
            .expect("Couldn't find 'root' in log_list_item.ui");

        let log_image_container: Overlay = builder
            .object("log_image_container")
            .expect("Couldn't find 'log_image_container' in log_list_item.ui");

        let image: RoundedPicture = builder
            .object("image")
            .expect("Couldn't find 'image' in log_list_item.ui");
        image.set_from_file(Some(&file));
        //image.set_can_shrink(true);

        /*
        match Texture::from_file(&File::for_path(file)) {
            Ok(tex) => image.set_paintable(Some(&tex)),
            Err(err) => eprintln!("Failed to load JPEG: {err}"),
        }
        */

        let time: Label = builder
            .object("time")
            .expect("Couldn't find 'time' in log_list_item.ui");
        time.set_label("10:05 AM");

        Self {
            root,
            log_image_container,
            image,
            time
        }
    }
}
