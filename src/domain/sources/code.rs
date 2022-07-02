use crate::core::domain::{Uuid, Value};

use std::any::Any;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Coordinate {
    pub start_line: u64,
    pub end_line: u64,
    pub start_char: u64,
    pub end_char: u64,
}

#[derive(Clone, PartialEq)]
pub struct Code {
    pub coordinate: Coordinate,
    pub file_uuid: Uuid,
}

impl Value for Code {
    fn equals(&self, value: &dyn Any) -> bool {
        let code = match value.downcast_ref::<Code>() {
            Some(value) => value,
            None => return false,
        };

        self.coordinate.start_line == code.coordinate.start_line
            && self.coordinate.end_line == code.coordinate.end_line
            && self.coordinate.start_char == code.coordinate.start_char
            && self.coordinate.end_char == code.coordinate.end_char
            && self.coordinate.end_char == code.coordinate.end_char
    }
}
impl Code {
    pub fn new(coordinate: Coordinate, file_uuid: Uuid) -> Code {
        Code {
            coordinate,
            file_uuid,
        }
    }
}
