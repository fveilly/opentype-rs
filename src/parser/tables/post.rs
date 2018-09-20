use nom::{be_i16, be_u16, be_i32, be_u32};

/// PostScript Table
///
/// This table contains additional information needed to use TrueType or OpenType™ fonts on
/// PostScript printers. This includes data for the FontInfo dictionary entry and the PostScript
/// names of all the glyphs. For more information about PostScript names, see the [Adobe GlyphList Specification](https://github.com/adobe-type-tools/agl-specification).
///
/// Versions 1.0, 2.0, and 2.5 refer to TrueType fonts and OpenType fonts with TrueType data.
/// OpenType fonts with TrueType data may also use Version 3.0. OpenType fonts with CFF data use
/// Version 3.0 only.
///
/// More information on ['hmtx'](https://docs.microsoft.com/en-gb/typography/opentype/spec/post)
#[derive(Debug, Clone, Eq, PartialEq)]
#[allow(non_camel_case_types)]
pub enum PostScriptTable {
    /// This version is used in order to supply PostScript glyph names when the font file contains
    /// exactly the 258 glyphs in the standard Macintosh TrueType font file (see 'post' Format 1 in
    /// Apple’s specification for a list of the 258 Macintosh glyph names), and the font does not
    /// otherwise supply glyph names. As a result, the glyph names are taken from the system with
    /// no storage required by the font.
    Version_1_0(PostScriptTableHeader),

    /// This is the version required in order to supply PostScript glyph names for fonts which do
    /// not supply them elsewhere. A version 2.0 'post' table can be used in fonts with TrueType
    /// or CFF version 2 outlines.
    Version_2_0(PostScriptTableV20),

    /// This version of the 'post' table has been deprecated as of OpenType Specification v1.3.
    ///
    /// This version provides a space-saving table for TrueType-based fonts which contain a pure
    /// subset of, or a simple reordering of, the standard Macintosh glyph set.
    #[deprecated]
    Version_2_5(PostScriptTableHeader),

    /// This version makes it possible to create a font that is not burdened with a large 'post'
    /// table set of glyph names. A version 3.0 'post' table can be used by OpenType fonts with
    /// TrueType or CFF (version 1 or 2) data.
    ///
    /// This version specifies that no PostScript name information is provided for the glyphs in
    /// this font file. The printing behavior of this version on PostScript printers is
    /// unspecified, except that it should not result in a fatal or unrecoverable error. Some
    /// drivers may print nothing; other drivers may attempt to print using a default naming scheme.
    ///
    /// Windows makes use of the italic angle value in the 'post' table but does not actually
    /// require any glyph names to be stored as Pascal strings.
    Version_3_0(PostScriptTableHeader),

    /// Composite fonts on Japanese, Chinese or Korean printers work only with character codes.
    /// AAT printer drivers only know about glyph index values. The TrueType scaler uses format 4
    /// 'post' table to reencode a font that maps to a composite font on a printer. This encoding
    /// consists of naming the glyphs by using their character codes. The driver has PostScript
    /// code that knows how to take this ASCII string, strip the leading "a," and convert the rest
    /// to hexadecimal. The resulting hexadecimal number is the character code of the glyph. In
    /// this manner, the composite fonts on the printer are used.
    ///
    /// Any font that maps to a composite font on the printer needs to include a format 4 'post'
    /// table. The structure of a format 4 'post' table is as follows: the 'post' table header is
    /// followed by an array of uint16 values. An entry for every glyph is required. The index into
    /// the array is the glyph index. The data in the array is the character code that maps to
    /// that glyph, or 0xFFFF if there is no associated character code for that glyph.
    ///
    /// As a rule, format 4 'post' tables are no longer necessary and should be avoided.
    /// Source: [https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6post.html](https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6post.html)
    #[deprecated]
    Version_4_0(PostScriptTableHeader)
}

impl PostScriptTable {
    /// See [italic_angle](PostScriptTableHeader.t.html#method.italic_angle).
    pub fn italic_angle(&self) -> i32 {
        match self {
            PostScriptTable::Version_1_0(header) => header.italic_angle(),
            PostScriptTable::Version_2_0(post_script_table_v2) => post_script_table_v2.italic_angle(),
            PostScriptTable::Version_2_5(header) => header.italic_angle(),
            PostScriptTable::Version_3_0(header) => header.italic_angle(),
            PostScriptTable::Version_4_0(header) => header.italic_angle(),
        }
    }

