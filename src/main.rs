mod ui;
mod bus;
mod utils;

use std::thread;
use rdev::{listen};
use rusb::{Context, Hotplug, HotplugBuilder, UsbContext};
use crate::bus::event_bus::send_event;
use crate::ui::gtk4::app::App;
use crate::utils::{camera, keyboard, usb};
use crate::utils::usb::{HotplugHandler};

fn main() {

    keyboard::run();
    usb::run();
    //camera::run();


    let app = App::new();
    app.run();



    //thread::park();
}


