//! OpenType font file parser
//!
//! https://docs.microsoft.com/en-gb/typography/opentype/spec/otff

use error::ErrorKindExt;
use nom::{be_u16, be_u32};
use types::{Offset32, Tag};
use std::fmt;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum OpenTypeFontKind {
    Font(OffsetTable),

    FontCollection(TTCHeader)
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

named!(pub parse_otff<&[u8],OpenTypeFontKind>,
    switch!(take!(4),
        [0x00, 0x01, 0x00, 0x00] => map!(call!(parse_offset_table, SfntVersion::TrueType),
            |offset_table| OpenTypeFontKind::Font(offset_table)) |
        b"OTTO" => map!(call!(parse_offset_table, SfntVersion::CFF),
            |offset_table| OpenTypeFontKind::Font(offset_table)) |
        b"ttcf" => map!(call!(parse_ttc_header),
            |ttc_header| OpenTypeFontKind::FontCollection(ttc_header))
    )
);

/// The OpenType font starts with the Offset Table. If the font file contains only one font, the
/// Offset Table will begin at byte 0 of the file. If the font file is an OpenType Font Collection
/// file (see below), the beginning point of the Offset Table for each font is indicated in the
/// TTCHeader.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct OffsetTable {
    // Font file format type
    sfnt_version: SfntVersion,

    /// Number of tables
    num_tables: u16,

    /// (Maximum power of 2 <= num_tables) x 16
    search_range: u16,

    /// Log2(maximum power of 2 <= num_tables)
    entry_selector: u16,

    /// num_tables x 16 - search_range
    range_shift: u16
}

impl OffsetTable {
    pub fn sfnt_version(&self) -> SfntVersion {
        self.sfnt_version
    }

    pub fn num_tables(&self) -> u16 {
        self.num_tables
    }
}

fn previous_power_of_two(mut x: u16) -> u16 {
    x = x | (x >> 1);
    x = x | (x >> 2);
    x = x | (x >> 4);
    x = x | (x >> 8);
    return x - (x >> 1);
}

named_args!(pub parse_offset_table(sfnt_version: SfntVersion)<&[u8],OffsetTable>,
    do_parse!(
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

/// The Offset Table is followed immediately by the Table Record entries. Entries in the Table
/// Record must be sorted in ascending order by tag. Offset values in the Table Record are measured
/// from the start of the font file.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct TableRecord {
    /// Table identifier
    table_tag: Tag,

    /// CheckSum for this table
    check_sum: u32,

    /// Offset from beginning of TrueType font file
    offset: Offset32,

    /// Length of this table
    length: u32
}

impl TableRecord {
    pub fn table_tag(&self) -> Tag {
        self.table_tag
    }

    pub fn check_sum(&self) -> u32 {
        self.check_sum
    }

    pub fn offset(&self) -> Offset32 {
        self.offset
    }

    pub fn length(&self) -> u32 {
        self.length
    }
}

named_args!(pub parse_table_records(num_tables: u16)<&[u8],Vec<TableRecord>>,
    count!(parse_table_record, num_tables as usize)
);

named!(pub parse_table_record<&[u8],TableRecord>,
    do_parse!(
        table_tag: take!(4) >>
        check_sum: be_u32 >>
        offset: be_u32 >>
        length: be_u32 >>
        (
        TableRecord{
            table_tag: Tag::new(table_tag),
            check_sum,
            offset,
            length
        })
    )
);

/// The purpose of the TTC Header table is to locate the different Offset Tables within a TTC file.
/// The TTC Header is located at the beginning of the TTC file (offset = 0). It consists of an
/// identification tag, a version number, a count of the number of OpenType fonts in the file, and
/// an array of offsets to each Offset Table.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TTCHeader {
    /// Number of fonts in TTC
    num_fonts: u32,

    /// Array of offsets to the OffsetTable for each font from the beginning of the file
    offset_table: Vec<Offset32>,

    /// There are two versions of the TTC Header: Version 1.0 has been used for TTC files without
    /// digital signatures. Version 2.0 can be used for TTC files with or without digital
    /// signatures.
    dsig: Option<TTCDigitalSignature>
}

