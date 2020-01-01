use nom::IResult;
use nom::bytes::complete::take;
use nom::combinator::verify;
use nom::number::complete::{be_i16, be_u16};

/// Horizontal Header Table
///
/// The 'hhea' table contains information needed to layout fonts whose characters are written
/// horizontally, that is, either left to right or right to left.
///
/// More information on ['hhea'](https://docs.microsoft.com/en-gb/typography/opentype/spec/hhea)
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct HorizontalHeaderTable {
    ascender: i16,
    descender: i16,
    line_gap: i16,
    advance_width_max: u16,
    min_left_side_bearing: i16,
    min_right_side_bearing: i16,
    x_max_extent: i16,
    caret_slope_rise: i16,
    caret_slope_run: i16,
    caret_offset: i16,
    metric_data_format: i16,
    number_of_hmetrics: u16
}

impl HorizontalHeaderTable {
    /// Distance from baseline of highest ascender.
    pub fn ascender(&self) -> i16 {
        self.ascender
    }

    /// Distance from baseline of lowest descender.
    pub fn descender(&self) -> i16 {
        self.descender
    }

    /// Typographic line gap.
    pub fn line_gap(&self) -> i16 {
        self.line_gap
    }

    /// Maximum advance width value in 'hmtx' table.
    pub fn advance_width_max(&self) -> u16 {
        self.advance_width_max
    }

    /// Minimum left sidebearing value in 'hmtx' table.
    pub fn min_left_side_bearing(&self) -> i16 {
        self.min_left_side_bearing
    }

    /// Minimum right sidebearing value; calculated as Min(aw - lsb - (xMax - xMin)).
    pub fn min_right_side_bearing(&self) -> i16 {
        self.min_right_side_bearing
    }

    /// Max(lsb + (xMax - xMin)).
    pub fn x_max_extent(&self) -> i16 {
        self.x_max_extent
    }

    /// Used to calculate the slope of the cursor (rise/run); 1 for vertical.
    pub fn caret_slope_rise(&self) -> i16 {
        self.caret_slope_rise
    }

    /// 0 for vertical.
    pub fn caret_slope_run(&self) -> i16 {
        self.caret_slope_run
    }

    /// The amount by which a slanted highlight on a glyph needs to be shifted to produce the best
    /// appearance. Set to 0 for non-slanted fonts.
    pub fn caret_offset(&self) -> i16 {
        self.caret_offset
    }

    /// 0 for current format.
    pub fn metric_data_format(&self) -> i16 {
        self.metric_data_format
    }

    /// Number of hMetric entries in 'hmtx' table.
    pub fn number_of_hmetrics(&self) -> u16 {
        self.number_of_hmetrics
    }
}

impl_parse!(
    /// Parse Horizontal Header Table.
    ///
    /// # Example
    ///
    /// ```
    /// extern crate opentype_rs as otf;
    ///
    /// use otf::tables::hhea::HorizontalHeaderTable;
     /// use otf::parser::Parse;
    ///
    /// let bytes: &[u8]  = &[
    ///     0x00, 0x01, 0x00, 0x00, 0x07, 0x6C, 0xFE, 0x0C, 0x00, 0x00, 0x09, 0x49, 0xFA, 0x1B,
    ///     0xFE, 0x4A, 0x09, 0x30, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ///     0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x05, 0x0E];
    ///
    /// let horizontal_header_table = HorizontalHeaderTable::parse(bytes).unwrap().1;
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
    HorizontalHeaderTable, parse_horizontal_header_table
);

pub fn parse_horizontal_header_table(input: &[u8]) -> IResult<&[u8], HorizontalHeaderTable>
{
    let (input, _) = verify(be_u16, |major_version| *major_version == 1)(input)?;
    let (input, _) = verify(be_u16, |minor_version| *minor_version == 0)(input)?;
    let (input, ascender) = be_i16(input)?;
    let (input, descender) = be_i16(input)?;
    let (input, line_gap) = be_i16(input)?;
    let (input, advance_width_max) = be_u16(input)?;
    let (input, min_left_side_bearing) = be_i16(input)?;
    let (input, min_right_side_bearing) = be_i16(input)?;
    let (input, x_max_extent) = be_i16(input)?;
    let (input, caret_slope_rise) = be_i16(input)?;
    let (input, caret_slope_run) = be_i16(input)?;
    let (input, caret_offset) = be_i16(input)?;
    // reserved
    let (input, _) = take(8usize)(input)?;
    let (input, metric_data_format) = be_i16(input)?;
    let (input, number_of_hmetrics) = be_u16(input)?;

    Ok((input, HorizontalHeaderTable {
        ascender,
        descender,
        line_gap,
        advance_width_max,
        min_left_side_bearing,
        min_right_side_bearing,
        x_max_extent,
        caret_slope_rise,
        caret_slope_run,
        caret_offset,
        metric_data_format,
        number_of_hmetrics
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::Err;
    use nom::error::ErrorKind;

    #[test]
    fn case_head_invalid_empty_slice() {
        let bytes: &[u8] = &[];

        let expected = Err(Err::Error(error_position!(bytes, ErrorKind::Eof)));
        assert_eq!(parse_horizontal_header_table(bytes), expected);
    }
}
