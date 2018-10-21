//! This module contains all nom parsers required to parse the OpenType font tables.

pub mod cmap;
pub mod head;
pub mod hhea;
pub mod hmtx;
pub mod maxp;
pub mod name;
pub mod os2;
pub mod post;

pub use self::cmap::*;
pub use self::head::*;
pub use self::hhea::*;
pub use self::hmtx::*;
pub use self::maxp::*;
pub use self::name::*;
pub use self::os2::*;
pub use self::post::*;

/// A glyph identifier.
pub type GlyphId = u16;
