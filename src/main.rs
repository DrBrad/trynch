mod ui;
mod bus;
mod utils;

use std::thread;
use rdev::{listen, EventType};
use rusb::{Context, DeviceDescriptor, UsbContext};
use crate::bus::event_bus::send_event;
use crate::bus::events::key_event::KeyEvent;
use crate::ui::gtk4::app::App;

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

    /*
    let context = Context::new().unwrap();
    let devices = context.devices().unwrap();

    for device in devices.iter() {
        let desc = device.device_descriptor().unwrap();
        let class = desc.class_code();
        let sub = desc.sub_class_code();
        let proto = desc.protocol_code();


        //println!("{} {class} {sub} {proto}", determine_device(desc));
        println!("Bus {:03} Device {:03} ID {:04x}:{:04x}",
                 device.bus_number(),
                 device.address(),
                 desc.vendor_id(),
                 desc.product_id());
    }
    */



    let app = App::new();
    app.run();



    //thread::park();
}

fn determine_device(desc: DeviceDescriptor) -> String {
    let class = desc.class_code();
    let sub = desc.sub_class_code();
    let proto = desc.protocol_code();

    // HID class (0x03), Boot subclass (0x01), Protocol 0x01=Keyboard, 0x02=Mouse
    if class == 0x03 && sub == 0x01 && proto == 0x01 {
        return String::from("Keyboard");
    }
    if class == 0x03 && sub == 0x01 && proto == 0x02 {
        return String::from("Mouse");
    }

    String::from("Unknown")
}
