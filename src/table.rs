use byteorder::{ByteOrder, BigEndian};
use parser::TableRecord;
use types::Tag;
use types::HexSlice;
use std::{fmt, cmp};

pub struct Table<'otf> {
    buf: &'otf[u8],
    table_record: TableRecord
}

impl<'otf> Table<'otf> {
    pub fn new(buf: &'otf[u8], table_record: TableRecord) -> Table {
        Table {
            buf,
            table_record
        }
    }

    pub fn tag(&self) -> Tag {
        self.table_record.table_tag()
    }

    // TODO: move checksum compute part on parser
    pub fn validate(&self) -> bool {
        if self.table_record.length() == 0 {
            return self.table_record.check_sum() == 0;
        }

        let boundary : usize = (self.table_record.offset() + ((self.table_record.length() + 3) & !3)) as usize;

        if self.buf.len() < boundary  {
            return false;
        }

        let table_buf = &self.buf[self.table_record.offset() as usize..boundary];

        if &self.table_record.table_tag() == b"head" {
            self.validate_head(table_buf)
        }
        else {
            let mut sum : u32 = 0;

            // FIXME: Should use exact_chuncks instead of chuncks when stable (cf. #47115)
            for chunk in table_buf.chunks(4) {
                if chunk.len() >= 4 {
                    sum = sum.wrapping_add(BigEndian::read_u32(chunk));
                }
            }

            sum == self.table_record.check_sum()
        }
    }

    fn validate_head(&self, buf: &[u8]) -> bool {
        // Find the checkSumAdjustment offset based on the magicNumber offset
        let magic_number_offset = match self.find_head_magic_number_offset(buf) {
            Some(offset) => offset,
            _ => return false
        };

        if magic_number_offset < 8 || buf.len() < magic_number_offset + 4 {
            return false;
        }

        let magic_number_chunck = &buf[magic_number_offset..magic_number_offset+4];
        let check_sum_adjustment_offset = magic_number_offset - 4;

        if BigEndian::read_u32(magic_number_chunck) != 0x5F0F3CF5 {
            return false;
        }

        // FIXME: Should use exact_chuncks instead of chuncks when stable (cf. #47115)
        let mut iter = buf.chunks(4);
        let mut sum : u32 = 0;

        for (index, chunk) in iter.enumerate() {
            if chunk.len() >= 4 {
                let chunk_offset = index * 4;

                // Ignore the checkSumAdjustment bits while computing the checksum of the head table
                if chunk_offset >= check_sum_adjustment_offset && chunk_offset < check_sum_adjustment_offset + 4 {
                    let mut arr : [u8; 4] = Default::default();
                    arr.copy_from_slice(chunk);
                    for i in chunk_offset - check_sum_adjustment_offset..4 {
                        arr[i] = 0;
                    }
                    sum = sum.wrapping_add(BigEndian::read_u32(&arr));
                }
                else {
                    sum = sum.wrapping_add(BigEndian::read_u32(chunk));
                }
            }
        }

        sum == self.table_record.check_sum()
    }

    fn find_head_magic_number_offset(&self, buf: &[u8]) -> Option<usize> {
        for (index, chunk) in buf.windows(4).enumerate() {
            if chunk.len() >= 4 && BigEndian::read_u32(chunk) == 0x5F0F3CF5 {
                return Some(index);
            }
        }
        None
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