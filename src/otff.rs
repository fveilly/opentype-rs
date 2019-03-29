use error::Error;
use font::Font;
use offset_table::{OffsetTable, parse_offset_table};
use ttc_header::{TTCHeader, parse_ttc_header};

/// An OpenType font file contains data, in table format, that comprises either a TrueType or a
/// Compact Font Format (CFF) outline font. Rasterizers use combinations of data from the tables
/// contained in the font to render the TrueType or PostScript glyph outlines. Some of this
/// supporting data is used no matter which outline format is used; some of the supporting data is
/// specific to either TrueType or PostScript.
///
/// More information on ['ottf'](https://docs.microsoft.com/en-gb/typography/opentype/spec/otff)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum OpenTypeFontKind {
    Font(OffsetTable),

    FontCollection(TTCHeader)
}

pub struct OpenTypeFontFile<'otf> {
    buf: &'otf[u8],
    remainder: &'otf[u8],
    kind: OpenTypeFontKind
}

impl<'otf> OpenTypeFontFile<'otf> {
    /// Parse the OpenType font file header and return an iterator over the fonts.
    ///
    /// ```
    /// extern crate opentype_rs as otf;
    ///
    /// use otf::OpenTypeFontFile;
    ///
    /// let buf = include_bytes!("../fonts/Roboto/Roboto-Regular.ttf") as &[u8];
    /// let otff = OpenTypeFontFile::parse(buf).unwrap();
    ///
    /// for font in otff {
    ///     // Do something here
    /// }
    /// ```
    pub fn parse(buf: &'otf[u8]) -> Result<OpenTypeFontFile, Error> {
        let res = parse_otff(buf)?;

        Ok(OpenTypeFontFile {
            buf,
            remainder: res.0,
            kind: res.1
        })
    }
}

pub struct OpenTypeFontFileIterator<'otf> {
    otff: OpenTypeFontFile<'otf>,
    pos: usize
}

impl<'otf> IntoIterator for OpenTypeFontFile<'otf> {
    type Item = Font<'otf>;
    type IntoIter = OpenTypeFontFileIterator<'otf>;

    fn into_iter(self) -> Self::IntoIter {
        OpenTypeFontFileIterator {
            otff: self,
            pos: 0
        }
    }
}

impl<'otf> Iterator for OpenTypeFontFileIterator<'otf> {
    type Item = Font<'otf>;

    fn next(&mut self) -> Option<Font<'otf>> {
        match &self.otff.kind {
            OpenTypeFontKind::Font(offset_table) => {
                if self.pos > 0 {
                    None
                }
                else {
                    self.pos =  self.pos + 1;
                    Some(Font::new(self.otff.buf, self.otff.remainder, *offset_table))
                }
            },
            OpenTypeFontKind::FontCollection(_ttc_header) => {
                // TODO
                None
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match &self.otff.kind {
            OpenTypeFontKind::Font(_) => {
                (1, Some(1))
            },
            OpenTypeFontKind::FontCollection(ttc_header) => {
                let num_fonts = ttc_header.num_fonts() as usize;
                (num_fonts, Some(num_fonts))
            }
        }
    }
}

named!(pub parse_otff<&[u8],OpenTypeFontKind>,
    alt!(
        map!(parse_offset_table, |offset_table| OpenTypeFontKind::Font(offset_table)) |
        map!(parse_ttc_header, |ttc_header| OpenTypeFontKind::FontCollection(ttc_header))
    )
);

#[cfg(test)]
mod tests {
    use super::*;
    use offset_table::SfntVersion;

    #[test]
    fn case_open_type_font_file() {
        let bytes: &[u8]  = &[
            0x00, 0x01, 0x00, 0x00, 0x00, 0x12, 0x01, 0x00, 0x00, 0x04, 0x00, 0x20];

        let kind = parse_otff(bytes).unwrap().1;

        match kind {
            OpenTypeFontKind::Font(offset_table) => {
                assert_eq!(offset_table.sfnt_version(), SfntVersion::TrueType);
                assert_eq!(offset_table.num_tables(), 18);
                assert_eq!(offset_table.search_range(), 256);
                assert_eq!(offset_table.entry_selector(), 4);
                assert_eq!(offset_table.range_shift(), 32);
            },
            _ => assert!(false)
        }
    }
}
