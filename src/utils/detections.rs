use std::fmt;
use std::fmt::Formatter;

#[derive(Clone, Copy, Debug)]
pub enum Detections {
    Keyboard,
    Usb,
    Capture,
    Motion,
    Mic
}

impl fmt::Display for Detections {

    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Self::Keyboard => "keyboard",
            Self::Usb => "usb",
            Self::Capture => "capture",
            Self::Motion => "motion",
            Self::Mic => "mic"
        })
    }
}
