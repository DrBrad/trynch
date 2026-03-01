mod ui;
mod bus;
mod utils;

use std::thread;
use rdev::{listen};
use rusb::{Context, Hotplug, HotplugBuilder, UsbContext};
use crate::bus::event_bus::send_event;
use crate::bus::events::key_event::KeyEvent;
use crate::ui::gtk4::app::App;
use crate::utils::usb::{HotplugHandler};

fn main() {
    thread::spawn(|| {
        if let Err(err) = listen(|event| {
            if let Some(name) = event.name {
                send_event(Box::new(KeyEvent::new(name)));
            }
        }) {
            eprintln!("Error: {:?}", err);
        }
    });

    thread::spawn(|| {
        let context = Context::new().unwrap();

        HotplugBuilder::new().enumerate(true).register::<Context, _>(&context, Box::new(HotplugHandler)).unwrap();

        loop {
            context.handle_events(None).unwrap();
        }
    });


    let app = App::new();
    app.run();



    //thread::park();
}


