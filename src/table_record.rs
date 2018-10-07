use error::Error;
use parser;
use types::{TableTag, Tag};
use std::{fmt, cmp, ops};
use parser::table_record::{compute_checksum, compute_checksum_for_head};
use nom::types::CompleteByteSlice;

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

        if table_record_len == 0 {
            return None
        }

        let offset_limit = table_record_offset + table_record_len + table_record_len % 4;

        if buf.len() < offset_limit {
            return None
        }

        Some(TableRecord {
            buf: &buf[table_record_offset..offset_limit],
            table_record,
            tag
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