use error::Error;
use nom::{be_i16, be_u16, IResult};

/// Horizontal Metrics Table
///
/// Glyph metrics used for horizontal text layout include glyph advance widths, side bearings and
/// X-direction min and max values (xMin, xMax). These are derived using a combination of the glyph
/// outline data ('glyf', 'CFF ' or CFF2) and the horizontal metrics table. The horizontal metrics
/// ('hmtx') table provides glyph advance widths and left side bearings.
///
/// In a font with TrueType outline data, the 'glyf' table provides xMin and xMax values, but not
/// advance widths or side bearings. The advance width is always obtained from the 'hmtx' table.
/// In some fonts, depending on the state of flags in the 'head' table, the left side bearings may
/// be the same as the xMin values in the 'glyf' table, though this is not true for all fonts. (See
/// the description of bit 1 of the flags field in the 'head' table.) For this reason, left side
/// bearings are provided in the 'hmtx' table. The right side bearing is always derived using
/// advance width and left side bearing values from the 'hmtx' table, plus bounding-box information
/// in the glyph description.
///
/// In a variable font with TrueType outline data, the left side bearing value in the 'hmtx' table
/// must always be equal to xMin (bit 1 of the 'head' flags field must be set). Hence, these values
/// can also be derived directly from the 'glyf' table. Note that these values apply only to the
/// default instance of the variable font: non-default instances may have different side bearing
/// values. These can be derived from interpolated “phantom point” coordinates using the 'gvar'
/// table (see below for additional details), or by applying variation data in the HVAR table to
/// default-instance values from the 'glyf' or 'hmtx' table.
///
/// In a font with CFF version 1 outline data, the 'CFF ' table does include advance widths. These
/// values are used by PostScript processors, but are not used in OpenType layout. In an OpenType
/// context, the 'hmtx' table is required and must be used for advance widths. Note that fonts in a
/// Font Collection file that share a 'CFF ' table may specify different advance widths in
/// font-specific 'hmtx' tables for a particular glyph index. Also note that the CFF2 table does
/// not include advance widths. In addition, for either CFF or CFF2 data, there are no explicit
/// xMin and xMax values; side bearings are implicitly contained within the CharString data, and
/// can be obtained from the the CFF / CFF2 rasterizer. Some layout engines may use left side
/// bearing values in the 'hmtx' table, however; hence, font production tools should ensure that
/// the lsb values in the 'hmtx' table match the implicit xMin values reflected in the CharString
/// data. In a variable font with CFF2 outline data, left side bearing and advance width values
/// for non-default instances should be obtained by combining information from the 'hmtx' and HVAR
/// tables.
///
/// The table uses a longHorMetric record to give the advance width and left side bearing of a
/// glyph. Records are indexed by glyph ID. As an optimization, the number of records can be less
/// than the number of glyphs, in which case the advance width value of the last record applies to
/// all remaining glyph IDs. This can be useful in monospaced fonts, or in fonts that have a large
/// number of glyphs with the same advance width (provided the glyphs are ordered appropriately).
/// The number of longHorMetric records is determined by the numberOfHMetrics field in the 'hhea'
/// table.
///
/// If the longHorMetric array is less than the total number of glyphs, then that array is followed
/// by an array for the left side bearing values of the remaining glyphs. The number of elements
/// in the left side bearing will be derived from numberOfHMetrics plus the numGlyphs field in the
/// 'maxp' table.
///
/// More information on ['hmtx'](https://docs.microsoft.com/en-gb/typography/opentype/spec/hmtx)
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct HorizontalMetricsTable {
    h_metrics: Vec<LongHorMetricRecord>,
    left_side_bearings: Vec<i16>
}

