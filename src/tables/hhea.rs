use parser;
use std::ops;
use error::Error;
use traits::{Parser, TableParser};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct HorizontalHeaderTable<'otf> {
    buf: &'otf[u8],
    table: parser::tables::HorizontalHeaderTable
}

impl<'otf> Parser<'otf> for HorizontalHeaderTable<'otf> {
    type Item = HorizontalHeaderTable<'otf>;

    /// Parse Horizontal Header Table.
    ///
    /// # Example
    ///
    /// ```
    /// extern crate opentype_rs as otf;
    ///
    /// use otf::HorizontalHeaderTable;
    /// use otf::traits::Parser;
    ///
    /// let bytes: &[u8]  = &[
    ///     0x00, 0x01, 0x00, 0x00, 0x07, 0x6C, 0xFE, 0x0C, 0x00, 0x00, 0x09, 0x49, 0xFA, 0x1B,
    ///     0xFE, 0x4A, 0x09, 0x30, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ///     0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x05, 0x0E];
    ///
    /// let horizontal_header_table = HorizontalHeaderTable::parse(bytes).unwrap();
    ///
    /// assert_eq!(horizontal_header_table.ascender(), 1900);
    /// assert_eq!(horizontal_header_table.descender(), -500);
    /// assert_eq!(horizontal_header_table.line_gap(), 0);
    /// assert_eq!(horizontal_header_table.advance_width_max(), 2377);
    /// assert_eq!(horizontal_header_table.min_left_side_bearing(), -1509);
    /// assert_eq!(horizontal_header_table.min_right_side_bearing(), -438);
    /// assert_eq!(horizontal_header_table.x_max_extent(), 2352);
    /// assert_eq!(horizontal_header_table.caret_slope_rise(), 1);
    /// assert_eq!(horizontal_header_table.caret_slope_run(), 0);
    /// assert_eq!(horizontal_header_table.caret_offset(), 0);
    /// assert_eq!(horizontal_header_table.metric_data_format(),  0);
    /// assert_eq!(horizontal_header_table.number_of_hmetrics(), 1294);
    /// ```
    fn parse(buf: &'otf[u8]) -> Result<Self::Item, Error> {
        let res = parser::tables::parse_horizontal_header_table(buf)?;

        Ok(HorizontalHeaderTable {
            buf: res.0,
            table: res.1
        })
    }
}

impl<'otf> TableParser<'otf> for HorizontalHeaderTable<'otf> {}

impl<'otf> ops::Deref for HorizontalHeaderTable<'otf> {
    type Target = parser::tables::HorizontalHeaderTable;
    fn deref(&self) -> &Self::Target {
        &self.table
    }
}