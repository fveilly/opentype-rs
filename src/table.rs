use error::Error;
use tables::{TableTag, Tag};
use std::{fmt, cmp, ops};
use table_record::{compute_checksum, compute_checksum_for_head};

pub struct Table<'otf> {
    buf: &'otf[u8],
    tag: TableTag,
    check_sum: u32,
    offset: usize,
    length: usize
}

impl<'otf> Table<'otf> {
    pub(crate) fn new(buf: &'otf[u8], tag: TableTag, check_sum: u32, offset: usize, length: usize) -> Table<'otf> {
        Table {
            buf,
            tag,
            check_sum,
            offset,
            length
        }
    }

    /// Get the slice of the table.
    ///
    /// If the table record content is corrupted or the checksum does not match this method
    /// shall return None. Else return the actual slice of the table non padded.
    pub(crate) fn get_table_as_slice(&self) -> Result<&'otf[u8], Error> {
        // All tables must begin on four-byte boundaries, and any remaining space between tables
        // is padded with zeros. The length of all tables should be recorded in the table record
        // with their actual length (not their padded length).
        let offset_limit = self.offset + self.length + self.length % 4;

        let table_padded_buf = self.buf.get(self.offset..offset_limit).ok_or(Error::new("Table slice out of bounds"))?;

        match self.tag {
            TableTag::Head => {
                let checksum = compute_checksum_for_head(table_padded_buf)?;

                if (checksum != self.check_sum) {
                    return Err(Error::new(format!("Invalid checksum: expected {} got {}", self.check_sum, checksum)))
                }

                Ok(&table_padded_buf[..self.length])
            },
            _ => {
                let checksum = compute_checksum(table_padded_buf)?;

                if (checksum != self.check_sum) {
                    return Err(Error::new(format!("Invalid checksum: expected {} got {}", self.check_sum, checksum)))
                }

                Ok(&table_padded_buf[..self.length])
            }
        }
    }

    /// Table tag.
    pub fn tag(&self) -> TableTag {
        self.tag
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_table_record() {
        let bytes: &[u8]  = &[0x05, 0x48, 0x65, 0x6C, 0x6C, 0x6F, 0x05, 0x00,
            0x00, 0x00, 0x00, 0x00];

        assert_eq!(Table::new(
            bytes, TableTag::Cmap,1907845740, 0, 7).get_table_as_slice().unwrap(), &bytes[..7]);
    }

    #[test]
    fn case_table_record_empty() {
        let table_record = Table::new(
            &[] as &[u8], TableTag::Cmap,0, 0, 0);

        assert_eq!(table_record.get_table_as_slice().unwrap(), &[] as &[u8]);
    }

    #[test]
    fn case_table_record_invalid_offset() {
        let bytes: &[u8]  = &[0x05, 0x48, 0x65, 0x6C, 0x6C, 0x6F, 0x05, 0x57,
            0x6F, 0x72, 0x6C, 0x64];

        assert!(Table::new(
            bytes, TableTag::Cmap,0, 13, 5).get_table_as_slice().is_err());
    }

    #[test]
    fn case_table_record_invalid_length() {
        let bytes: &[u8]  = &[0x05, 0x48, 0x65, 0x6C, 0x6C, 0x6F, 0x05, 0x57,
            0x6F, 0x72, 0x6C, 0x64];

        assert!(Table::new(
            bytes, TableTag::Cmap,0, 0, 13).get_table_as_slice().is_err());

        assert!(Table::new(
            bytes, TableTag::Cmap,0, 4, 7).get_table_as_slice().is_err());
    }
}