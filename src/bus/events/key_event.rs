use std::any::Any;
use crate::bus::events::inter::event::Event;

#[derive(Debug, Clone)]
pub struct KeyEvent {
    prevent_default: bool,
    pub(crate) key: String
}

impl KeyEvent {

    pub fn new(key: String) -> Self {
        Self {
            prevent_default: false,
            key
        }
    }
}

impl Event for KeyEvent {

    fn get_name(&self) -> String {
        String::from("key_event")
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
