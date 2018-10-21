use parser;
use std::ops;
use error::Error;
use traits::{Parser, TableParser};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CharacterGlyphIndexMappingTable<'otf> {
    buf: &'otf[u8],
    table: parser::tables::CharacterGlyphIndexMappingTable
}

impl<'otf> CharacterGlyphIndexMappingTable<'otf> {
    pub fn iter(&self) -> CharacterGlyphIndexMappingTableIterator<'otf> {
        CharacterGlyphIndexMappingTableIterator {
            buf: self.buf,
            num_tables: self.table.num_tables(),
            pos: 0
        }
    }
}

impl<'otf> Parser<'otf> for CharacterGlyphIndexMappingTable<'otf> {
    type Item = CharacterGlyphIndexMappingTable<'otf>;

    /// Parse Character to Glyph Index Mapping Table.
    ///
    /// # Example
    ///
    /// ```
    /// // TODO
    /// ```
    fn parse(buf: &'otf[u8]) -> Result<Self::Item, Error> {
        let res = parser::tables::parse_character_glyph_index_mapping_table(buf)?;

        Ok(CharacterGlyphIndexMappingTable {
            buf: res.0,
            table: res.1
        })
    }
}

impl<'otf> TableParser<'otf> for CharacterGlyphIndexMappingTable<'otf> {}

impl<'otf> ops::Deref for CharacterGlyphIndexMappingTable<'otf> {
    type Target = parser::tables::CharacterGlyphIndexMappingTable;
    fn deref(&self) -> &Self::Target {
        &self.table
    }
}

pub struct CharacterGlyphIndexMappingTableIterator<'otf> {
    buf: &'otf[u8],
    num_tables: u16,
    pos: u16
}

impl<'otf> Iterator for CharacterGlyphIndexMappingTableIterator<'otf> {
    type Item = parser::tables::EncodingRecord<'otf>;

    fn next(&mut self) -> Option<parser::tables::EncodingRecord<'otf>> {
        if self.pos >= self.num_tables {
            return None;
        }

        match parser::tables::parse_encoding_record(self.buf) {
            Ok((bytes, encoding_record)) => {
                self.buf = bytes;
                self.pos = self.pos + 1;
                Some(encoding_record)
            },
            _ => None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = usize::from(self.num_tables);
        (size, Some(size))
    }
}
