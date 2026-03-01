use std::any::Any;
use crate::bus::events::inter::event::Event;

#[derive(Debug, Clone)]
pub struct CameraEvent {
    prevent_default: bool,
    pub(crate) file: String
}

impl CameraEvent {

    pub fn new(file: String) -> Self {
        Self {
            prevent_default: false,
            file
        }
    }
}

impl Event for CameraEvent {

    fn get_name(&self) -> String {
        String::from("camera_event")
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
