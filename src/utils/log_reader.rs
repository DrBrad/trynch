use std::fmt;
use std::fmt::Formatter;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

pub struct LogReader {
    reader: BufReader<File>
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LogReaderError {
    _type: ErrorKind,
    message: String
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ErrorKind {
    PathNotFound,
    TypeNotFound,
    Parsing,
    WrongClass,
    Format,
    ExtraRRData,
    UnexpectedEof
}

impl LogReaderError {

    pub fn new(_type: ErrorKind, message: &str) -> Self {
        Self {
            _type,
            message: message.to_string()
        }
    }
}

impl fmt::Display for LogReaderError {

    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}: {}", self._type, self.message)
    }
}

impl LogReader {

    pub fn open<P: Into<PathBuf>>(file_path: P) -> Result<Self, LogReaderError> {
        let file = File::open(file_path.into()).map_err(|e| LogReaderError::new(ErrorKind::PathNotFound, &e.to_string()))?;
        let reader = BufReader::new(file);

        Ok(Self {
            reader
        })
    }

    pub fn read_log(&mut self) -> Result<Option<(String)>, LogReaderError> {
        let mut line = String::new();

        let timestamp = line[..15].to_string();

        match self.reader.read_line(&mut line) {
            Ok(length) => {
                println!("{}  {}", line, length);

            }
            Err(e) => return Err(LogReaderError::new(ErrorKind::UnexpectedEof, &e.to_string()))
        }

        Ok(Some("".to_string()))
    }

    pub fn logs(&mut self) -> LogReaderIter {
        LogReaderIter {
            reader: self
        }
    }
}

pub struct LogReaderIter<'a> {
    reader: &'a mut LogReader
}

impl<'a> Iterator for LogReaderIter<'a> {

    type Item = Result<(String), LogReaderError>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.reader.read_log() {
            Ok(Some(rec)) => Some(Ok(rec)),
            Ok(None) => None,
            Err(e) => Some(Err(e))
        }
    }
}
