use parser;
use std::ops;
use error::Error;
use traits::{Parser, TableParser};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MaximumProfileTable<'otf> {
    buf: &'otf[u8],
    table: parser::tables::MaximumProfileTable
}

impl<'otf> Parser<'otf> for MaximumProfileTable<'otf> {
    type Item = MaximumProfileTable<'otf>;

    /// Parse Maximum Profile Table.
    ///
    /// # Example
    ///
    /// Maximum Profile Table version 0.5
    /// ```
    /// extern crate opentype_rs as otf;
    ///
    /// use otf::MaximumProfileTable;
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
    /// use otf::MaximumProfileTable;
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
        let res = parser::tables::parse_maximum_profile_table(buf)?;

        Ok(MaximumProfileTable {
            buf: res.0,
            table: res.1
        })
    }
}

impl<'otf> TableParser<'otf> for MaximumProfileTable<'otf> {}

impl<'otf> ops::Deref for MaximumProfileTable<'otf> {
    type Target = parser::tables::MaximumProfileTable;
    fn deref(&self) -> &Self::Target {
        &self.table
    }
}