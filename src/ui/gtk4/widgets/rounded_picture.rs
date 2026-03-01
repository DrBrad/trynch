use std::cell::{Cell, RefCell};
use std::path::Path;

use gtk4::{glib, gsk, Buildable, Orientation, Snapshot, Widget};
use gtk4::gdk::Texture;
use gtk4::gio::File;
use gtk4::graphene::Rect;
use gtk4::prelude::{ObjectExt, SnapshotExt, WidgetExt};
use gtk4::subclass::prelude::{ObjectImpl, ObjectSubclass, ObjectSubclassIsExt, WidgetClassExt, WidgetImpl};

const MIN_WIDTH: i32 = 20;
const MIN_HEIGHT: i32 = 20;

mod imp {
    use gtk4::subclass::prelude::{DerivedObjectProperties, ObjectSubclassExt};
    use super::*;

    #[derive(Default, glib::Properties)]
    #[properties(wrapper_type = super::RoundedPicture)]
    pub struct RoundedPictureImpl {
        pub texture: RefCell<Option<Texture>>,
        #[property(name = "radius", get, set = Self::set_radius, type = f32, default = 0.0)]
        pub radius: Cell<f32>
    }

    impl RoundedPictureImpl {

        fn set_radius(&self, value: f32) {
            self.radius.set(value.max(0.0));
            self.obj().queue_draw();
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for RoundedPictureImpl {

        const NAME: &'static str = "RoundedPicture";
        type Type = RoundedPicture;
        type ParentType = Widget;

        fn class_init(class: &mut Self::Class) {
            class.set_css_name("rounded-picture");
        }
    }

    impl ObjectImpl for RoundedPictureImpl {

        fn properties() -> &'static [glib::ParamSpec] {
            Self::derived_properties()
        }

        fn set_property(&self, id: usize, value: &glib::Value, pspec: &glib::ParamSpec) {
            self.derived_set_property(id, value, pspec)
        }

        fn property(&self, id: usize, pspec: &glib::ParamSpec) -> glib::Value {
            self.derived_property(id, pspec)
        }
    }

    impl WidgetImpl for RoundedPictureImpl {

        fn snapshot(&self, snapshot: &Snapshot) {
            let widget = self.obj();
            let width = widget.width() as f32;
            let height = widget.height() as f32;

            if width <= 0.0 || height <= 0.0 {
                return;
            }

            let mut radius = self.radius.get();

            let max_r = 0.5 * width.min(height);
            radius = radius.min(max_r);

            if radius > 0.0 {
                snapshot.push_rounded_clip(&gsk::RoundedRect::from_rect(
                    Rect::new(0.0, 0.0, width, height),
                    radius,
                ));
            }

            if let Some(texture) = self.texture.borrow().as_ref() {
                snapshot.append_texture(texture, &Rect::new(0.0, 0.0, width, height));
            }

            if radius > 0.0 {
                snapshot.pop();
            }
        }

        fn measure(&self, orientation: Orientation, _for_size: i32) -> (i32, i32, i32, i32) {
            match orientation {
                Orientation::Horizontal => (MIN_WIDTH, MIN_WIDTH, -1, -1),
                Orientation::Vertical => (MIN_HEIGHT, MIN_HEIGHT, -1, -1),
                _ => unreachable!(),
            }
        }
    }
}

glib::wrapper! {
    pub struct RoundedPicture(ObjectSubclass<imp::RoundedPictureImpl>)
        @extends Widget,
        @implements Buildable;
}

impl RoundedPicture {

    pub fn new() -> Self {
        glib::Object::builder::<RoundedPicture>().build()
    }

    pub fn set_from_file(&self, path: Option<&str>) {
        let Some(path) = path else {
            *self.imp().texture.borrow_mut() = None;
            self.queue_draw();
            return;
        };

        let file = File::for_path(Path::new(path));
        match Texture::from_file(&file) {
            Ok(texture) => {
                *self.imp().texture.borrow_mut() = Some(texture);
                self.queue_draw();
            }
            Err(_) => {
                *self.imp().texture.borrow_mut() = None;
                self.queue_draw();
            }
        }
    }
}
