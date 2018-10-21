use parser;
use std::ops;
use error::Error;
use traits::{Parser, TableParser};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FontHeaderTable<'otf> {
    buf: &'otf[u8],
    table: parser::tables::FontHeaderTable
}

impl<'otf> Parser<'otf> for FontHeaderTable<'otf> {
    type Item = FontHeaderTable<'otf>;

    /// Parse Font Header Table.
    ///
    /// # Example
    ///
    /// ```
    /// extern crate opentype_rs as otf;
    ///
    /// use otf::FontHeaderTable;
    /// use otf::Rect;
    /// use otf::traits::Parser;
    ///
    /// let bytes: &[u8]  = &[
    ///     0x00, 0x01, 0x00, 0x00, 0x00, 0x02, 0x23, 0x12, 0x8A, 0x7F, 0x70, 0x48, 0x5F, 0x0F,
    ///     0x3C, 0xF5, 0x00, 0x19, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0xC4, 0xF0, 0x11, 0x2E,
    ///     0x00, 0x00, 0x00, 0x00, 0xD5, 0x01, 0x52, 0xF4, 0xFA, 0x1B, 0xFD, 0xD5, 0x09, 0x30,
    ///     0x08, 0x73, 0x00, 0x00, 0x00, 0x09, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00];
    ///
    /// let font_header_table = FontHeaderTable::parse(bytes).unwrap();
    ///
    /// assert_eq!(font_header_table.font_revision(), 140050);
    /// assert_eq!(font_header_table.check_sum_adjustment(), 2323607624);
    /// assert_eq!(font_header_table.flags(), 25);
    /// assert_eq!(font_header_table.units_per_em(), 2048);
    /// assert_eq!(font_header_table.created(), 3304067374);
    /// assert_eq!(font_header_table.modified(), 3573633780);
    /// assert_eq!(font_header_table.bounding_box(), Rect::new(-1509, -555, 2352, 2163));
    /// assert_eq!(font_header_table.mac_style(), 0);
    /// assert_eq!(font_header_table.lowest_rec_ppem(), 9);
    /// assert_eq!(font_header_table.font_direction_hint(), 2);
    /// assert_eq!(font_header_table.index_to_loc_format(),  0);
    /// assert_eq!(font_header_table.glyph_data_format(), 0);
    /// ```
    fn parse(buf: &'otf[u8]) -> Result<Self::Item, Error> {
        let res = parser::tables::parse_font_header_table(buf)?;

        Ok(FontHeaderTable {
            buf: res.0,
            table: res.1
        })
    }
}

impl<'otf> TableParser<'otf> for FontHeaderTable<'otf> {}

impl<'otf> ops::Deref for FontHeaderTable<'otf> {
    type Target = parser::tables::FontHeaderTable;
    fn deref(&self) -> &Self::Target {
        &self.table
    }
}