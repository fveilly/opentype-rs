use offset_table::OffsetTable;
use std::ops;
use table::Table;
use tables::TableTag;
use table_record::parse_table_record;

pub struct Font<'otf> {
    buf: &'otf[u8],
    remainder: &'otf[u8],
    offset_table: OffsetTable
}

impl<'otf> Font<'otf> {
    pub(crate) fn new(buf: &'otf[u8], remainder: &'otf[u8], offset_table: OffsetTable) -> Font<'otf> {
        Font {
            buf,
            remainder,
            offset_table
        }
    }

    /// TableRecord iterator. Each iteration will parse the next TableRecord lazily.
    pub fn iter(&self) -> FontIterator {
        FontIterator {
            buf: self.buf,
            remainder: self.remainder,
            num_tables: self.offset_table.num_tables(),
            pos: 0
        }
    }
}

impl<'otf> IntoIterator for Font<'otf> {
    type Item = Table<'otf>;
    type IntoIter = FontIterator<'otf>;

    fn into_iter(self) -> Self::IntoIter {
        FontIterator {
            buf: self.buf,
            remainder: self.remainder,
            num_tables: self.offset_table.num_tables(),
            pos: 0
        }
    }
}

impl<'otf> ops::Deref for Font<'otf> {
    type Target = OffsetTable;
    fn deref(&self) -> &Self::Target {
        &self.offset_table
    }
}

pub struct FontIterator<'otf> {
    buf: &'otf[u8],
    remainder: &'otf[u8],
    num_tables: u16,
    pos: u16
}

impl<'otf> Iterator for FontIterator<'otf> {
    type Item = Table<'otf>;

    /// Try to parse the next TableRecord.
    ///
    /// If the parsing fail or if the last TableRecord has been parsed, return None. If the
    /// table tag is unknown, skip and try to parse the next one.
    fn next(&mut self) -> Option<Table<'otf>> {
        loop {
            if self.pos >= self.num_tables {
                break;
            }

            match parse_table_record(self.remainder) {
                Ok((bytes, table_record)) => {
                    self.remainder = bytes;
                    self.pos = self.pos + 1;

                    if let Some(tag) = TableTag::parse(table_record.table_tag()) {
                        return Some(Table::new(self.buf, tag, table_record.check_sum(),
                                                table_record.offset() as usize, table_record.length() as usize));
                    }
                },
                _ => break
            }
        }

        None
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = usize::from(self.num_tables);
        (size, Some(size))
    }
}