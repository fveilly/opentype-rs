//#![deny(missing_docs, /*unused_imports*/)]

#[macro_use]
extern crate nom;

#[macro_use]
extern crate bitflags;

#[macro_use]
pub mod parser;

mod error;
mod font;
mod offset_table;
mod otff;
mod table;
mod table_record;
mod ttc_header;
pub mod tables;
pub mod types;

pub use self::otff::OpenTypeFontFile;
pub use self::font::Font;
pub use self::table_record::TableRecord;