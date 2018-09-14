#[macro_use]
extern crate nom;

extern crate byteorder;

pub mod tables;

mod font;
mod error;
mod otff;
mod parser;
mod table;
mod types;

pub use self::otff::OpenTypeFontFile;
pub use self::types::{
    TableTag,
    Rect
};