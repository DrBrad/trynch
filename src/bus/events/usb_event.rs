use std::any::Any;
use crate::bus::events::inter::event::Event;

#[derive(Debug, Clone)]
pub struct UsbEvent {
    prevent_default: bool,
    pub(crate) name: String
}

impl UsbEvent {

    pub fn new(name: String) -> Self {
        Self {
            prevent_default: false,
            name
        }
    }
}

impl Event for UsbEvent {

    fn get_name(&self) -> String {
        String::from("usb_event")
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
