use error::Error;
use nom::Err as NomErr;
use nom::IResult;
use nom::error::ErrorKind;
use nom::number::complete::{be_u16, be_u32};
use nom::multi::count;
use types::{Offset16, Offset32};

/// Index to Location
///
/// The indexToLoc table stores the offsets to the locations of the glyphs in the font, relative to
/// the beginning of the glyphData table. In order to compute the length of the last glyph element,
/// there is an extra entry after the last valid index.
///
/// By definition, index zero points to the “missing character”, which is the character that
/// appears if a character is not found in the font. The missing character is commonly represented
/// by a blank box or a space. If the font does not contain an outline for the missing character,
/// then the first and second offsets should have the same value. This also applies to any other
/// characters without an outline, such as the space character. If a glyph has no outline, then
/// loca[n] = loca [n+1]. In the particular case of the last glyph(s), loca[n] will be equal the
/// length of the glyph data ('glyf') table. The offsets must be in ascending order with
/// loca[n] <= loca[n+1].
///
/// Most routines will look at the 'maxp' table to determine the number of glyphs in the font, but
/// the value in the 'loca' table must agree.
///
/// There are two versions of this table: a short version, and a long version. The version is
/// specified in the indexToLocFormat entry in the 'head' table.
pub enum IndexToLocationTable {
    Short(Vec<Offset16>),
    Long(Vec<Offset32>)
}

impl<'otf> IndexToLocationTable {
    pub fn get_glyf_offset(&self, glyph_index: u32) -> Option<u32> {
        match self {
            IndexToLocationTable::Short(offsets) => offsets.get(glyph_index as usize).map(
                |offset| *offset as u32),
            IndexToLocationTable::Long(offsets) => offsets.get(glyph_index as usize).map(
                |offset| *offset)
        }
    }

    /// Parse Index to Location Table.
    ///
    /// * `index_to_loc_format` - The index to location table format is determined by the
    /// [indexToLocFormat](./Head.t.html#method.index_to_loc_format) field in the 'head' table.
    /// * `num_glyphs` - The number of glyphs in the font is determined by the
    /// [numGlyphs](./Maxp.t.html#method.num_glyphs) field in the 'maxp' table.
    ///
    /// # Example
    ///
    /// ```
    /// extern crate opentype_rs as otf;
    ///
    /// use otf::tables::loca::IndexToLocationTable;
    ///
    /// // TODO
    /// ```
    pub fn parse(buf: &'otf[u8], index_to_loc_format: i16, num_glyphs: u16) -> Result<IndexToLocationTable, Error> {
        Ok(parse_index_to_location_table(buf, index_to_loc_format, num_glyphs)?.1)
    }
}

pub fn parse_index_to_location_table(input: &[u8], index_to_loc_format: i16, num_glyphs: u16)
                                      -> IResult<&[u8], IndexToLocationTable> {
    match index_to_loc_format {
        // 0 for short offsets (Offset16)
        0 => {
            let (input, offsets) = count(be_u16, usize::from(num_glyphs) + 1)(input)?;
            Ok((input, IndexToLocationTable::Short(offsets)))
        },
        // 1 for long (Offset32)
        1 => {
            let (input, offsets) = count(be_u32, usize::from(num_glyphs) + 1)(input)?;
            Ok((input, IndexToLocationTable::Long(offsets)))
        },
        _ => Err(NomErr::Error(error_position!(input, ErrorKind::Alt)))
    }
}