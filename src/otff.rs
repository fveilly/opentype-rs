use font::Font;
use parser::{
    OffsetTable,
    OpenTypeFontKind,
    TTCHeader,
    parse_otff,
    parse_table_records
};
use error::Error;

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

                    match parse_table_records(self.otff.remainder, offset_table.num_tables()) {
                        Ok((_, table_records)) => Some(Font::new(
                            self.otff.buf, offset_table.sfnt_version(), table_records)),
                        _ => None
                    }
                }
            },
            OpenTypeFontKind::FontCollection(ttc_header) => {
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
