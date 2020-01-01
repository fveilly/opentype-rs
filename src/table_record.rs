use nom::IResult;
use nom::number::complete::be_u32;
use nom::bytes::complete::take;
use nom::multi::{fold_many0, fold_many_m_n, count};
use tables::Tag;
use types::Offset32;

/// The Offset Table is followed immediately by the Table Record entries. Entries in the Table
/// Record must be sorted in ascending order by tag. Offset values in the Table Record are measured
/// from the start of the font file.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct TableRecord {
    table_tag: Tag,
    checksum: u32,
    offset: Offset32,
    length: u32
}

impl TableRecord {
    #[allow(dead_code)]
    pub(crate) fn new(table_tag: Tag, checksum: u32, offset: Offset32, length: u32) -> TableRecord {
        TableRecord {
            table_tag,
            checksum,
            offset,
            length
        }
    }

    /// Table identifier
    pub fn table_tag(&self) -> Tag {
        self.table_tag
    }

    /// CheckSum for this table
    pub fn checksum(&self) -> u32 {
        self.checksum
    }

    /// Offset from beginning of TrueType font file
    pub fn offset(&self) -> Offset32 {
        self.offset
    }

    /// Length of this table
    pub fn length(&self) -> u32 {
        self.length
    }
}

pub fn parse_table_records(input: &[u8], num_tables: u16) -> IResult<&[u8], Vec<TableRecord>>
{
    count(parse_table_record, usize::from(num_tables))(input)
}

pub fn parse_table_record(input: &[u8]) -> IResult<&[u8], TableRecord>
{
    let (input, table_tag) = take(4usize)(input)?;
    let (input, checksum) = be_u32(input)?;
    let (input, offset) = be_u32(input)?;
    let (input, length) = be_u32(input)?;

    Ok((input, TableRecord {
        table_tag: Tag::new(table_tag),
        checksum,
        offset,
        length
    }))
}

pub fn compute_checksum(input: &[u8]) -> IResult<&[u8], u32> {
    fold_many0(be_u32, 0, |acc: u32, v| acc.wrapping_add(v))(input)
}

pub fn compute_checksum_for_head(input: &[u8]) -> IResult<&[u8], u32> {
    let (input, s0) = fold_many_m_n(0, 2, be_u32, 0, |acc: u32, v| acc.wrapping_add(v))(input)?;
    let (input, _) = take(4usize)(input)?;
    let (input, s1) = fold_many0(be_u32, 0, |acc: u32, v| acc.wrapping_add(v))(input)?;

    Ok((input, s0.wrapping_add(s1)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use tables::TableTag;

    static ROBOTO_REGULAR: &[u8] = include_bytes!("../fonts/Roboto/Roboto-Regular.ttf");

    #[test]
    fn case_table_record() {
        let expected = (&b""[..], TableRecord {
            table_tag: Tag::from(TableTag::Gdef),
            checksum: 3024269442,
            offset: 141532,
            length: 610
        });

        let res = parse_table_record(&ROBOTO_REGULAR[12..28]).unwrap();
        assert_eq!(res,  expected);
    }
}