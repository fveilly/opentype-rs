//#![deny(missing_docs, /*unused_imports*/)]

#[macro_use]
extern crate nom;

#[macro_use]
extern crate bitflags;

mod font;
mod error;
mod otff;
mod table_record;
pub mod traits;
mod types;

#[macro_use]
mod nom_ext;

pub mod parser;
mod tables;

pub use self::otff::OpenTypeFontFile;
pub use self::font::Font;
pub use self::table_record::TableRecord;
pub use self::types::{
    Tag,
    TableTag,
    Rect
};

pub use self::tables::cmap::*;
pub use self::tables::head::*;
pub use self::tables::hhea::*;
pub use self::tables::hmtx::*;
pub use self::tables::maxp::*;
pub use self::tables::name::*;
pub use self::tables::os2::*;
pub use self::tables::post::*;

pub use self::parser::offset_table::OffsetTable;

pub use self::parser::tables::name::{
    Platform,
    UnicodeEncoding,
    MacintoshEncoding,
    MacintoshLanguage,
    IsoEncoding,
    WindowsEncoding,
    WindowsLanguage,
    NameId
};

pub use self::parser::tables::os2::{
    Os2Version, FontSelectionFlags, CodePageRange, UnicodeRange, Panose
};
pub use self::parser::tables::post::PostScriptVersion;