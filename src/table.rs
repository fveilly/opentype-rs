use byteorder::{ByteOrder, BigEndian};
use error::Error;
use parser::{TableRecord};
use parser::tables::{FontTable, parse_table};
use types::{TableTag, Tag};
use std::{fmt, cmp};

pub struct Table<'otf> {
    buf: &'otf[u8],
    tag: TableTag,
    checksum: u32
}

impl<'otf> Table<'otf> {
    pub fn new(buf: &'otf[u8], tag: TableTag, checksum: u32) -> Table {
        Table {
            buf,
            tag,
            checksum
        }
    }

    pub fn tag(&self) -> TableTag {
        self.tag
    }

    pub fn validate(&self) -> bool {
        let mut sum : u32 = 0;

        // FIXME: Should use exact_chuncks instead of chuncks when stable (cf. #47115)
        let mut iter = self.buf.chunks(4);

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

        sum == self.checksum
    }

    pub fn parse(&self) -> Result<FontTable, Error> {
        Ok(parse_table(self.buf, self.tag)?.1)
    }
}

impl<'otf> fmt::Display for Table<'otf> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:X?}", self.buf)
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