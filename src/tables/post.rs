use parser;
use std::ops;
use error::Error;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PostScriptTable<'otf> {
    buf: &'otf[u8],
    table: parser::tables::PostScriptTable
}

impl<'otf> PostScriptTable<'otf> {
    /// Parse Post Script Table.
    ///
    /// # Example
    ///
    /// Post Script Table version 2
    /// ```
    /// // TODO
    /// ```
    ///
    /// Post Script Table version 3
    /// ```
    /// extern crate opentype_rs as otf;
    ///
    /// use otf::{PostScriptTable, PostScriptVersion};
    ///
    /// let bytes: &[u8]  = &[
    ///     0x00, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0x6A, 0x00, 0x64, 0x00, 0x00,
    ///     0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ///     0x00, 0x00, 0x00, 0x00];
    ///
    /// let post_script_table = PostScriptTable::parse(bytes).unwrap();
    ///
    /// assert_eq!(post_script_table.italic_angle(), 0);
    /// assert_eq!(post_script_table.underline_position(), -150);
    /// assert_eq!(post_script_table.underline_thickness(), 100);
    /// assert_eq!(post_script_table.is_fixed_pitch(), 0);
    /// assert_eq!(post_script_table.min_mem_type_42(), 0);
    /// assert_eq!(post_script_table.max_mem_type_42(), 0);
    /// assert_eq!(post_script_table.min_mem_type_1(), 0);
    /// assert_eq!(post_script_table.max_mem_type_1(), 0);
    ///
    /// match post_script_table.version() {
    ///     PostScriptVersion::Version_3_0(header) => {
    ///         assert_eq!(header.italic_angle(), 0);
    ///         assert_eq!(header.underline_position(), -150);
    ///         assert_eq!(header.underline_thickness(), 100);
    ///         assert_eq!(header.is_fixed_pitch(), 0);
    ///         assert_eq!(header.min_mem_type_42(), 0);
    ///         assert_eq!(header.max_mem_type_42(), 0);
    ///         assert_eq!(header.min_mem_type_1(), 0);
    ///         assert_eq!(header.max_mem_type_1(), 0);
    ///     },
    ///     _ => assert!(false)
    /// }
    /// ```
    pub fn parse(buf: &'otf[u8]) -> Result<PostScriptTable, Error> {
        let res = parser::tables::parse_post_script_table(buf)?;

        Ok(PostScriptTable {
            buf: res.0,
            table: res.1
        })
    }
}

impl<'otf> ops::Deref for PostScriptTable<'otf> {
    type Target = parser::tables::PostScriptTable;
    fn deref(&self) -> &Self::Target {
        &self.table
    }
}