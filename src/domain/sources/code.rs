use crate::core::domain::{Uuid, Value};
use serde::{Deserialize, Serialize};

use std::any::Any;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Point {
    pub row: usize,
    pub column: usize,
}

// TODO: replace equals with PartialEq implem
impl Value for Point {
    fn equals(&self, value: &dyn Any) -> bool {
        let point = match value.downcast_ref::<Point>() {
            Some(point) => point,
            None => return false,
        };

        self == point
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Coordinate {
    pub start: Point,
    pub end: Point,
}

impl Value for Coordinate {
    fn equals(&self, value: &dyn Any) -> bool {
        let coordinate = match value.downcast_ref::<Coordinate>() {
            Some(coordinate) => coordinate,
            None => return false,
        };

        self == coordinate
    }
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug, Hash, Eq)]
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

        self == code
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
