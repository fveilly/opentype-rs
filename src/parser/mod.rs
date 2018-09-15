mod offset_table;
mod table_record;
mod ttc_header;

pub mod tables;

pub use self::offset_table::{OffsetTable, SfntVersion, parse_offset_table};
pub use self::table_record::{TableRecord, parse_table_record, parse_table_records};
pub use self::ttc_header::{TTCHeader, parse_ttc_header};

/// An OpenType font file contains data, in table format, that comprises either a TrueType or a
/// Compact Font Format (CFF) outline font. Rasterizers use combinations of data from the tables
/// contained in the font to render the TrueType or PostScript glyph outlines. Some of this
/// supporting data is used no matter which outline format is used; some of the supporting data is
/// specific to either TrueType or PostScript.
///
/// More information on ['ottf'](https://docs.microsoft.com/en-gb/typography/opentype/spec/otff)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum OpenTypeFontKind {
    Font(OffsetTable),

    FontCollection(TTCHeader)
}

named!(
    #[doc="
        Parse OpenType font file

        # Example

        ```
        extern crate opentype_rs as otf;

        use otf::parser::{OpenTypeFontKind, SfntVersion, parse_otff};
        use otf::Rect;

        let bytes: &[u8]  = &[
            0x00, 0x01, 0x00, 0x00, 0x00, 0x12, 0x01, 0x00, 0x00, 0x04, 0x00, 0x20];

        let kind = parse_otff(bytes).unwrap().1;

        match kind {
            OpenTypeFontKind::Font(offset_table) => {
                assert_eq!(offset_table.sfnt_version(), SfntVersion::TrueType);
                assert_eq!(offset_table.num_tables(), 18);
                assert_eq!(offset_table.search_range(), 256);
                assert_eq!(offset_table.entry_selector(), 4);
                assert_eq!(offset_table.range_shift(), 32);
            },
            _ => assert!(false)
        }
        ```
    "],
    pub parse_otff<&[u8],OpenTypeFontKind>,
    alt!(
        map!(parse_offset_table, |offset_table| OpenTypeFontKind::Font(offset_table)) |
        map!(parse_ttc_header, |ttc_header| OpenTypeFontKind::FontCollection(ttc_header))
    )
);