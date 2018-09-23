use byteorder::{ByteOrder, BigEndian};
use error::Error;
use parser;
use types::{TableTag, Tag};
use std::{fmt, cmp, ops};

pub struct TableRecord<'otf> {
    buf: &'otf[u8],
    table_record: parser::TableRecord,
    tag: TableTag
}

impl<'otf> TableRecord<'otf> {
    pub fn new(buf: &'otf[u8], table_record: parser::TableRecord) -> Option<TableRecord<'otf>> {
        let tag = match TableTag::parse(table_record.table_tag()) {
            Some(table_tag) => table_tag,
            _ => return None
        };

        if table_record.length() == 0 {
            return None
        }

        let boundary: usize = (table_record.offset() + ((table_record.length() + 3) & !3)) as usize;

        if buf.len() < boundary {
            return None
        }

        Some(TableRecord {
            buf: &buf[table_record.offset() as usize..boundary],
            table_record,
            tag
        })
    }

    pub fn tag(&self) -> TableTag {
        self.tag
    }

    pub fn validate(&self) -> bool {
        // FIXME: Should use exact_chuncks instead of chuncks when stable (cf. #47115)
        let mut iter = self.buf.chunks(4);

        let mut sum : u32 = 0;

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