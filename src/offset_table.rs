use std::fmt;
use nom::IResult;
use nom::Err as NomErr;
use nom::error::ErrorKind;
use nom::number::complete::{be_u16, be_u32};

/// The OpenType font starts with the Offset Table. If the font file contains only one font, the
/// Offset Table will begin at byte 0 of the file. If the font file is an OpenType Font Collection
/// file, the beginning point of the Offset Table for each font is indicated in the
/// [TTCHeader](TTCHeader.t.html).
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct OffsetTable {
    sfnt_version: SfntVersion,
    num_tables: u16,
    search_range: u16,
    entry_selector: u16,
    range_shift: u16,
}

impl OffsetTable {
    /// Font file format type
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

/// SFNT version (see [SFNT](https://en.wikipedia.org/wiki/SFNT))
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum SfntVersion {
    /// TrueType is an outline font standard developed by Apple and Microsoft in the late 1980s as
    /// a competitor to Adobe's Type 1 fonts used in PostScript. It has become the most common
    /// format for fonts on the classic Mac OS, macOS, and Microsoft Windows operating systems (source:
    /// [TrueType](https://en.wikipedia.org/wiki/TrueType)).
    TrueType,

    /// Compact Font Format (also known as CFF font format, Type 2 font format, or CFF/Type 2 font
    /// format) is a lossless compaction of the Type 1 format using Type 2 charstrings. It is
    /// designed to use less storage space than Type 1 fonts, by using operators with multiple
    /// arguments, various predefined default values, more efficient allotment of encoding values
    /// and shared subroutines within a FontSet (family of fonts) (source:
    /// [CFF](https://en.wikipedia.org/wiki/PostScript_fonts#Compact_Font_Format)).
    CFF,
}

impl fmt::Display for SfntVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SfntVersion::TrueType => write!(f, "TrueType"),
            SfntVersion::CFF => write!(f, "CFF"),
        }
    }
}

pub fn parse_offset_table(input: &[u8]) -> IResult<&[u8], OffsetTable>
{
    let (input, sfnt_version) = parse_sfnt_version(input)?;
    let (input, num_tables) = be_u16(input)?;
    let (input, search_range) = be_u16(input)?;
    let (input, entry_selector) = be_u16(input)?;
    let (input, range_shift) = be_u16(input)?;

    Ok((input, OffsetTable {
        sfnt_version,
        num_tables,
        search_range,
        entry_selector,
        range_shift
    }))
}

fn parse_sfnt_version(input: &[u8]) -> IResult<&[u8], SfntVersion>
{
    let (input, sfnt_version) = be_u32(input)?;
    match sfnt_version {
        0x00010000 | 0x74727565 /* true */ | 0x74797031 /* typ1 */ => Ok((input, SfntVersion::TrueType)),
        0x4F54544F /* OTTO */ => Ok((input, SfntVersion::CFF)),
        _ => Err(NomErr::Error(error_position!(input, ErrorKind::Alt)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::Err;
    use nom::error::ErrorKind;

    mod util {
        pub fn bytes_with_sfnt_version(sfnt_version: &[u8]) -> [u8; 12] {
            let rest: &[u8] = &[0x00, 0x12, 0x01, 0x00, 0x00, 0x04, 0x00, 0x20];
            let bytes = &[sfnt_version, rest].concat();
            let mut array = [0; 12];
            array.copy_from_slice(bytes);
            array
        }
    }

    #[test]
    fn case_sfnt_version_tt_1() {
        let bytes = util::bytes_with_sfnt_version(&[0x00, 0x01, 0x00, 0x00]);
        assert_eq!(parse_sfnt_version(&bytes).unwrap().1, SfntVersion::TrueType);
    }

    #[test]
    fn case_sfnt_version_tt_2() {
        let bytes = util::bytes_with_sfnt_version("true".as_bytes());
        assert_eq!(parse_sfnt_version(&bytes).unwrap().1, SfntVersion::TrueType);
    }

    #[test]
    fn case_sfnt_version_tt_3() {
        let bytes = util::bytes_with_sfnt_version("typ1".as_bytes());
        assert_eq!(parse_sfnt_version(&bytes).unwrap().1, SfntVersion::TrueType);
    }

    #[test]
    fn case_sfnt_version_cff() {
        let bytes = util::bytes_with_sfnt_version("OTTO".as_bytes());
        assert_eq!(parse_sfnt_version(&bytes).unwrap().1, SfntVersion::CFF);
    }

    #[test]
    fn case_offset_table() {
        let bytes: &[u8] = &[
            0x00, 0x01, 0x00, 0x00, 0x00, 0x12, 0x01, 0x00, 0x00, 0x04, 0x00, 0x20,
        ];

        let offset_table = parse_offset_table(bytes).unwrap().1;

        assert_eq!(offset_table.sfnt_version(), SfntVersion::TrueType);
        assert_eq!(offset_table.num_tables(), 18);
        assert_eq!(offset_table.search_range(), 256);
        assert_eq!(offset_table.entry_selector(), 4);
        assert_eq!(offset_table.range_shift(), 32);
    }

    #[test]
    fn case_offset_table_invalid_empty_slice() {
        let bytes: &[u8] = &[];

        let expected = Err(Err::Error(error_position!(bytes, ErrorKind::Eof)));
        assert_eq!(parse_offset_table(bytes), expected);
    }

    #[test]
    fn case_offset_table_invalid_sfnt_version() {
        let bytes: &[u8] = &[
            0x00, 0x00, 0x00, 0x00, 0x00, 0x0f, 0x00, 0x80, 0x00, 0x03, 0x00, 0x70,
        ];


        let expected = Err(Err::Error(error_position!(&bytes[4..], ErrorKind::Alt)));
        assert_eq!(parse_offset_table(bytes), expected);
    }
}
