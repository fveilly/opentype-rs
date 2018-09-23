use parser;
use std::ops;
use error::Error;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct HorizontalMetricsTable<'otf> {
    buf: &'otf[u8],
    table: parser::tables::HorizontalMetricsTable
}

impl<'otf> HorizontalMetricsTable<'otf> {
    /// Parse Horizontal Metrics Table.
    ///
    /// * `number_of_hmetrics` - The number of longHorMetric records is determined by the
    /// [numberOfHMetrics](./Hhea.t.html#method.number_of_hmetrics) field in the 'hhea' table.
    /// * `num_glyphs` - The number of glyphs in the font is determined by the
    /// [numGlyphs](./Maxp.t.html#method.num_glyphs) field in the 'hhea' table.
    ///
    /// # Example
    ///
    /// ```
    /// extern crate opentype_rs as otf;
    ///
    /// use otf::HorizontalMetricsTable;
    ///
    /// let bytes: &[u8]  = &[
    ///     0x03, 0x8C, 0x00, 0x64, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0xFB,
    ///     0x00, 0x00];
    ///
    /// let horizontal_metrics_table = HorizontalMetricsTable::parse(bytes, 4, 4).unwrap();
    ///
    /// assert_eq!(horizontal_metrics_table.h_metrics().len(), 4);
    /// assert!(horizontal_metrics_table.left_side_bearings().is_empty());
    ///
    /// assert_eq!(horizontal_metrics_table.h_metrics().get(0).unwrap().advance_width(), 908);
    /// assert_eq!(horizontal_metrics_table.h_metrics().get(0).unwrap().lsb(), 100);
    /// ```
    pub fn parse(buf: &'otf[u8], number_of_hmetrics: u16, num_glyphs: u16) -> Result<HorizontalMetricsTable, Error> {
        let res = parser::tables::parse_horizontal_metrics_table(
            buf, number_of_hmetrics, num_glyphs)?;

        Ok(HorizontalMetricsTable {
            buf: res.0,
            table: res.1
        })
    }
}

impl<'otf> ops::Deref for HorizontalMetricsTable<'otf> {
    type Target = parser::tables::HorizontalMetricsTable;
    fn deref(&self) -> &Self::Target {
        &self.table
    }
}