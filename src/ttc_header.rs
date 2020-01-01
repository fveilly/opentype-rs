use nom::multi::count;
use nom::Err as NomErr;
use nom::IResult;
use nom::bytes::complete::tag;
use nom::error::ErrorKind;
use nom::combinator::map_res;
use nom::number::complete::{be_u16, be_u32};
use std::convert::TryFrom;
use types::Offset32;

/// The purpose of the TTC Header table is to locate the different Offset Tables within a TTC file.
/// The TTC Header is located at the beginning of the TTC file (offset = 0). It consists of an
/// identification tag, a version number, a count of the number of OpenType fonts in the file, and
/// an array of offsets to each Offset Table.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TTCHeader {
    offset_table: Vec<Offset32>,
    dsig: Option<TTCDigitalSignature>
}

impl TTCHeader {
    /// Array of offsets to the OffsetTable for each font from the beginning of the file.
    #[allow(dead_code)]
    pub fn offset_table(&self) -> &[u32] {
        &self.offset_table
    }

    /// There are two versions of the TTC Header: Version 1.0 has been used for TTC files without
    /// digital signatures. Version 2.0 can be used for TTC files with or without digital
    /// signatures.
    #[allow(dead_code)]
    pub fn dsig(&self) -> Option<TTCDigitalSignature> {
        self.dsig
    }
}

/// Digital Signature header.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct TTCDigitalSignature {
    dsig_length: u32,
    dsig_offset: u32,
}

impl TTCDigitalSignature {
    /// The length (in bytes) of the DSIG table
    #[allow(dead_code)]
    pub fn dsig_length(&self) -> u32 {
        self.dsig_length
    }

    /// The offset (in bytes) of the DSIG table from the beginning of the TTC
    #[allow(dead_code)]
    pub fn dsig_offset(&self) -> u32 {
        self.dsig_offset
    }
}

pub fn parse_ttc_header(input: &[u8]) -> IResult<&[u8], TTCHeader>
{
    let (input, _) = tag("ttcf")(input)?;
    let (input, major_version) = be_u16(input)?;
    let (input, minor_version) = be_u16(input)?;

    if major_version == 1 && minor_version == 0 {
        let (input, num_fonts) = map_res(be_u32, |v| usize::try_from(v))(input)?;
        let (input, offset_table) = count(be_u32, num_fonts)(input)?;

        Ok((input, TTCHeader {
            offset_table,
            dsig: None
        }))
    }
    else if major_version == 2 && minor_version == 0 {
        let (input, num_fonts) = map_res(be_u32, |v| usize::try_from(v))(input)?;
        let (input, offset_table) = count(be_u32, num_fonts)(input)?;
        let (input, dsig_tag) = be_u32(input)?;
        let (input, dsig_length) = be_u32(input)?;
        let (input, dsig_offset) = be_u32(input)?;

        // If there’s no signature, then the last three fields of the version 2.0 header
        // are left null
        let dsig = match dsig_tag {
            0x44534947 => {
                Some(TTCDigitalSignature {
                    dsig_length,
                    dsig_offset
                })
            },
            _ => None
        };

        Ok((input, TTCHeader {
            offset_table,
            dsig
        }))
    }
    else {
        Err(NomErr::Error(error_position!(input, ErrorKind::Alt)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::Err;
    use nom::error::ErrorKind;

    #[test]
    fn case_ttc_header_v1_0() {
        let bytes: &[u8] = &[
            0x74, 0x74, 0x63, 0x66, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00,
            0x00, 0x2C, 0x00, 0x00, 0x01, 0x28, 0x00, 0x00, 0x02, 0x24, 0x00, 0x00, 0x03, 0x20,
            0x00, 0x00, 0x04, 0x1C, 0x00, 0x00, 0x05, 0x18, 0x00, 0x00, 0x06, 0x14, 0x00, 0x00,
            0x07, 0x10, 0x4F, 0x54, 0x54, 0x4F];

        let ttc_header = parse_ttc_header(bytes).unwrap().1;

        assert_eq!(ttc_header.offset_table().len(), 8);
        assert_eq!(ttc_header.offset_table(), &([44, 296, 548, 800, 1052, 1304, 1556, 1808] as [u32; 8]));
        assert_eq!(ttc_header.dsig(), None);
    }

    #[test]
    fn case_ttc_header_v2_0() {
        // TODO
    }

    #[test]
    fn case_ttc_header_invalid_version() {
        let bytes: &[u8]  = &[
            0x74, 0x74, 0x63, 0x66, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x1C,
            0x00, 0x00, 0x01, 0x18, 0x00, 0x00, 0x02, 0x14,
            0x00, 0x00, 0x03, 0x10
        ];
        let expected = Err(Err::Error(error_position!(&bytes[8..], ErrorKind::Alt)));

        let res = parse_ttc_header(bytes);
        assert_eq!(res,  expected);
    }
}