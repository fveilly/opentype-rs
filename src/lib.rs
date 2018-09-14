#[macro_use]
extern crate nom;

extern crate byteorder;

#[macro_use]
mod macros;

mod font;
mod error;
mod otff;
mod parser;
mod table;
mod tables;
mod types;

pub use self::otff::OpenTypeFontFile;
pub use self::types::TableTag;