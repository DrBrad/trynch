mod ui;

use rusb::{Context, DeviceDescriptor, UsbContext};
use crate::ui::gtk4::app::App;

fn main() {
    let app = App::new();
    app.run();



    /*
    thread::spawn(|| {
        if let Err(err) = listen(|event| {
            match event.event_type {
                EventType::KeyRelease(key) => {
                    /*
                    if key == Key::BackSlash {
                        exit(0);
                    }
                    */
                    println!("{:?}", key);
                    //send_event(Box::new(ButtonEvent::new(key)))
                }
                _ => {}
            }
        }) {
            eprintln!("Error: {:?}", err);
        }
    });

    thread::park();
    */

    /*
    let context = Context::new().unwrap();
    let devices = context.devices().unwrap();

    for device in devices.iter() {
        let desc = device.device_descriptor().unwrap();
        let class = desc.class_code();
        let sub = desc.sub_class_code();
        let proto = desc.protocol_code();


        println!("{} {class} {sub} {proto}", determine_device(desc));
        /.*
        println!("Bus {:03} Device {:03} ID {:04x}:{:04x}",
                 device.bus_number(),
                 device.address(),
                 desc.vendor_id(),
                 desc.product_id());
                 *./
    }
    */
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
