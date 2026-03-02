use gtk4::{Builder, Image, Label, ListBoxRow, Overlay};
use gtk4::prelude::{StyleContextExt, WidgetExt};
use crate::ui::gtk4::widgets::rounded_picture::RoundedPicture;
use crate::utils::detections::Detections;
use crate::utils::severities::Severities;

pub struct LogImageListItem {
    pub root: ListBoxRow,
    pub log_image_container: Overlay,
    pub image: RoundedPicture,
    pub time: Label
}

impl LogImageListItem {

    pub fn new(file: &str, detection: Detections, severity: Severities) -> Self {
        let builder = Builder::from_resource("/trynch/rust/res/ui/log_image_list_item.ui");

        let root: ListBoxRow = builder
            .object("root")
            .expect("Couldn't find 'root' in log_list_item.ui");
        root.style_context().add_class(&severity.to_string());

        let icon: Image = builder
            .object("icon")
            .expect("Couldn't find 'icon' in log_list_item.ui");

        match detection {
            Detections::Capture => icon.set_resource(Some("/trynch/rust/res/icons/ic_capture.svg")),
            Detections::Motion => icon.set_resource(Some("/trynch/rust/res/icons/ic_motion.svg")),
            _ => {}
        }

        let log_image_container: Overlay = builder
            .object("log_image_container")
            .expect("Couldn't find 'log_image_container' in log_list_item.ui");

        let image: RoundedPicture = builder
            .object("image")
            .expect("Couldn't find 'image' in log_list_item.ui");
        image.set_from_file(Some(&file));

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
