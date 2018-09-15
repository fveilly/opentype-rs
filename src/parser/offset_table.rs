use nom::{be_u16, be_u32};
use std::fmt;

/// The OpenType font starts with the Offset Table. If the font file contains only one font, the
/// Offset Table will begin at byte 0 of the file. If the font file is an OpenType Font Collection
/// file (see below), the beginning point of the Offset Table for each font is indicated in the
/// TTCHeader.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct OffsetTable {
    sfnt_version: SfntVersion,
    num_tables: u16,
    search_range: u16,
    entry_selector: u16,
    range_shift: u16
}

impl OffsetTable {
    // Font file format type
    pub fn sfnt_version(&self) -> SfntVersion {
        self.sfnt_version
    }

    /// Number of tables
    pub fn num_tables(&self) -> u16 {
        self.num_tables
    }

    /// (Maximum power of 2 <= num_tables) x 16
    pub fn search_range(&self) -> u16 {
        self.search_range
    }

    /// Log2(maximum power of 2 <= num_tables)
    pub fn entry_selector(&self) -> u16 {
        self.entry_selector
    }

    /// num_tables x 16 - search_range
    pub fn range_shift(&self) -> u16 {
        self.range_shift
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum SfntVersion {
    TrueType,
    CFF
}

impl fmt::Display for SfntVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SfntVersion::TrueType =>  write!(f, "TrueType"),
            SfntVersion::CFF => write!(f, "CFF"),
        }
    }
}

named!(
    #[doc="
        Parse Offset Table.

        # Example

        Parse TrueType Offset Table
        ```
        extern crate opentype_rs as otf;

        use otf::parser::{OffsetTable, SfntVersion, parse_offset_table};

        let bytes: &[u8] = &[
            0x00, 0x01, 0x00, 0x00, 0x00, 0x12, 0x01, 0x00, 0x00, 0x04, 0x00, 0x20];

        let offset_table = parse_offset_table(bytes).unwrap().1;

        assert_eq!(offset_table.sfnt_version(), SfntVersion::TrueType);
        assert_eq!(offset_table.num_tables(), 18);
        assert_eq!(offset_table.search_range(), 256);
        assert_eq!(offset_table.entry_selector(), 4);
        assert_eq!(offset_table.range_shift(), 32);
        ```

        Parse CFF Offset Table
        ```
        extern crate opentype_rs as otf;

        use otf::parser::{OffsetTable, SfntVersion, parse_offset_table};

        let bytes: &[u8] = &[
            0x4F, 0x54, 0x54, 0x4F, 0x00, 0x0E, 0x00, 0x80, 0x00, 0x03, 0x00, 0x60
        ];

        let offset_table = parse_offset_table(bytes).unwrap().1;

        assert_eq!(offset_table.sfnt_version(), SfntVersion::CFF);
        assert_eq!(offset_table.num_tables(), 14);
        assert_eq!(offset_table.search_range(), 128);
        assert_eq!(offset_table.entry_selector(), 3);
        assert_eq!(offset_table.range_shift(), 96);
        ```
    "],
    pub parse_offset_table<&[u8],OffsetTable>,
    do_parse!(
        sfnt_version: parse_sfnt_version >>
        num_tables: be_u16 >>
        search_range: be_u16 >>
        entry_selector: be_u16 >>
        range_shift: be_u16 >>
        (
            OffsetTable {
                sfnt_version,
                num_tables,
                search_range,
                entry_selector,
                range_shift
            }
        )
    )
);

named!(parse_sfnt_version<&[u8],SfntVersion>,
    map_opt!(be_u32, |sfnt_version| {
        match sfnt_version {
            0x00010000 => Some(SfntVersion::TrueType),
            0x4F54544F => Some(SfntVersion::CFF),
            _ => None
        }
    })
);

#[cfg(test)]
mod tests {
    use super::*;
    use nom::{Err, ErrorKind, Context, Needed};

    #[test]
    fn case_offset_table_invalid_empty_slice() {
        let bytes: &[u8] = &[];

        let expected = Result::Err(Err::Incomplete(Needed::Size(4)));
        assert_eq!(parse_offset_table(bytes), expected);
    }

    #[test]
    fn case_offset_table_invalid_sfnt_version() {
        let bytes: &[u8]  = &[
            0x00, 0x00, 0x00, 0x00, 0x00, 0x0f, 0x00, 0x80,
            0x00, 0x03, 0x00, 0x70
        ];

        let expected = Result::Err(Err::Error(Context::Code(bytes, ErrorKind::MapOpt)));
        assert_eq!(parse_offset_table(bytes),  expected);
    }
}
