use std::fmt;
use std::fmt::Formatter;

#[derive(Clone, Copy, Debug)]
pub enum Severities {
    Good,
    Warning,
    Severe
}

impl fmt::Display for Severities {

    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Self::Good => "good",
            Self::Warning => "warning",
            Self::Severe => "severe"
        })
    }
}