impl<'otf> HorizontalMetricsTable {
    pub fn h_metrics(&self) -> &Vec<LongHorMetricRecord> {
        &self.h_metrics
    }

    pub fn left_side_bearings(&self) -> &[i16] {
        &self.left_side_bearings
    }

    /// Parse Horizontal Metrics Table.
    ///
    /// * `number_of_hmetrics` - The number of longHorMetric records is determined by the
    /// [numberOfHMetrics](./Hhea.t.html#method.number_of_hmetrics) field in the 'hhea' table.
    /// * `num_glyphs` - The number of glyphs in the font is determined by the
    /// [numGlyphs](./Maxp.t.html#method.num_glyphs) field in the 'maxp' table.
    ///
    /// # Example
    ///
    /// ```
    /// extern crate opentype_rs as otf;
    ///
    /// use otf::tables::hmtx::HorizontalMetricsTable;
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
        Ok(parse_horizontal_metrics_table(buf, number_of_hmetrics, num_glyphs)?.1)
    }
}

/// In a font with TrueType outlines, xMin and xMax values for each glyph are given in the 'glyf'
/// table. The advance width (“aw”) and left side bearing (“lsb”) can be derived from the glyph
/// “phantom points”, which are computed by the TrueType rasterizer; or they can be obtained from
/// the 'hmtx' table. In a font with CFF or CFF2 outlines, xMin (= left side bearing) and xMax
/// values can be obtained from the CFF / CFF2 rasterizer. From those values, the right side
/// bearing (“rsb”) is calculated as follows:
/// >> rsb = aw - (lsb + xMax - xMin)
///
/// If pp1 and pp2 are TrueType phantom points used to control lsb and rsb, their initial position
/// in the X-direction is calculated as follows:
/// >> pp1 = xMin - lsb
/// >> pp2 = pp1 + aw
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct LongHorMetricRecord {
    advance_width: u16,
    lsb: i16
}

impl LongHorMetricRecord {
    /// Advance width, in font design units.
    pub fn advance_width(&self) -> u16 {
        self.advance_width
    }

    /// Glyph left side bearing, in font design units.
    pub fn lsb(&self) -> i16 {
        self.lsb
    }
}

pub fn parse_horizontal_metrics_table(input: &[u8], number_of_hmetrics: u16, num_glyphs: u16)
    -> IResult<&[u8], HorizontalMetricsTable> {
    do_parse!(
        input,
        h_metrics: count!(parse_long_hor_metric_record, usize::from(number_of_hmetrics)) >>
        left_side_bearings: map!(cond!(number_of_hmetrics < num_glyphs,
            count!(be_i16, usize::from(num_glyphs - number_of_hmetrics))), |left_side_bearings_opt| {
                left_side_bearings_opt.unwrap_or(Vec::new())
            }) >>
        (
            HorizontalMetricsTable {
                h_metrics,
                left_side_bearings
            }
        )
    )
}

named!(parse_long_hor_metric_record<&[u8],LongHorMetricRecord>,
    do_parse!(
        advance_width: be_u16 >>
        lsb: be_i16 >>
        (
            LongHorMetricRecord {
                advance_width,
                lsb
            }
        )
    )
);

#[cfg(test)]
mod tests {
    use super::*;
    use nom::{Err, Needed};

    #[test]
    fn case_horizontal_metrics_table_left_side_bearings() {
        let bytes: &[u8] = &[0x03, 0x8C, 0x00, 0x64, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x01, 0xFB, 0x00, 0x00];

        let expected = (&b""[..], HorizontalMetricsTable {
            h_metrics: Vec::new(),
            left_side_bearings: vec![908, 100, 0, 0, 0, 0, 507, 0],
        });

        let res = parse_horizontal_metrics_table(bytes, 0, 8).unwrap();
        assert_eq!(res,  expected);
    }

    #[test]
    fn case_horizontal_metrics_table_invalid_empty_slice() {
        let bytes: &[u8] = &[];

        let expected = Result::Err(Err::Incomplete(Needed::Size(2)));
        assert_eq!(parse_horizontal_metrics_table(bytes, 10, 10), expected);
    }
}
