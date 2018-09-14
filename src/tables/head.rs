//! Font Header Table
//!
//! https://docs.microsoft.com/en-gb/typography/opentype/spec/head
//! https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6head.html

use nom::{be_i16, be_u16, be_i32, be_u32, be_i64};
use types::{Fixed, LongDateTime, Rect};

/// The 'head' table contains global information about the font. It records such facts as the font
/// version number, the creation and modification dates, revision number and basic typographic data
/// that applies to the font as a whole.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Head {
    /// Set by font manufacturer
    font_revision: Fixed,

    /// To compute: set it to 0, calculate the checksum for the 'head' table and put it in the
    /// table directory, sum the entire font as a uint32_t, then store 0xB1B0AFBA - sum
    check_sum_adjustment: u32,

    /// bit 0 - y value of 0 specifies baseline
    /// bit 1 - x position of left most black bit is LSB
    /// bit 2 - scaled point size and actual point size will differ (i.e. 24 point glyph differs
    ///         from 12 point glyph scaled by factor of 2)
    /// bit 3 - use integer scaling instead of fractional
    /// bit 4 - (used by the Microsoft implementation of the TrueType scaler)
    /// bit 5 - This bit should be set in fonts that are intended to e laid out vertically, and in
    ///         which the glyphs have been drawn such that an x-coordinate of 0 corresponds to the
    ///         desired vertical baseline.
    /// bit 6 - This bit must be set to zero.
    /// bit 7 - This bit should be set if the font requires layout for correct linguistic rendering
    ///         (e.g. Arabic fonts).
    /// bit 8 - This bit should be set for an AAT font which has one or more metamorphosis effects
    ///         designated as happening by default.
    /// bit 9 - This bit should be set if the font contains any strong right-to-left glyphs.
    /// bit 10 - This bit should be set if the font contains Indic-style rearrangement effects.
    /// bits 11-13 - Defined by Adobe.
    /// bit 14 - This bit should be set if the glyphs in the font are simply generic symbols for
    ///          code point ranges, such as for a last resort font.
    flags: u16,

    /// Range from 64 to 16384
    units_per_em: u16,

    /// Number of seconds since 12:00 midnight that started January 1st 1904 in GMT/UTC time zone
    created: LongDateTime,

    /// Number of seconds since 12:00 midnight that started January 1st 1904 in GMT/UTC time zone
    modified: LongDateTime,

    /// For all glyph bounding boxes
    x_min: i16,

    /// For all glyph bounding boxes
    y_min: i16,

    /// For all glyph bounding boxes
    x_max: i16,

    /// For all glyph bounding boxes
    y_max: i16,

    /// bit 0 bold
    /// bit 1 italic
    /// bit 2 underline
    /// bit 3 outline
    /// bit 4 shadow
    /// bit 5 condensed (narrow)
    /// bit 6 extended
    mac_style: u16,

    /// Smallest readable size in pixels
    lowest_rec_ppem: u16,

    /// Deprecated (Set to 2).
    /// 0: Fully mixed directional glyphs;
    /// 1: Only strongly left to right;
    /// 2: Like 1 but also contains neutrals;
    /// -1: Only strongly right to left;
    /// -2: Like -1 but also contains neutrals.
    font_direction_hint: i16,

    /// 0 for short offsets (Offset16), 1 for long (Offset32)
    index_to_loc_format: i16,

    /// 0 for current format
    glyph_data_format: i16
}

impl Head {
    pub fn font_revision(&self) -> Fixed {
        self.font_revision
    }

    pub fn check_sum_adjustment(&self) -> u32 {
        self.check_sum_adjustment
    }

    pub fn flags(&self) -> u16 {
        self.flags
    }

    pub fn units_per_em(&self) -> u16 {
        self.units_per_em
    }

    pub fn created(&self) -> LongDateTime {
        self.created
    }

    pub fn modified(&self) -> LongDateTime {
        self.modified
    }

    pub fn bounding_box(&self) -> Rect<i16> {
        Rect::new(self.x_min, self.y_min, self.x_max, self.y_max)
    }

