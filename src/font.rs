use parser::{SfntVersion, OffsetTable, TableRecord};
use table::Table;
use types::{Tag, TableTag};

pub struct Font<'otf> {
    buf: &'otf[u8],
    sfnt_version: SfntVersion,
    table_records: Vec<TableRecord>
}

impl<'otf> Font<'otf> {
    pub fn new(buf: &'otf[u8], sfnt_version: SfntVersion, table_records: Vec<TableRecord>) -> Font<'otf> {
        Font {
            buf,
            sfnt_version,
            table_records
        }
    }

    pub fn iter(&self) -> FontIterator {
        FontIterator {
            buf: self.buf,
            table_records: &self.table_records,
            pos: 0
        }
    }

    pub fn sfnt_version(&self) -> SfntVersion {
        self.sfnt_version
    }

    pub fn search(&self, table_tag: TableTag) -> Option<Table> {
        let tag: Tag = Tag::from(table_tag);
        match self.table_records.binary_search_by(|table_record| table_record.table_tag().cmp(&tag)) {
            Ok(index) => self.table_records.get(index).map(|table_record| Table::new(self.buf, *table_record)),
            _ => None
        }
    }
}

pub struct FontIterator<'otf, 'a> {
    buf: &'otf[u8],
    table_records: &'a[TableRecord],
    pos: usize
}

impl<'otf, 'a> Iterator for FontIterator<'otf, 'a> {
    type Item = Table<'otf>;

    fn next(&mut self) -> Option<Table<'otf>> {
        if self.pos > self.table_records.len() {
            None
        }
        else {
            let element = self.table_records.get(self.pos);
            self.pos = self.pos + 1;
            element.map(|table_record| Table::new(self.buf, *table_record))
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = self.table_records.len();
        (size, Some(size))
    }
}