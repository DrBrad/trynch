use std::thread;
use rdev::listen;
use crate::bus::event_bus::send_event;
use crate::bus::events::key_event::KeyEvent;

pub fn run() {
    thread::spawn(|| {
        if let Err(err) = listen(|event| {
            if let Some(name) = event.name {
                send_event(Box::new(KeyEvent::new(name)));
            }
        }) {
            eprintln!("Error: {:?}", err);
        }
    });
}