    pub fn mac_style(&self) -> u16 {
        self.mac_style
    }

    pub fn lowest_rec_ppem(&self) -> u16 {
        self.lowest_rec_ppem
    }

    pub fn font_direction_hint(&self) -> i16 {
        self.font_direction_hint
    }

    pub fn index_to_loc_format(&self) -> i16 {
        self.index_to_loc_format
    }

    pub fn glyph_data_format(&self) -> i16 {
        self.glyph_data_format
    }
}

named!(pub parse_head<&[u8],Head>,
    do_parse!(
        verify!(be_u16, |major_version| major_version == 1) >>
        verify!(be_u16, |minor_version| minor_version == 0) >>
        font_revision: be_i32 >>
        check_sum_adjustment: be_u32 >>
        verify!(be_u32, |magic_number| magic_number == 0x5F0F3CF5) >>
        flags: be_u16 >>
        units_per_em: be_u16 >>
        created: be_i64 >>
        modified: be_i64 >>
        x_min: be_i16 >>
        y_min: be_i16 >>
        x_max: be_i16 >>
        y_max: be_i16 >>
        mac_style: be_u16 >>
        lowest_rec_ppem: be_u16 >>
        font_direction_hint: be_i16 >>
        index_to_loc_format: be_i16 >>
        glyph_data_format: be_i16 >>
        (
            Head{
                font_revision,
                check_sum_adjustment,
                flags,
                units_per_em,
                created,
                modified,
                x_min,
                y_min,
                x_max,
                y_max,
                mac_style,
                lowest_rec_ppem,
                font_direction_hint,
                index_to_loc_format,
                glyph_data_format
            }
        )
    )
);

#[cfg(test)]
mod tests {
    use super::*;
    use nom::{Err, ErrorKind, Context, Needed};

    #[test]
    fn case_head() {
        let bytes: &[u8]  = &[
            0x00, 0x01, 0x00, 0x00, 0x00, 0x02, 0x23, 0x12, 0x8A, 0x7F, 0x70, 0x48, 0x5F, 0x0F,
            0x3C, 0xF5, 0x00, 0x19, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0xC4, 0xF0, 0x11, 0x2E,
            0x00, 0x00, 0x00, 0x00, 0xD5, 0x01, 0x52, 0xF4, 0xFA, 0x1B, 0xFD, 0xD5, 0x09, 0x30,
            0x08, 0x73, 0x00, 0x00, 0x00, 0x09, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00
        ];

        let expected = Head {
            font_revision: 140050,
            check_sum_adjustment: 2323607624,
            flags: 25,
            units_per_em: 2048,
            created: 3304067374,
            modified: 3573633780,
            x_min: -1509,
            y_min: -555,
            x_max: 2352,
            y_max: 2163,
            mac_style: 0,
            lowest_rec_ppem: 9,
            font_direction_hint: 2,
            index_to_loc_format: 0,
            glyph_data_format: 0
        };

        let res = parse_head(bytes).unwrap();
        assert_eq!(res.1,  expected);
    }

    #[test]
    fn case_head_invalid_empty_slice() {
        let bytes: &[u8]  = &[];

        let expected = Result::Err(Err::Incomplete(Needed::Size(2)));
        assert_eq!(parse_head(bytes),  expected);
    }

    #[test]
    fn case_head_invalid_magic_number() {
        let bytes: &[u8]  = &[
            0x00, 0x01, 0x00, 0x00, 0x00, 0x02, 0x23, 0x12, 0x8A, 0x7F, 0x70, 0x48, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x19, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0xC4, 0xF0, 0x11, 0x2E,
            0x00, 0x00, 0x00, 0x00, 0xD5, 0x01, 0x52, 0xF4, 0xFA, 0x1B, 0xFD, 0xD5, 0x09, 0x30,
            0x08, 0x73, 0x00, 0x00, 0x00, 0x09, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00
        ];

        let expected = Result::Err(Err::Error(Context::Code(&bytes[12..], ErrorKind::Verify)));
        assert_eq!(parse_head(bytes),  expected);
    }
}