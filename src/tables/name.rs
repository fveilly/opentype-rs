use parser;
use std::ops;
use error::Error;

pub use parser::tables::Platform;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NamingTable<'otf> {
    buf: &'otf[u8],
    table: parser::tables::NamingTable
}

impl<'otf> NamingTable<'otf> {
    /// Parse Naming Table.
    ///
    /// # Example
    ///
    /// Naming Table format 0
    /// ```
    /// extern crate opentype_rs as otf;
    ///
    /// use otf::{NamingTable, Platform, MacintoshEncoding, MacintoshLanguage, NameId};
    ///
    /// let bytes: &[u8]  = &[
    ///     0x00, 0x00, 0x00, 0x1A, 0x01, 0x3E, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ///     0x00, 0x2F, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x06,
    ///     0x00, 0x2F, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x07, 0x00, 0x35,
    ///     0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 0x00, 0x06, 0x00, 0x2F, 0x00, 0x01,
    ///     0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0x00, 0x06, 0x00, 0x2F, 0x00, 0x01, 0x00, 0x00,
    ///     0x00, 0x00, 0x00, 0x05, 0x00, 0x13, 0x00, 0x3C, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00,
    ///     0x00, 0x06, 0x00, 0x0E, 0x00, 0x4F, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x07,
    ///     0x00, 0x20, 0x00, 0x5D, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x09, 0x00, 0x06,
    ///     0x00, 0x7D, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0B, 0x00, 0x0A, 0x00, 0x83,
    ///     0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0C, 0x00, 0x13, 0x00, 0x8D, 0x00, 0x01,
    ///     0x00, 0x00, 0x00, 0x00, 0x00, 0x0D, 0x00, 0x2E, 0x00, 0xA0, 0x00, 0x01, 0x00, 0x00,
    ///     0x00, 0x00, 0x00, 0x0E, 0x00, 0x2A, 0x00, 0xCE, 0x00, 0x03, 0x00, 0x01, 0x04, 0x09,
    ///     0x00, 0x00, 0x00, 0x5E, 0x00, 0xF8, 0x00, 0x03, 0x00, 0x01, 0x04, 0x09, 0x00, 0x01,
    ///     0x00, 0x0C, 0x01, 0x56, 0x00, 0x03, 0x00, 0x01, 0x04, 0x09, 0x00, 0x02, 0x00, 0x0E,
    ///     0x01, 0x62, 0x00, 0x03, 0x00, 0x01, 0x04, 0x09, 0x00, 0x03, 0x00, 0x0C, 0x01, 0x56,
    ///     0x00, 0x03, 0x00, 0x01, 0x04, 0x09, 0x00, 0x04, 0x00, 0x0C, 0x01, 0x56, 0x00, 0x03,
    ///     0x00, 0x01, 0x04, 0x09, 0x00, 0x05, 0x00, 0x26, 0x01, 0x70, 0x00, 0x03, 0x00, 0x01,
    ///     0x04, 0x09, 0x00, 0x06, 0x00, 0x1C, 0x01, 0x96, 0x00, 0x03, 0x00, 0x01, 0x04, 0x09,
    ///     0x00, 0x07, 0x00, 0x40, 0x01, 0xB2, 0x00, 0x03, 0x00, 0x01, 0x04, 0x09, 0x00, 0x09,
    ///     0x00, 0x0C, 0x01, 0xF2, 0x00, 0x03, 0x00, 0x01, 0x04, 0x09, 0x00, 0x0B, 0x00, 0x14,
    ///     0x01, 0xFE, 0x00, 0x03, 0x00, 0x01, 0x04, 0x09, 0x00, 0x0C, 0x00, 0x26, 0x02, 0x12,
    ///     0x00, 0x03, 0x00, 0x01, 0x04, 0x09, 0x00, 0x0D, 0x00, 0x5C, 0x02, 0x38, 0x00, 0x03,
    ///     0x00, 0x01, 0x04, 0x09, 0x00, 0x0E, 0x00, 0x54, 0x02, 0x94];
    ///
    /// let naming_table = NamingTable::parse(bytes).unwrap();
    ///
    /// assert_eq!(naming_table.string_offset(), 318);
    /// assert_eq!(naming_table.name_records().len(), 26);
    /// assert!(naming_table.lang_tag_records().is_none());
    ///
    /// let first_name_record = naming_table.name_records().get(0).unwrap();
    ///
    /// match first_name_record.platform() {
    ///     Platform::Macintosh(encoding_id, language_id) => {
    ///         assert_eq!(encoding_id, MacintoshEncoding::Roman);
    ///         assert_eq!(language_id.unwrap(), MacintoshLanguage::English);
    ///     },
    ///     _ => assert!(false)
    /// }
    ///
    /// assert_eq!(first_name_record.name_id(), NameId::Copyright);
    /// assert_eq!(first_name_record.length(), 47);
    /// assert_eq!(first_name_record.offset(), 0);
    /// ```
    ///
    /// Naming Table format 1
    /// ```
    /// // TODO
    /// ```
    pub fn parse(buf: &'otf[u8]) -> Result<NamingTable, Error> {
        let res = parser::tables::parse_naming_table(buf)?;

        Ok(NamingTable {
            buf: res.0,
            table: res.1
        })
    }
}

impl<'otf> ops::Deref for NamingTable<'otf> {
    type Target = parser::tables::NamingTable;
    fn deref(&self) -> &Self::Target {
        &self.table
    }
}