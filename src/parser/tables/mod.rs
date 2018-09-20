//! This module contains all nom parsers required to parse the OpenType font tables.

mod head;
mod hhea;
mod hmtx;
mod maxp;
mod name;
mod os2;

pub use self::head::*;
pub use self::hhea::*;
pub use self::hmtx::*;
pub use self::maxp::*;
pub use self::name::*;
pub use self::os2::*;

use error::Error;
use nom::{IResult, Err, ErrorKind};
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
    Hmtx(hmtx::HorizontalMetricsTable),
    /// Maximum profile
    Maxp(maxp::Maxp),
    /// Naming table
    Name(NamingTable),
    /// OS/2 and Windows specific metrics
    Os2(os2::Os2),
    /// PostScript information
    Post
}

#[doc="
    Parse OpenType font table

    # Example

    ```
    extern crate opentype_rs as otf;

    use otf::parser::tables::{FontTable, parse_table};
    use otf::{TableTag, Rect};

    let bytes: &[u8]  = &[
        0x00, 0x01, 0x00, 0x00, 0x00, 0x02, 0x23, 0x12, 0x8A, 0x7F, 0x70, 0x48, 0x5F, 0x0F,
        0x3C, 0xF5, 0x00, 0x19, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0xC4, 0xF0, 0x11, 0x2E,
        0x00, 0x00, 0x00, 0x00, 0xD5, 0x01, 0x52, 0xF4, 0xFA, 0x1B, 0xFD, 0xD5, 0x09, 0x30,
        0x08, 0x73, 0x00, 0x00, 0x00, 0x09, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00];

    let table = parse_table(bytes, TableTag::Head).unwrap().1;

    match table {
        FontTable::Head(head_table) => {
            assert_eq!(head_table.font_revision(), 140050);
            assert_eq!(head_table.check_sum_adjustment(), 2323607624);
            assert_eq!(head_table.flags(), 25);
            assert_eq!(head_table.units_per_em(), 2048);
            assert_eq!(head_table.created(), 3304067374);
            assert_eq!(head_table.modified(), 3573633780);
            assert_eq!(head_table.bounding_box(), Rect::new(-1509, -555, 2352, 2163));
            assert_eq!(head_table.mac_style(), 0);
            assert_eq!(head_table.lowest_rec_ppem(), 9);
            assert_eq!(head_table.font_direction_hint(), 2);
            assert_eq!(head_table.index_to_loc_format(),  0);
            assert_eq!(head_table.glyph_data_format(), 0);
        },
        _ => assert!(false)
    }
    ```
"]
pub fn parse_table(input: &[u8], table_tag: TableTag)-> IResult<&[u8], FontTable> {
    match table_tag {
        TableTag::Head => map!(input, head::parse_head, |head_table| FontTable::Head(head_table)),
        TableTag::Hhea => map!(input, hhea::parse_hhea, |hhea_table| FontTable::Hhea(hhea_table)),
        TableTag::Maxp => map!(input, maxp::parse_maxp, |maxp_table| FontTable::Maxp(maxp_table)),
        TableTag::Name => map!(input, name::parse_naming_table, |naming_table| FontTable::Name(naming_table)),
        TableTag::Os2 => map!(input, os2::parse_os2, |os2_table| FontTable::Os2(os2_table)),
        _ => Err(Err::Error(error_position!(&input[..], ErrorKind::Switch)))
    }
}
