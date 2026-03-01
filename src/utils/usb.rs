use std::thread;
use std::time::Duration;
use rusb::{Context, Device, HotplugBuilder, UsbContext};
use crate::bus::event_bus::send_event;
use crate::bus::events::usb_event::UsbEvent;

pub struct HotplugHandler;

impl<T: UsbContext> rusb::Hotplug<T> for HotplugHandler {

    fn device_arrived(&mut self, device: Device<T>) {
        send_event(Box::new(UsbEvent::new(format!("[+] {}", device_pretty_name(&device)))));
    }

    fn device_left(&mut self, device: Device<T>) {
        send_event(Box::new(UsbEvent::new(format!("[-] {}", device_pretty_name(&device)))));
    }
}

pub fn run() {
    thread::spawn(|| {
        let context = Context::new().unwrap();

        HotplugBuilder::new().enumerate(true).register::<Context, _>(&context, Box::new(HotplugHandler)).unwrap();

        loop {
            context.handle_events(None).unwrap();
        }
    });
}

fn device_pretty_name<T: UsbContext>(device: &Device<T>) -> String {
    match device.device_descriptor() {
        Ok(desc) => {
            //let class = desc.class_code();
            //let sub = desc.sub_class_code();
            //let proto = desc.protocol_code();

            let mut manufacturer = None;
            let mut product = None;
            let mut serial = None;

            if let Ok(mut handle) = device.open() {
                let _ = handle.set_auto_detach_kernel_driver(true);

                if let Ok(langs) = handle.read_languages(Duration::from_millis(200)) {
                    if let Some(lang) = langs.get(0).copied() {
                        manufacturer = handle
                            .read_manufacturer_string(lang, &desc, Duration::from_millis(200))
                            .ok();

                        product = handle
                            .read_product_string(lang, &desc, Duration::from_millis(200))
                            .ok();

                        serial = handle
                            .read_serial_number_string(lang, &desc, Duration::from_millis(200))
                            .ok();
                    }
                }
            }

            let pretty_name = match (manufacturer.as_deref(), product.as_deref()) {
                (Some(m), Some(p)) if !m.is_empty() && !p.is_empty() => format!("{m} {p}"),
                (Some(m), _) if !m.is_empty() => m.to_string(),
                (_, Some(p)) if !p.is_empty() => p.to_string(),
                _ => "<unknown (no string descriptor / permission)>".to_string(),
            };

            /*
            println!(
                "Bus {:03} Device {:03} ID {:04x}:{:04x}  class/sub/proto={}/{}/{}  name={}{}",
                device.bus_number(),
                device.address(),
                desc.vendor_id(),
                desc.product_id(),
                class,
                sub,
                proto,
                pretty_name,
                serial
                    .as_deref()
                    .map(|s| format!("  serial={s}"))
                    .unwrap_or_default(),
            );
            */

            pretty_name
        },
        Err(_) => String::from("<unknown (no string descriptor / permission)>")
    }
}
