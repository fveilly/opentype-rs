//#![deny(missing_docs, /*unused_imports*/)]

#[macro_use]
extern crate nom;

#[macro_use]
extern crate bitflags;

extern crate byteorder;

mod font;
mod error;
mod otff;
mod table;
mod types;

pub mod parser;

pub use self::otff::OpenTypeFontFile;
pub use self::types::{
    Tag,
    TableTag,
    Rect
};