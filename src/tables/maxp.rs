use error::Error;
use nom::{be_u16, be_i32};
use traits::{Parser, TableParser};

/// Maximum Profile Table
///
/// The 'maxp' table establishes the memory requirements for a font. It begins with a table
/// version number.
///
/// More information on ['maxp'](https://docs.microsoft.com/en-gb/typography/opentype/spec/maxp)
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct MaximumProfileTable {
    num_glyphs: u16,
    extension: Option<MaximumProfileTableExtension>
}

impl<'otf> MaximumProfileTable {
    /// The number of glyphs in the font
    pub fn num_glyphs(&self) -> u16 {
        self.num_glyphs
    }

    /// Version 1.0 extension
    pub fn extension(&self) -> Option<&MaximumProfileTableExtension> {
        self.extension.as_ref()
    }
}

impl<'otf> Parser<'otf> for MaximumProfileTable {
    type Item = MaximumProfileTable;

    /// Parse Maximum Profile Table.
    ///
    /// # Example
    ///
    /// Maximum Profile Table version 0.5
    /// ```
    /// extern crate opentype_rs as otf;
    ///
    /// use otf::tables::maxp::MaximumProfileTable;
    /// use otf::traits::Parser;
    ///
    /// let bytes: &[u8]  = &[
    ///     0x00, 0x00, 0x50, 0x00, 0x05, 0x0E];
    ///
    /// let maximum_profile_table = MaximumProfileTable::parse(bytes).unwrap();
    ///
    /// assert_eq!(maximum_profile_table.num_glyphs(), 1294);
    /// assert_eq!(maximum_profile_table.extension(), None);
    /// ```
    ///
    /// Maximum Profile Table version 1.0
    /// ```
    /// extern crate opentype_rs as otf;
    ///
    /// use otf::tables::maxp::MaximumProfileTable;
    /// use otf::traits::Parser;
    ///
    /// let bytes: &[u8]  = &[
    ///     0x00, 0x01, 0x00, 0x00, 0x05, 0x0E, 0x00, 0x8F, 0x00, 0x16, 0x00, 0x54, 0x00, 0x05,
    ///     0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0E, 0x00, 0x00, 0x02, 0x00, 0x02, 0x24,
    ///     0x00, 0x06, 0x00, 0x01];
    ///
    /// let maximum_profile_table = MaximumProfileTable::parse(bytes).unwrap();
    /// let maximum_profile_table_extension = maximum_profile_table.extension().unwrap();
    ///
    /// assert_eq!(maximum_profile_table.num_glyphs(), 1294);
    /// assert_eq!(maximum_profile_table_extension.max_points(), 143);
    /// assert_eq!(maximum_profile_table_extension.max_contours(), 22);
    /// assert_eq!(maximum_profile_table_extension.max_composite_points(), 84);
    /// assert_eq!(maximum_profile_table_extension.max_composite_contours(), 5);
    /// assert_eq!(maximum_profile_table_extension.max_zones(), 1);
    /// assert_eq!(maximum_profile_table_extension.max_twilight_points(), 0);
    /// assert_eq!(maximum_profile_table_extension.max_storage(), 0);
    /// assert_eq!(maximum_profile_table_extension.max_function_defs(), 14);
    /// assert_eq!(maximum_profile_table_extension.max_instruction_defs(), 0);
    /// assert_eq!(maximum_profile_table_extension.max_stack_elements(), 512);
    /// assert_eq!(maximum_profile_table_extension.max_size_of_instructions(), 548);
    /// assert_eq!(maximum_profile_table_extension.max_component_elements(), 6);
    /// assert_eq!(maximum_profile_table_extension.max_component_depth(), 1);
    /// ```
    fn parse(buf: &'otf[u8]) -> Result<Self::Item, Error> {
        Ok(parse_maximum_profile_table(buf)?.1)
    }
}

impl<'otf> TableParser<'otf> for MaximumProfileTable {}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct MaximumProfileTableExtension {
    max_points: u16,
    max_contours: u16,
    max_composite_points: u16,
    max_composite_contours: u16,
    max_zones: u16,
    max_twilight_points: u16,
    max_storage: u16,
    max_function_defs: u16,
    max_instruction_defs: u16,
    max_stack_elements: u16,
    max_size_of_instructions: u16,
    max_component_elements: u16,
    max_component_depth: u16
}

impl MaximumProfileTableExtension {
    /// Maximum points in a non-composite glyph
    pub fn max_points(&self) -> u16 {
        self.max_points
    }

    /// Maximum contours in a non-composite glyph
    pub fn max_contours(&self) -> u16 {
        self.max_contours
    }

    /// Maximum points in a composite glyph
    pub fn max_composite_points(&self) -> u16 {
        self.max_composite_points
    }

    /// Maximum contours in a composite glyph
    pub fn max_composite_contours(&self) -> u16 {
        self.max_composite_contours
    }

    /// 1 if instructions do not use the twilight zone (Z0), or 2 if instructions do use Z0;
    /// should be set to 2 in most cases.
    pub fn max_zones(&self) -> u16 {
        self.max_zones
    }

    /// Maximum points used in Z0
    pub fn max_twilight_points(&self) -> u16 {
        self.max_twilight_points
    }

    /// Number of Storage Area locations
    pub fn max_storage(&self) -> u16 {
        self.max_storage
    }

    /// Number of FDEFs, equal to the highest function number + 1
    pub fn max_function_defs(&self) -> u16 {
        self.max_function_defs
    }

    /// Number of IDEFs
    pub fn max_instruction_defs(&self) -> u16 {
        self.max_instruction_defs
    }

    /// Maximum stack depth across Font Program ('fpgm' table), CVT Program ('prep' table) and
    /// all glyph instructions (in the 'glyf' table).
    pub fn max_stack_elements(&self) -> u16 {
        self.max_stack_elements
    }

    /// Maximum byte count for glyph instructions.
    pub fn max_size_of_instructions(&self) -> u16 {
        self.max_size_of_instructions
    }

    /// Maximum number of components referenced at “top level” for any composite glyph
    pub fn max_component_elements(&self) -> u16 {
        self.max_component_elements
    }

    /// Maximum levels of recursion; 1 for simple components.
    pub fn max_component_depth(&self) -> u16 {
        self.max_component_depth
    }
}

named!(pub parse_maximum_profile_table<&[u8],MaximumProfileTable>,
    alt!(parse_maximum_profile_table_v0_5 | parse_maximum_profile_table_v1_0)
);

named!(parse_maximum_profile_table_v0_5<&[u8],MaximumProfileTable>,
    do_parse!(
        verify!(be_i32, |version| version == 0x00005000) >>
        num_glyphs: be_u16 >>
        (
            MaximumProfileTable {
                num_glyphs,
                extension: None
            }
        )
    )
);

named!(parse_maximum_profile_table_v1_0<&[u8],MaximumProfileTable>,
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
        ({

            MaximumProfileTable {
                num_glyphs,
                extension: Some(MaximumProfileTableExtension {
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

            }
        })
    )
);

#[cfg(test)]
mod tests {
    use super::*;
    use nom::{Err, Needed};

    #[test]
    fn case_maximum_profile_table_invalid_empty_slice() {
        let bytes: &[u8] = &[];

        let expected = Result::Err(Err::Incomplete(Needed::Size(4)));
        assert_eq!(parse_maximum_profile_table(bytes), expected);
    }
}
