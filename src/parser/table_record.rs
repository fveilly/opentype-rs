use nom::be_u32;
use types::{Offset32, Tag};

/// The Offset Table is followed immediately by the Table Record entries. Entries in the Table
/// Record must be sorted in ascending order by tag. Offset values in the Table Record are measured
/// from the start of the font file.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct TableRecord {
    table_tag: Tag,
    check_sum: u32,
    offset: Offset32,
    length: u32
}

impl TableRecord {
    /// Table identifier
    pub fn table_tag(&self) -> Tag {
        self.table_tag
    }

    /// CheckSum for this table
    pub fn check_sum(&self) -> u32 {
        self.check_sum
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

#[cfg(test)]
mod tests {
    use super::*;
    use nom::{Err, ErrorKind, Context};
    use types::TableTag;

    static ROBOTO_REGULAR: &[u8] = include_bytes!("../../fonts/Roboto/Roboto-Regular.ttf");

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
}