    /// See [underline_position](PostScriptTableHeader.t.html#method.underline_position).
    pub fn underline_position(&self) -> i16 {
        match self {
            PostScriptTable::Version_1_0(header) => header.underline_position(),
            PostScriptTable::Version_2_0(post_script_table_v2) => post_script_table_v2.underline_position(),
            PostScriptTable::Version_2_5(header) => header.underline_position(),
            PostScriptTable::Version_3_0(header) => header.underline_position(),
            PostScriptTable::Version_4_0(header) => header.underline_position(),
        }
    }

    /// See [underline_thickness](PostScriptTableHeader.t.html#method.underline_thickness).
    pub fn underline_thickness(&self) -> i16 {
        match self {
            PostScriptTable::Version_1_0(header) => header.underline_thickness(),
            PostScriptTable::Version_2_0(post_script_table_v2) => post_script_table_v2.underline_thickness(),
            PostScriptTable::Version_2_5(header) => header.underline_thickness(),
            PostScriptTable::Version_3_0(header) => header.underline_thickness(),
            PostScriptTable::Version_4_0(header) => header.underline_thickness(),
        }
    }

    /// See [is_fixed_pitch](PostScriptTableHeader.t.html#method.is_fixed_pitch).
    pub fn is_fixed_pitch(&self) -> u32 {
        match self {
            PostScriptTable::Version_1_0(header) => header.is_fixed_pitch(),
            PostScriptTable::Version_2_0(post_script_table_v2) => post_script_table_v2.is_fixed_pitch(),
            PostScriptTable::Version_2_5(header) => header.is_fixed_pitch(),
            PostScriptTable::Version_3_0(header) => header.is_fixed_pitch(),
            PostScriptTable::Version_4_0(header) => header.is_fixed_pitch(),
        }
    }

    /// See [min_mem_type_42](PostScriptTableHeader.t.html#method.min_mem_type_42).
    pub fn min_mem_type_42(&self) -> u32 {
        match self {
            PostScriptTable::Version_1_0(header) => header.min_mem_type_42(),
            PostScriptTable::Version_2_0(post_script_table_v2) => post_script_table_v2.min_mem_type_42(),
            PostScriptTable::Version_2_5(header) => header.min_mem_type_42(),
            PostScriptTable::Version_3_0(header) => header.min_mem_type_42(),
            PostScriptTable::Version_4_0(header) => header.min_mem_type_42(),
        }
    }

    /// See [max_mem_type_42](PostScriptTableHeader.t.html#method.max_mem_type_42).
    pub fn max_mem_type_42(&self) -> u32 {
        match self {
            PostScriptTable::Version_1_0(header) => header.max_mem_type_42(),
            PostScriptTable::Version_2_0(post_script_table_v2) => post_script_table_v2.max_mem_type_42(),
            PostScriptTable::Version_2_5(header) => header.max_mem_type_42(),
            PostScriptTable::Version_3_0(header) => header.max_mem_type_42(),
            PostScriptTable::Version_4_0(header) => header.max_mem_type_42(),
        }
    }

    /// See [min_mem_type_1](PostScriptTableHeader.t.html#method.min_mem_type_1).
    pub fn min_mem_type_1(&self) -> u32 {
        match self {
            PostScriptTable::Version_1_0(header) => header.min_mem_type_1(),
            PostScriptTable::Version_2_0(post_script_table_v2) => post_script_table_v2.min_mem_type_1(),
            PostScriptTable::Version_2_5(header) => header.min_mem_type_1(),
            PostScriptTable::Version_3_0(header) => header.min_mem_type_1(),
            PostScriptTable::Version_4_0(header) => header.min_mem_type_1(),
        }
    }

    /// See [max_mem_type_1](PostScriptTableHeader.t.html#method.max_mem_type_1).
    pub fn max_mem_type_1(&self) -> u32 {
        match self {
            PostScriptTable::Version_1_0(header) => header.max_mem_type_1(),
            PostScriptTable::Version_2_0(post_script_table_v2) => post_script_table_v2.max_mem_type_1(),
            PostScriptTable::Version_2_5(header) => header.max_mem_type_1(),
            PostScriptTable::Version_3_0(header) => header.max_mem_type_1(),
            PostScriptTable::Version_4_0(header) => header.max_mem_type_1(),
        }
    }
}

