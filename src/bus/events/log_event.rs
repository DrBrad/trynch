use std::any::Any;
use std::time::Duration;
use crate::bus::events::inter::event::Event;
use crate::utils::detections::Detections;
use crate::utils::severities::Severities;

#[derive(Debug, Clone)]
pub struct LogEvent {
    prevent_default: bool,
    pub(crate) log: String,
    pub(crate) detection: Detections,
    pub(crate) severity: Severities,
    pub(crate) time: Duration
}

impl LogEvent {

    pub fn new(log: String, detection: Detections, severity: Severities, time: Duration) -> Self {
        Self {
            prevent_default: false,
            log,
            detection,
            severity,
            time
        }
    }
}

impl Event for LogEvent {

    fn get_name(&self) -> String {
        String::from("log_event")
    }

    fn is_prevent_default(&self) -> bool {
        todo!()
    }

    fn prevent_default(&mut self) {
        todo!()
    }

    fn upcast(&self) -> &dyn Event {
        self
    }

    fn upcast_mut(&mut self) -> &mut dyn Event {
        self
    }

    fn dyn_clone(&self) -> Box<dyn Event> {
        Box::new(self.clone())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
