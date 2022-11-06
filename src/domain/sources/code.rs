use crate::core::domain::{Uuid, Value};
use serde::{Deserialize, Serialize};

use std::any::Any;

use super::file::FileUuid;

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Point {
    pub row: usize,
    pub column: usize,
}

impl Value<Point> for Point {
    fn equals(&self, value: &Point) -> bool {
        self == value
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Coordinate {
    pub start: Point,
    pub end: Point,
}

impl Value<Coordinate> for Coordinate {
    fn equals(&self, value: &Coordinate) -> bool {
        self == value
    }
}

#[derive(Default, Clone, PartialEq, Serialize, Deserialize, Debug, Hash, Eq)]
pub struct Code {
    pub coordinate: Coordinate,
    pub file_uuid: FileUuid,
}

impl Value<Code> for Code {
    fn equals(&self, value: &Code) -> bool {
        self == value
    }
}

impl Code {
    pub fn new(coordinate: Coordinate, file_uuid: FileUuid) -> Code {
        Code {
            coordinate,
            file_uuid,
        }
    }
}
