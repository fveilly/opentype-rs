//! The following data types are used in the OpenType font file. All OpenType fonts use
//! Motorola-style byte ordering (Big Endian).
//!
//! https://docs.microsoft.com/en-gb/typography/opentype/spec/otff

use std::{fmt, str};
use tables::TableTag;

/// Short offset to a table, same as uint16, NULL offset = 0x0000
pub type Offset16 = u16;

/// Long offset to a table, same as uint32, NULL offset = 0x00000000
pub type Offset32 = u32;

pub type LongDateTime = i64;

pub type Fixed = i32;

/// A rectangular bounding box defined by two points (x_min, y_min) and (x_max, y_max).
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Rect<T> {
    x_min: T,
    y_min: T,
    x_max: T,
    y_max: T,
}

impl<T> Rect<T> where T: Copy {
    pub fn new(x_min: T, y_min: T, x_max: T, y_max: T) -> Rect<T> {
        Rect {
            x_min,
            y_min,
            x_max,
            y_max
        }
    }

    pub fn x_min(&self) -> T {
        self.x_min
    }

    pub fn y_min(&self) -> T {
        self.y_min
    }

    pub fn x_max(&self) -> T {
        self.x_max
    }

    pub fn y_max(&self) -> T {
        self.y_max
    }
}