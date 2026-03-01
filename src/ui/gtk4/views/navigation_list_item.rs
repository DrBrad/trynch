use gtk4::{Builder, Image, Label, ListBoxRow};
use gtk4::prelude::{StyleContextExt, WidgetExt};

#[derive(Clone)]
pub struct NavigationListItem {
    pub root: ListBoxRow,
    pub icon: Image,
    pub title: Label
}

impl NavigationListItem {

    pub fn new(icon: &str, title: &str) -> Self {
        let builder = Builder::from_resource("/trynch/rust/res/ui/navigation_list_item.ui");
        let root: ListBoxRow = builder
            .object("root")
            .expect("Couldn't find 'root' in navigation_list_item.ui");

        let icon_view: Image = builder
            .object("icon")
            .expect("Couldn't find 'icon' in navigation_list_item.ui");

        icon_view.set_resource(Some(icon));

        let title_view: Label = builder
            .object("title")
            .expect("Couldn't find 'title' in navigation_list_item.ui");
        title_view.set_label(title);

        Self {
            root,
            icon: icon_view,
            title: title_view
        }
    }
}
