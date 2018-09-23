use parser;
use std::ops;
use error::Error;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Os2<'otf> {
    buf: &'otf[u8],
    table: parser::tables::Os2
}

impl<'otf> Os2<'otf> {
    /// Parse OS/2 Table.
    ///
    /// # Example
    ///
    /// OS/2 Table version 0
    /// ```
    /// // TODO
    /// ```
    ///
    /// OS/2 Table version 1
    /// ```
    /// // TODO
    /// ```
    ///
    /// OS/2 Table version 3
    /// ```
    /// extern crate opentype_rs as otf;
    ///
    /// use otf::{Os2, Os2Version, FontSelectionFlags, CodePageRange, UnicodeRange, Panose};
    /// use otf::Tag;
    ///
    /// let bytes: &[u8]  = &[
    ///     0x00, 0x03, 0x04, 0x86, 0x01, 0x90, 0x00, 0x05, 0x00, 0x00, 0x05, 0x9A, 0x05, 0x33,
    ///     0x00, 0x00, 0x01, 0x1F, 0x05, 0x9A, 0x05, 0x33, 0x00, 0x00, 0x03, 0xD1, 0x00, 0x66,
    ///     0x02, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ///     0xE0, 0x00, 0x02, 0xFF, 0x50, 0x00, 0x20, 0x5B, 0x00, 0x00, 0x00, 0x20, 0x00, 0x00,
    ///     0x00, 0x00, 0x47, 0x4F, 0x4F, 0x47, 0x00, 0x40, 0x00, 0x00, 0xFF, 0xFD, 0x06, 0x00,
    ///     0xFE, 0x00, 0x00, 0x66, 0x07, 0x9A, 0x02, 0x00, 0x20, 0x00, 0x01, 0x9F, 0x00, 0x00,
    ///     0x00, 0x00, 0x04, 0x3A, 0x05, 0xB0, 0x00, 0x20, 0x00, 0x20, 0x00, 0x03];
    ///
    /// let os2_table = Os2::parse(bytes).unwrap();
    ///
    /// match os2_table.version() {
    ///     Os2Version::Version3(os2) => {
    ///         assert_eq!(os2.x_avg_char_width(), 1158);
    ///         assert_eq!(os2.us_weight_class(), 400);
    ///         assert_eq!(os2.us_width_class(), 5);
    ///         assert_eq!(os2.fs_type(), 0);
    ///         assert_eq!(os2.y_subscript_xsize(), 1434);
    ///         assert_eq!(os2.y_subscript_ysize(), 1331);
    ///         assert_eq!(os2.y_subscript_xoffset(), 0);
    ///         assert_eq!(os2.y_subscript_yoffset(), 287);
    ///         assert_eq!(os2.y_superscript_xsize(), 1434);
    ///         assert_eq!(os2.y_superscript_ysize(), 1331);
    ///         assert_eq!(os2.y_superscript_xoffset(), 0);
    ///         assert_eq!(os2.y_superscript_yoffset(), 977);
    ///         assert_eq!(os2.y_strikeout_size(), 102);
    ///         assert_eq!(os2.y_strikeout_position(), 512);
    ///         assert_eq!(os2.s_family_class(), 0);
    ///         assert_eq!(os2.panose(), &Panose::new(&[2, 0, 0, 0, 0, 0, 0, 0, 0, 0]));
    ///         assert_eq!(os2.ul_unicode_range(), UnicodeRange::new(3758097151, 1342185563, 32, 0));
    ///         assert_eq!(os2.ach_vend_id(), Tag::new(b"GOOG"));
    ///         assert_eq!(os2.fs_selection(), FontSelectionFlags::OBLIQUE);
    ///         assert_eq!(os2.us_first_char_index(), 0);
    ///         assert_eq!(os2.us_last_char_index(), 65533);
    ///         assert_eq!(os2.s_typo_ascender(), 1536);
    ///         assert_eq!(os2.s_typo_descender(), -512);
    ///         assert_eq!(os2.s_typo_line_gap(), 102);
    ///         assert_eq!(os2.us_win_ascent(), 1946);
    ///         assert_eq!(os2.us_win_descent(), 512);
    ///         assert_eq!(os2.ul_code_page_range(), CodePageRange::new(536871327, 0));
    ///         assert_eq!(os2.sx_height(), 1082);
    ///         assert_eq!(os2.s_cap_height(), 1456);
    ///         assert_eq!(os2.us_default_char(), 32);
    ///         assert_eq!(os2.us_break_char(), 32);
    ///         assert_eq!(os2.us_max_context(), 3);
    ///     },
    ///     _ => assert!(false)
    /// }
    /// ```
    ///
    /// OS/2 Table version 5
    /// ```
    /// // TODO
    /// ```
    pub fn parse(buf: &'otf[u8]) -> Result<Os2, Error> {
        let res = parser::tables::parse_os2(buf)?;

        Ok(Os2 {
            buf: res.0,
            table: res.1
        })
    }
}

impl<'otf> ops::Deref for Os2<'otf> {
    type Target = parser::tables::Os2;
    fn deref(&self) -> &Self::Target {
        &self.table
    }
}