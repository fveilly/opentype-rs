//! Maximum Profile Table
//!
//! https://docs.microsoft.com/en-gb/typography/opentype/spec/maxp
//! https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6maxp.html

use nom::{be_i16, be_u16, be_i32, be_u32, be_i64};
use types::{Fixed, LongDateTime, Rect};

/// The 'maxp' table establishes the memory requirements for a font. It begins with a table
/// version number.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Maxp {
    Simple(MaxpSimple),
    Extended(MaxpExtended)
}

impl Maxp {
    pub fn num_glyphs(&self) -> u16 {
        match self {
            Maxp::Simple(maxp) => maxp.num_glyphs(),
            Maxp::Extended(maxp) => maxp.num_glyphs()
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct MaxpSimple {
    /// The number of glyphs in the font
    num_glyphs: u16
}

impl MaxpSimple {
    pub fn num_glyphs(&self) -> u16 {
        self.num_glyphs
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct MaxpExtended {
    /// The number of glyphs in the font
    num_glyphs: u16,

    /// Maximum points in a non-composite glyph
    max_points: u16,

    /// Maximum contours in a non-composite glyph
    max_contours: u16,

    /// Maximum points in a composite glyph
    max_composite_points: u16,

    /// Maximum contours in a composite glyph
    max_composite_contours: u16,

    /// 1 if instructions do not use the twilight zone (Z0), or 2 if instructions do use Z0;
    /// should be set to 2 in most cases.
    max_zones: u16,

    /// Maximum points used in Z0
    max_twilight_points: u16,

    /// Number of Storage Area locations
    max_storage: u16,

    /// Number of FDEFs, equal to the highest function number + 1
    max_function_defs: u16,

    /// Number of IDEFs
    max_instruction_defs: u16,

    /// Maximum stack depth across Font Program ('fpgm' table), CVT Program ('prep' table) and
    /// all glyph instructions (in the 'glyf' table).
    max_stack_elements: u16,

    /// Maximum byte count for glyph instructions.
    max_size_of_instructions: u16,

    /// Maximum number of components referenced at “top level” for any composite glyph
    max_component_elements: u16,

    /// Maximum levels of recursion; 1 for simple components.
    max_component_depth: u16
}

impl MaxpExtended {
    pub fn num_glyphs(&self) -> u16 {
        self.num_glyphs
    }

    pub fn max_points(&self) -> u16 {
        self.max_points
    }

    pub fn max_contours(&self) -> u16 {
        self.max_contours
    }

    pub fn max_composite_points(&self) -> u16 {
        self.max_composite_points
    }

    pub fn max_composite_contours(&self) -> u16 {
        self.max_composite_contours
    }

    pub fn max_zones(&self) -> u16 {
        self.max_zones
    }

    pub fn max_twilight_points(&self) -> u16 {
        self.max_twilight_points
    }

    pub fn max_storage(&self) -> u16 {
        self.max_storage
    }

    pub fn max_function_defs(&self) -> u16 {
        self.max_function_defs
    }

    pub fn max_instruction_defs(&self) -> u16 {
        self.max_instruction_defs
    }

    pub fn max_stack_elements(&self) -> u16 {
        self.max_stack_elements
    }

    pub fn max_size_of_instructions(&self) -> u16 {
        self.max_size_of_instructions
    }

    pub fn max_component_elements(&self) -> u16 {
        self.max_component_elements
    }

    pub fn max_component_depth(&self) -> u16 {
        self.max_component_depth
    }
}

named!(pub parse_maxp<&[u8],Maxp>,
    alt!(parse_maxp_simple | parse_maxp_extended)
);

named!(parse_maxp_simple<&[u8],Maxp>,
    do_parse!(
        verify!(be_i32, |version| version == 0x00005000) >>
        num_glyphs: be_u16 >>
        (
            Maxp::Simple(MaxpSimple {
                num_glyphs
            })
        )
    )
);

named!(parse_maxp_extended<&[u8],Maxp>,
    do_parse!(
        verify!(be_i32, |version| version == 0x00010000) >>
        num_glyphs: be_u16 >>
        max_points: be_u16 >>
        max_contours: be_u16 >>
        max_composite_points: be_u16 >>
        max_composite_contours: be_u16 >>
        max_zones: be_u16 >>
        max_twilight_points: be_u16 >>
        max_storage: be_u16 >>
        max_function_defs: be_u16 >>
        max_instruction_defs: be_u16 >>
        max_stack_elements: be_u16 >>
        max_size_of_instructions: be_u16 >>
        max_component_elements: be_u16 >>
        max_component_depth: be_u16 >>
        (
            Maxp::Extended(MaxpExtended {
                num_glyphs,
                max_points,
                max_contours,
                max_composite_points,
                max_composite_contours,
                max_zones,
                max_twilight_points,
                max_storage,
                max_function_defs,
                max_instruction_defs,
                max_stack_elements,
                max_size_of_instructions,
                max_component_elements,
                max_component_depth
            })
        )
    )
);

#[cfg(test)]
mod tests {
    use super::*;
    use nom::{Err, ErrorKind, Context, Needed};

    #[test]
    fn case_maxp_simple() {
        let bytes: &[u8] = &[
            0x00, 0x00, 0x50, 0x00, 0x05, 0x0E
        ];

        let expected = Maxp::Simple(MaxpSimple {
            num_glyphs: 1294
        });

        let res = parse_maxp(bytes).unwrap();
        assert_eq!(res.1, expected);
    }

    #[test]
    fn case_maxp_extended() {
        let bytes: &[u8] = &[
            0x00, 0x01, 0x00, 0x00, 0x05, 0x0E, 0x00, 0x8F, 0x00, 0x16, 0x00, 0x54, 0x00, 0x05,
            0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0E, 0x00, 0x00, 0x02, 0x00, 0x02, 0x24,
            0x00, 0x06, 0x00, 0x01
        ];

        let expected = Maxp::Extended(MaxpExtended {
            num_glyphs: 1294,
            max_points: 143,
            max_contours: 22,
            max_composite_points: 84,
            max_composite_contours: 5,
            max_zones: 1,
            max_twilight_points: 0,
            max_storage: 0,
            max_function_defs: 14,
            max_instruction_defs: 0,
            max_stack_elements: 512,
            max_size_of_instructions: 548,
            max_component_elements: 6,
            max_component_depth: 1
        });

        let res = parse_maxp(bytes).unwrap();
        assert_eq!(res.1, expected);
    }

    #[test]
    fn case_maxp_invalid_empty_slice() {
        let bytes: &[u8] = &[];

        let expected = Result::Err(Err::Incomplete(Needed::Size(4)));
        assert_eq!(parse_maxp(bytes), expected);
    }
}
