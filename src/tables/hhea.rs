//! Horizontal Header Table
//!
//! The 'hhea' table contains information needed to layout fonts whose characters are written
//! horizontally, that is, either left to right or right to left.
//!
//! More information on ['hhea'](https://docs.microsoft.com/en-gb/typography/opentype/spec/hhea)

use nom::{be_i16, be_u16, be_i32, be_u32, be_i64};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Hhea {
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

impl Hhea {
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

named!(
    #[doc="
        Parse 'hhea' table.

        # Example

        ```
        extern crate opentype_rs as otf;

        use otf::tables::hhea::{Hhea, parse_hhea};

        let bytes: &[u8]  = &[
            0x00, 0x01, 0x00, 0x00, 0x07, 0x6C, 0xFE, 0x0C, 0x00, 0x00, 0x09, 0x49, 0xFA, 0x1B,
            0xFE, 0x4A, 0x09, 0x30, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x05, 0x0E];

        let hhea_table = parse_hhea(bytes).unwrap().1;

        assert_eq!(hhea_table.ascender(), 1900);
        assert_eq!(hhea_table.descender(), -500);
        assert_eq!(hhea_table.line_gap(), 0);
        assert_eq!(hhea_table.advance_width_max(), 2377);
        assert_eq!(hhea_table.min_left_side_bearing(), -1509);
        assert_eq!(hhea_table.min_right_side_bearing(), -438);
        assert_eq!(hhea_table.x_max_extent(), 2352);
        assert_eq!(hhea_table.caret_slope_rise(), 1);
        assert_eq!(hhea_table.caret_slope_run(), 0);
        assert_eq!(hhea_table.caret_offset(), 0);
        assert_eq!(hhea_table.metric_data_format(),  0);
        assert_eq!(hhea_table.number_of_hmetrics(), 1294);
        ```
    "],
    pub parse_hhea<&[u8],Hhea>,
    do_parse!(
        verify!(be_u16, |major_version| major_version == 1) >>
        verify!(be_u16, |minor_version| minor_version == 0) >>
        ascender: be_i16 >>
        descender: be_i16 >>
        line_gap: be_i16 >>
        advance_width_max: be_u16 >>
        min_left_side_bearing: be_i16 >>
        min_right_side_bearing: be_i16 >>
        x_max_extent: be_i16 >>
        caret_slope_rise: be_i16 >>
        caret_slope_run: be_i16 >>
        caret_offset: be_i16 >>
        take!(8) >> // reserved
        metric_data_format: be_i16 >>
        number_of_hmetrics: be_u16 >>
        (
            Hhea{
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
            }
        )
    )
);

#[cfg(test)]
mod tests {
    use super::*;
    use nom::{Err, ErrorKind, Context, Needed};

    #[test]
    fn case_head_invalid_empty_slice() {
        let bytes: &[u8] = &[];

        let expected = Result::Err(Err::Incomplete(Needed::Size(2)));
        assert_eq!(parse_hhea(bytes), expected);
    }
}