impl TTCHeader {
    pub fn num_fonts(&self) -> u32 {
        self.num_fonts
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct TTCDigitalSignature {
    /// Tag indicating that a DSIG table exists, 0x44534947 ('DSIG')
    dsig_tag: u32,

    /// The length (in bytes) of the DSIG table
    dsig_length: u32,

    /// The offset (in bytes) of the DSIG table from the beginning of the TTC
    dsig_offset: u32,
}

named!(parse_ttc_header<&[u8],TTCHeader>,
    alt!(parse_ttc_header_v1 | parse_ttc_header_v2)
);

named!(parse_ttc_header_v1<&[u8],TTCHeader>,
    do_parse!(
        tag!(&[0x00, 0x01]) >>
        tag!(&[0x00, 0x00]) >>
        num_fonts: be_u32 >>
        offset_table: count!(be_u32, num_fonts as usize) >>
        (
            TTCHeader {
                num_fonts,
                offset_table,
                dsig: None
            }
        )
    )
);

named!(parse_ttc_header_v2<&[u8],TTCHeader>,
    do_parse!(
        tag!(&[0x00, 0x02]) >>
        tag!(&[0x00, 0x00]) >>
        num_fonts: be_u32 >>
        offset_table: count!(be_u32, num_fonts as usize) >>
        dsig_tag: be_u32 >>
        dsig_length: be_u32 >>
        dsig_offset: be_u32 >>
        ({
            // If there’s no signature, then the last three fields of the version 2.0 header
            // are left null
            let dsig = if dsig_tag == 0 && dsig_length == 0 && dsig_offset == 0 {
                None
            }
            else {
                Some(TTCDigitalSignature {
                    dsig_tag,
                    dsig_length,
                    dsig_offset
                })
            };

            TTCHeader {
                num_fonts,
                offset_table,
                dsig
            }
        })
    )
);

#[cfg(test)]
mod tests {
    use super::*;
    use nom::{Err, ErrorKind, Context};
    use types::TableTag;

    static NOTO_SANS_CJK_REGULAR: &[u8] = include_bytes!("../fonts/NotoSansCJK/NotoSansCJK-Regular.ttc");
    static ROBOTO_REGULAR: &[u8] = include_bytes!("../fonts/Roboto/Roboto-Regular.ttf");
    static SOURCE_SERIF_PRO_REGULAR: &[u8] = include_bytes!("../fonts/source-serif-pro/SourceSerifPro-Regular.otf");

    #[test]
    fn case_offset_table_typefont() {
        let expected = (&b""[..], OpenTypeFontKind::Font(OffsetTable {
            sfnt_version: SfntVersion::TrueType,
            num_tables: 18,
            search_range: 256,
            entry_selector: 4,
            range_shift: 32
        }));

        let res = parse_otff(&ROBOTO_REGULAR[0..12]).unwrap();
        assert_eq!(res,  expected);
    }

    #[test]
    fn case_offset_table_ccf() {
        let expected = (&b""[..], OpenTypeFontKind::Font(OffsetTable {
            sfnt_version: SfntVersion::CFF,
            num_tables: 14,
            search_range: 128,
            entry_selector: 3,
            range_shift: 96
        }));

        let res = parse_otff(&SOURCE_SERIF_PRO_REGULAR[0..12]).unwrap();
        assert_eq!(res,  expected);
    }

    #[test]
    fn case_offset_table_invalid_sfnt_version() {
        let bytes: &[u8]  = &[
            0x00, 0x00, 0x00, 0x00, 0x00, 0x0f, 0x00, 0x80,
            0x00, 0x03, 0x00, 0x70
        ];

        let expected = Result::Err(Err::Error(Context::Code(bytes, ErrorKind::Switch)));
        assert_eq!(parse_otff(bytes),  expected);
    }

    #[test]
    fn case_table_record() {
        let expected = (&b""[..], TableRecord {
            table_tag: Tag::from(TableTag::Gdef),
            check_sum: 3024269442,
            offset: 141532,
            length: 610
        });

        let res = parse_table_record(&ROBOTO_REGULAR[12..28]).unwrap();
        assert_eq!(res,  expected);
    }

    #[test]
    fn case_ttc_header_v1() {
        let expected = (&b""[..], OpenTypeFontKind::FontCollection(
            TTCHeader {
                num_fonts: 8,
                offset_table: vec![44, 296, 548, 800, 1052, 1304, 1556, 1808],
                dsig: None,
            }
        ));

        let res = parse_otff(&NOTO_SANS_CJK_REGULAR[0..44]).unwrap();
        assert_eq!(res,  expected);
    }

    #[test]
    fn case_ttc_header_v2() {
        // TODO: find a font
    }

    #[test]
    fn case_ttc_header_invalid_version() {
        let bytes: &[u8]  = &[
            0x74, 0x74, 0x63, 0x66, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x1C,
            0x00, 0x00, 0x01, 0x18, 0x00, 0x00, 0x02, 0x14,
            0x00, 0x00, 0x03, 0x10
        ];
        let expected = Result::Err(Err::Error(Context::Code(bytes, ErrorKind::Switch)));

        let res = parse_otff(bytes);
        assert_eq!(res,  expected);
    }
}