use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};
use rdev::listen;
use crate::bus::event_bus::send_event;
use crate::bus::events::log_event::LogEvent;
use crate::utils::detections::Detections;
use crate::utils::severities::Severities;

pub fn run() {
    thread::spawn(|| {
        if let Err(err) = listen(|event| {
            if let Some(name) = event.name {
                let now = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards");
                send_event(Box::new(LogEvent::new(name, Detections::Keyboard, Severities::Warning, now)));
            }
        }) {
            eprintln!("Error: {:?}", err);
        }
    });
}
