use nom::{be_i16, be_u16, be_i32, be_u32, be_i64};
use types::{Fixed, LongDateTime, Rect};

/// OS/2 and Windows Metrics Table
///
/// The OS/2 table consists of a set of metrics and other data that are required in OpenType fonts.
///
/// More information on ['OS/2'](https://docs.microsoft.com/en-gb/typography/opentype/spec/os2)
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Os2 {
    Version0,
    Version1,
    Version2,
    Version3,
    Version4,
    Version5
}

impl Os2 {
    pub fn num_glyphs(&self) -> u16 {
        match self {
            Maxp::Simple(maxp) => maxp.num_glyphs(),
            Maxp::Extended(maxp) => maxp.num_glyphs()
        }
    }
}

pub struct Os2V0 {
    x_avg_char_width: i16,
    us_weight_class: u16,
    us_width_class: u16,
    fs_type: u16,
    y_subscript_xsize: i16,
    y_subscript_ysize: i16,
    y_subscript_xoffset: i16,
    y_subscript_yoffset: i16,
    y_superscript_xsize: i16,
    y_superscript_ysize: i16,
    y_superscript_xoffset: i16,
    y_superscript_yoffset: i16,
    y_strikeout_size: i16,
    y_strikeout_position: i16,
    s_family_class: i16,
    panose: [u8; 10],
    ul_unicode_range1: u32,
    ul_unicode_range2: u32,
    ul_unicode_range3: u32,
    ul_unicode_range4: u32,
    ach_vend_id: Tag,
    fs_selection: u16,
    us_first_char_index: u16,
    us_last_char_index: u16,
    s_typo_ascender: i16,
    s_typo_descender: i16,
    s_typo_line_gap: i16,
    us_win_ascent: u16,
    us_win_descent: u16
}