/// The last four entries in the table are present because PostScript drivers can do better memory
/// management if the virtual memory (VM) requirements of a downloadable OpenType font are known
/// before the font is downloaded. This information should be supplied if known. If it is not
/// known, set the value to zero. The driver will still work but will be less efficient.
///
/// Maximum memory usage is minimum memory usage plus maximum runtime memory use. Maximum runtime
/// memory use depends on the maximum band size of any bitmap potentially rasterized by the font
/// scaler. Runtime memory usage could be calculated by rendering characters at different point
/// sizes and comparing memory use.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct PostScriptTableHeader {
    italic_angle: i32,
    underline_position: i16,
    underline_thickness: i16,
    is_fixed_pitch: u32,
    min_mem_type_42: u32,
    max_mem_type_42: u32,
    min_mem_type_1: u32,
    max_mem_type_1: u32
}

impl PostScriptTableHeader {
    /// Italic angle in counter-clockwise degrees from the vertical. Zero for upright text,
    /// negative for text that leans to the right (forward).
    #[inline]
    pub fn italic_angle(&self) -> i32 {
        self.italic_angle
    }

    /// This is the suggested distance of the top of the underline from the baseline (negative
    /// values indicate below baseline).
    ///
    /// The PostScript definition of this FontInfo dictionary key (the y coordinate of the center
    /// of the stroke) is not used for historical reasons. The value of the PostScript key may be
    /// calculated by subtracting half the underlineThickness from the value of this field.
    #[inline]
    pub fn underline_position(&self) -> i16 {
        self.underline_position
    }

    /// Suggested values for the underline thickness. In general, the underline thickness should
    /// match the thickness of the underscore character (U+005F LOW LINE), and should also match
    /// the strikeout thickness, which is specified in the OS/2 table.
    #[inline]
    pub fn underline_thickness(&self) -> i16 {
        self.underline_thickness
    }

    /// Set to 0 if the font is proportionally spaced, non-zero if the font is not proportionally
    /// spaced (i.e. monospaced).
    #[inline]
    pub fn is_fixed_pitch(&self) -> u32 {
        self.is_fixed_pitch
    }

    /// Minimum memory usage when an OpenType font is downloaded.
    #[inline]
    pub fn min_mem_type_42(&self) -> u32 {
        self.min_mem_type_42
    }

    /// Maximum memory usage when an OpenType font is downloaded.
    #[inline]
    pub fn max_mem_type_42(&self) -> u32 {
        self.max_mem_type_42
    }

    /// Minimum memory usage when an OpenType font is downloaded as a Type 1 font.
    #[inline]
    pub fn min_mem_type_1(&self) -> u32 {
        self.min_mem_type_1
    }

    /// Maximum memory usage when an OpenType font is downloaded as a Type 1 font.
    #[inline]
    pub fn max_mem_type_1(&self) -> u32 {
        self.max_mem_type_1
    }
}

/// This font file contains glyphs not in the standard Macintosh set, or the ordering of the
/// glyphs in the font file differs from the standard Macintosh set. The glyph name array maps the
/// glyphs in this font to name index. If the name index is between 0 and 257, treat the name index
/// as a glyph index in the Macintosh standard order. If the name index is between 258 and 65535,
/// then subtract 258 and use that to index into the list of Pascal strings at the end of the
/// table. Thus a given font may map some of its glyphs to the standard glyph names, and some to
/// its own names.
///
/// If you do not want to associate a PostScript name with a particular glyph, use index number 0
/// which points to the name .notdef.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PostScriptTableV20 {
    header: PostScriptTableHeader,
    num_glyphs: u16,
    glyph_name_index: Vec<u16>
}

impl PostScriptTableV20 {
    /// See [italic_angle](PostScriptTableHeader.t.html#method.italic_angle).
    #[inline]
    pub fn italic_angle(&self) -> i32 {
        self.header.italic_angle
    }

    /// See [underline_position](PostScriptTableHeader.t.html#method.underline_position).
    #[inline]
    pub fn underline_position(&self) -> i16 {
        self.header.underline_position
    }

    /// See [underline_thickness](PostScriptTableHeader.t.html#method.underline_thickness).
    #[inline]
    pub fn underline_thickness(&self) -> i16 {
        self.header.underline_thickness
    }

    /// See [is_fixed_pitch](PostScriptTableHeader.t.html#method.is_fixed_pitch).
    #[inline]
    pub fn is_fixed_pitch(&self) -> u32 {
        self.header.is_fixed_pitch
    }

