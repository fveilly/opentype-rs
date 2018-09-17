use byteorder::{ByteOrder, BigEndian};
use parser::TableRecord;
use types::{TableTag, Tag};
use types::HexSlice;
use std::{fmt, cmp};

pub struct Table<'otf> {
    buf: &'otf[u8],
    tag: TableTag,
    table_record: TableRecord,
}

impl<'otf> Table<'otf> {
    pub fn new(buf: &'otf[u8], tag: TableTag, table_record: TableRecord) -> Table {
        Table {
            buf,
            tag,
            table_record
        }
    }

    pub fn tag(&self) -> TableTag {
        self.tag
    }

    pub fn validate(&self) -> bool {
        if self.table_record.length() == 0 {
            return self.table_record.check_sum() == 0;
        }

        let boundary : usize = (self.table_record.offset() + ((self.table_record.length() + 3) & !3)) as usize;

        if self.buf.len() < boundary  {
            return false;
        }

        let table_buf = &self.buf[self.table_record.offset() as usize..boundary];
        let mut sum : u32 = 0;

        // FIXME: Should use exact_chuncks instead of chuncks when stable (cf. #47115)
        let mut iter = table_buf.chunks(4);

        match self.tag {
            TableTag::Head => {
                // Compute the checksum for the first 8 bits of the 'head' table
                for i in 0..2 {
                    sum = match iter.next() {
                        Some(chunk) if chunk.len() >= 4 => sum.wrapping_add(BigEndian::read_u32(chunk)),
                        _ => return false
                    };
                }

                // Ignore the checkSumAdjustment field (32 bits) while computing the checksum of the
                // 'head' table
                if iter.next().is_none() {
                    return false;
                }

                // Compute the remaining bits
                for chunk in iter {
                    if chunk.len() >= 4 {
                        sum = sum.wrapping_add(BigEndian::read_u32(chunk));
                    }
                }
            },
            _ => {
                // FIXME: Should use exact_chuncks instead of chuncks when stable (cf. #47115)
                for chunk in iter {
                    if chunk.len() >= 4 {
                        sum = sum.wrapping_add(BigEndian::read_u32(chunk));
                    }
                }
            }
        }

        sum == self.table_record.check_sum()
    }
}

impl<'otf> fmt::Display for Table<'otf> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let boundary : usize = (self.table_record.offset() + self.table_record.length()) as usize;

        if self.buf.len() < boundary {
            return Err(fmt::Error);
        }

        write!(f, "{}", HexSlice::new(&self.buf[self.table_record.offset() as usize..boundary]))
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_offset_table_typefont() {
        let bytes: &[u8]  = &[
            0x00, 0x01, 0x00, 0x00, 0x00, 0x02, 0x23, 0x12,
            0x8A, 0x7F, 0x70, 0x48, 0x5F, 0x0F, 0x3C, 0xF5
        ];
    }
}