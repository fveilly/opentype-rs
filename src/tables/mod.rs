pub mod head;
pub mod hhea;
pub mod maxp;

use error::Error;
use types::{Tag, TableTag};

#[derive(Debug)]
pub enum FontTable {
    /// Required Tables
    /// Whether TrueType or CFF outlines are used in an OpenType font, the following tables are
    /// required for the font to function correctly.

    /// Character to glyph mapping
    Cmap,
    /// Font header
    Head(head::Head),
    /// Horizontal header
    Hhea(hhea::Hhea),
    /// Horizontal metrics
    Hmtx,
    /// Maximum profile
    Maxp(maxp::Maxp),
    /// Naming table
    Name,
    /// OS/2 and Windows specific metrics
    Os2,
    /// PostScript information
    Post
}

pub fn parse_table<'otf>(table_tag: TableTag, data: &'otf[u8]) -> Result<FontTable, Error>
{
    match table_tag {
        TableTag::Head => Ok(FontTable::Head((head::parse_head(data)?.1))),
        TableTag::Hhea => Ok(FontTable::Hhea((hhea::parse_hhea(data)?.1))),
        TableTag::Maxp => Ok(FontTable::Maxp((maxp::parse_maxp(data)?.1))),
        _ => Err(Error::new(format!("Missing parser for table tag {}", table_tag)))
    }
}

