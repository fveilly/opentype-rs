use error::Error;
use parser;
use types::{TableTag, Tag};
use std::{fmt, cmp, ops};
use parser::table_record::{compute_checksum, compute_checksum_for_head};

pub struct TableRecord<'otf> {
    buf: &'otf[u8],
    table_record: parser::TableRecord,
    tag: TableTag
}

impl<'otf> TableRecord<'otf> {
    pub(crate) fn new(buf: &'otf[u8], table_record: parser::TableRecord) -> Option<TableRecord<'otf>> {
        let tag = match TableTag::parse(table_record.table_tag()) {
            Some(table_tag) => table_tag,
            _ => return None
        };

        let table_record_len = table_record.length() as usize;
        let table_record_offset = table_record.offset() as usize;

        // All tables must begin on four-byte boundaries, and any remaining space between tables
        // is padded with zeros. The length of all tables should be recorded in the table record
        // with their actual length (not their padded length).
        let offset_limit = table_record_offset + table_record_len + table_record_len % 4;

        buf.get(table_record_offset..offset_limit).map(|table_buf| {
            TableRecord {
                buf: table_buf,
                table_record,
                tag
            }
        })
    }

    /// Table tag.
    pub fn tag(&self) -> TableTag {
        self.tag
    }

    /// Compute checksum of the table and verify it matches with the TableRecord value.
    pub fn validate(&self) -> bool {
        match self.tag {
            TableTag::Head => {
                match compute_checksum_for_head(self.buf) {
                    Some(checksum) => checksum == self.table_record.check_sum(),
                    _ => false
                }
            },
            _ => {
                match compute_checksum(self.buf) {
                    Some(checksum) => checksum == self.table_record.check_sum(),
                    _ => false
                }
            }
        }
    }
}

impl<'otf> fmt::Display for TableRecord<'otf> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:X?}", self.buf)
    }
}

impl<'otf> ops::Deref for TableRecord<'otf> {
    type Target = &'otf[u8];
    fn deref(&self) -> &Self::Target {
        &self.buf
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_table_record_empty() {
        let table_record = TableRecord::new(
            &[] as &[u8], parser::TableRecord::new(
                Tag::new(b"cmap"), 0, 0, 0)).unwrap();

        assert_eq!(*table_record, &[] as &[u8]);
        assert!(table_record.validate());
    }

    #[test]
    fn case_table_record_invalid_offset() {
        let bytes: &[u8]  = &[0x05, 0x48, 0x65, 0x6C, 0x6C, 0x6F, 0x05, 0x57,
            0x6F, 0x72, 0x6C, 0x64];

        assert!(TableRecord::new(
            bytes, parser::TableRecord::new(
                Tag::new(b"cmap"), 0, 13, 5)).is_none());
    }

    #[test]
    fn case_table_record_invalid_length() {
        let bytes: &[u8]  = &[0x05, 0x48, 0x65, 0x6C, 0x6C, 0x6F, 0x05, 0x57,
            0x6F, 0x72, 0x6C, 0x64];

        assert!(TableRecord::new(
            bytes, parser::TableRecord::new(
                Tag::new(b"cmap"), 0, 0, 13)).is_none());

        assert!(TableRecord::new(
            bytes, parser::TableRecord::new(
                Tag::new(b"cmap"), 0, 4, 7)).is_none());
    }
}