    /// See [min_mem_type_42](PostScriptTableHeader.t.html#method.min_mem_type_42).
    #[inline]
    pub fn min_mem_type_42(&self) -> u32 {
        self.header.min_mem_type_42
    }

    /// See [max_mem_type_42](PostScriptTableHeader.t.html#method.max_mem_type_42).
    #[inline]
    pub fn max_mem_type_42(&self) -> u32 {
        self.header.max_mem_type_42
    }

    /// See [min_mem_type_1](PostScriptTableHeader.t.html#method.min_mem_type_1).
    #[inline]
    pub fn min_mem_type_1(&self) -> u32 {
        self.header.min_mem_type_1
    }

    /// See [max_mem_type_1](PostScriptTableHeader.t.html#method.max_mem_type_1).
    #[inline]
    pub fn max_mem_type_1(&self) -> u32 {
        self.header.max_mem_type_1
    }

    /// Number of glyphs (this should be the same as numGlyphs in 'maxp' table).
    pub fn num_glyphs(&self) -> u16 {
        self.num_glyphs
    }

    /// This is not an offset, but is the ordinal number of the glyph in 'post' string tables.
    pub fn glyph_name_index(&self) -> &[u16] {
        &self.glyph_name_index
    }
}

named!(
    #[doc="
        Parse Post Script Table.

        # Example

        Post Script Table version 1
        ```
        extern crate opentype_rs as otf;

        use otf::parser::tables::{PostScriptTable, parse_post_script_table};
        use otf::Tag;

        let bytes: &[u8]  = &[
            0x00, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0x6A, 0x00, 0x64, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00];

        let post_script_table = parse_post_script_table(bytes).unwrap().1;

        match post_script_table {
            PostScriptTable::Version_3_0(header) => {
                assert_eq!(header.italic_angle(), 0);
                assert_eq!(header.underline_position(), -150);
                assert_eq!(header.underline_thickness(), 100);
                assert_eq!(header.is_fixed_pitch(), 0);
                assert_eq!(header.min_mem_type_42(), 0);
                assert_eq!(header.max_mem_type_42(), 0);
                assert_eq!(header.min_mem_type_1(), 0);
                assert_eq!(header.max_mem_type_1(), 0);
            },
            _ => assert!(false)
        }
        ```

        Post Script Table version 2
        ```
        // TODO
        ```
    "],
    pub parse_post_script_table<&[u8],PostScriptTable>,
    switch!(be_i32,
     	0x00010000 => map!(parse_post_script_header, |header| PostScriptTable::Version_1_0(header)) |
        0x00020000 => map!(parse_post_script_table_v2_0, |post_script_v2_0| PostScriptTable::Version_2_0(post_script_v2_0)) |
     	0x00025000 => map!(parse_post_script_header, |header| PostScriptTable::Version_2_5(header)) |
     	0x00030000 => map!(parse_post_script_header, |header| PostScriptTable::Version_3_0(header)) |
     	0x00040000 => map!(parse_post_script_header, |header| PostScriptTable::Version_4_0(header))
    )
);

named!(parse_post_script_header<&[u8],PostScriptTableHeader>,
    do_parse!(
        italic_angle: be_i32 >>
        underline_position: be_i16 >>
        underline_thickness: be_i16 >>
        is_fixed_pitch: be_u32 >>
        min_mem_type_42: be_u32 >>
        max_mem_type_42: be_u32 >>
        min_mem_type_1: be_u32 >>
        max_mem_type_1: be_u32 >>
        (
            PostScriptTableHeader {
                italic_angle,
                underline_position,
                underline_thickness,
                is_fixed_pitch,
                min_mem_type_42,
                max_mem_type_42,
                min_mem_type_1,
                max_mem_type_1
            }
        )
    )
);

named!(parse_post_script_table_v2_0<&[u8],PostScriptTableV20>,
    do_parse!(
        header: parse_post_script_header >>
        num_glyphs: be_u16 >>
        glyph_name_index: count!(be_u16, usize::from(num_glyphs)) >>
        (
            PostScriptTableV20 {
                header,
                num_glyphs,
                glyph_name_index
            }
        )
    )
);

#[cfg(test)]
mod tests {
    use super::*;
    use nom::{Err, ErrorKind, Context, Needed};

    #[test]
    fn case_post_script_table_invalid_empty_slice() {
        let bytes: &[u8] = &[];

        let expected = Result::Err(Err::Incomplete(Needed::Size(4)));
        assert_eq!(parse_post_script_table(bytes), expected);
    }
}