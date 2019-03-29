use error::Error;
use nom::{be_u8, be_i16, be_u16, be_u24, be_u32};
use tables::name::Platform;
use std::collections::HashMap;
use traits::{Parser, TableParser};
use super::GlyphId;

/// This table defines mapping of character codes to a default glyph index. Different subtables may
/// be defined that each contain mappings for different character encoding schemes. The table
/// header indicates the character encodings for which subtables are present.
///
/// Regardless of the encoding scheme, character codes that do not correspond to any glyph in the
/// font should be mapped to glyph index 0. The glyph at this location must be a special glyph
/// representing a missing character, commonly known as .notdef.
///
/// Each subtable is in one of seven possible formats and begins with a format field indicating the
/// format used. The first four formats — formats 0, 2, 4 and 6 — were originally defined prior to
/// Unicode 2.0. These formats allow for 8-bit single-byte, 8-bit multi-byte, and 16-bit encodings.
/// With the introduction of supplementary planes in Unicode 2.0, the Unicode addressable code
/// space extends beyond 16 bits. To accommodate this, three additional formats were added —
/// formats 8, 10 and 12 — that allow for 32-bit encoding schemes.
///
/// Other enhancements in Unicode led to the addition of other subtable formats. Subtable format
/// 13 allows for an efficient mapping of many characters to a single glyph; this is useful for
/// “last-resort” fonts that provide fallback rendering for all possible Unicode characters with a
/// distinct fallback glyph for different Unicode ranges. Subtable format 14 provides a unified
/// mechanism for supporting Unicode variation sequences.
///
/// Note: The 'cmap' table version number remains at 0x0000 for fonts that make use of the newer
/// subtable formats.
///
/// More information on ['cmap'](https://docs.microsoft.com/en-gb/typography/opentype/spec/cmap)
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CharacterGlyphIndexMappingTable {
    num_tables: u16
}

impl<'otf> CharacterGlyphIndexMappingTable {
    pub fn num_tables(&self) -> u16 {
        self.num_tables
    }
}

impl<'otf> Parser<'otf> for CharacterGlyphIndexMappingTable {
    type Item = EncodingRecords<'otf>;

    /// Parse Character to Glyph Index Mapping Table.
    ///
    /// # Example
    ///
    /// ```
    /// // TODO
    /// ```
    fn parse(buf: &'otf[u8]) -> Result<Self::Item, Error> {
        let res = parse_character_glyph_index_mapping_table(buf)?;

        Ok(EncodingRecords {
            buf: res.0,
            table: res.1
        })
    }
}

impl<'otf> TableParser<'otf> for CharacterGlyphIndexMappingTable {}

pub struct EncodingRecords<'otf> {
    buf: &'otf[u8],
    table: CharacterGlyphIndexMappingTable
}

impl<'otf> EncodingRecords<'otf> {
    pub fn iter(&self) -> EncodingRecordsIterator<'otf> {
        EncodingRecordsIterator {
            buf: self.buf,
            num_tables: self.table.num_tables(),
            pos: 0
        }
    }
}

pub struct EncodingRecordsIterator<'otf> {
    buf: &'otf[u8],
    num_tables: u16,
    pos: u16
}

impl<'otf> Iterator for EncodingRecordsIterator<'otf> {
    type Item = EncodingRecord<'otf>;

    fn next(&mut self) -> Option<EncodingRecord<'otf>> {
        if self.pos >= self.num_tables {
            return None;
        }

        match parse_encoding_record(self.buf) {
            Ok((bytes, encoding_record)) => {
                self.buf = bytes;
                self.pos = self.pos + 1;
                Some(encoding_record)
            },
            _ => None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = usize::from(self.num_tables);
        (size, Some(size))
    }
}


/// The array of encoding records specifies particular encodings and the offset to the subtable
/// for each encoding.
///
/// The platform ID and platform-specific encoding ID in the encoding record are used to specify a
/// particular character encoding. In the case of the Macintosh platform, a language field within
/// the mapping subtable is also used for this purpose.
///
/// The encoding record entries in the 'cmap' header must be sorted first by platform ID,
/// then by platform-specific encoding ID, and then by the language field in the corresponding
/// subtable. Each platform ID, platform-specific encoding ID, and subtable language
/// combination may appear only once in the 'cmap' table.
///
/// Complete details on platform IDs and platform-specific encoding and language IDs are provided
/// in the 'name' table chapter. Some specific details applicable to the 'cmap' table are
/// provided here.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct EncodingRecord<'otf> {
    platform: Platform,
    character_to_glyph_index_mapping_subtable: CharacterGlyphIndexMappingSubtable<'otf>
}

impl<'otf> EncodingRecord<'otf> {
    /// The platform, encoding and language IDs.
    pub fn platform(&self) -> Platform {
        self.platform
    }

    /// Subtable for this encoding.
    pub fn character_to_glyph_index_mapping_subtable(&self) -> &CharacterGlyphIndexMappingSubtable<'otf> {
        &self.character_to_glyph_index_mapping_subtable
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
#[allow(non_camel_case_types)]
pub enum CharacterGlyphIndexMappingSubtable<'otf> {
    Format_0(CharacterGlyphIndexMappingSubtable0<'otf>),
    Format_2(CharacterGlyphIndexMappingSubtable2),
    Format_4(CharacterGlyphIndexMappingSubtable4<'otf>),
    Format_6(CharacterGlyphIndexMappingSubtable6<'otf>),
    Format_8(CharacterGlyphIndexMappingSubtable8<'otf>),
    Format_10(CharacterGlyphIndexMappingSubtable10<'otf>),
    Format_12(CharacterGlyphIndexMappingSubtable12),
    Format_13(CharacterGlyphIndexMappingSubtable13),
    Format_14(CharacterGlyphIndexMappingSubtable14)
}

impl<'otf> CharacterGlyphIndexMappingSubtable<'otf> {
    pub fn language(&self) -> u16 {
        match self {
            CharacterGlyphIndexMappingSubtable::Format_0(subtable) => subtable.language(),
            CharacterGlyphIndexMappingSubtable::Format_2(subtable) => subtable.language(),
            CharacterGlyphIndexMappingSubtable::Format_4(subtable) => subtable.language(),
            CharacterGlyphIndexMappingSubtable::Format_6(subtable) => subtable.language(),
            CharacterGlyphIndexMappingSubtable::Format_8(subtable) => subtable.language(),
            CharacterGlyphIndexMappingSubtable::Format_10(subtable) => subtable.language(),
            CharacterGlyphIndexMappingSubtable::Format_12(subtable) => subtable.language(),
            CharacterGlyphIndexMappingSubtable::Format_13(subtable) => subtable.language(),
            CharacterGlyphIndexMappingSubtable::Format_14(_subtable) => 0
        }
    }

    pub fn get_glyph_id(&self, character_code: u32) -> Option<GlyphId> {
        match self {
            CharacterGlyphIndexMappingSubtable::Format_0(subtable) => {
                if character_code > u32::from(u8::max_value()) {
                    return None;
                }

                Some(subtable.get_glyph_id(character_code as u8))
            },
            CharacterGlyphIndexMappingSubtable::Format_2(_subtable) => None,
            CharacterGlyphIndexMappingSubtable::Format_4(_subtable) => None,
            CharacterGlyphIndexMappingSubtable::Format_6(subtable) => {
                if character_code > u32::from(u16::max_value()) {
                    return None;
                }

                subtable.get_glyph_id(character_code as u16)
            },
            CharacterGlyphIndexMappingSubtable::Format_8(_subtable) => None,
            CharacterGlyphIndexMappingSubtable::Format_10(_subtable) => None,
            CharacterGlyphIndexMappingSubtable::Format_12(_subtable) => None,
            CharacterGlyphIndexMappingSubtable::Format_13(_subtable) => None,
            CharacterGlyphIndexMappingSubtable::Format_14(_subtable) => None
        }
    }

    pub fn mapping(&self) -> HashMap<u32, GlyphId> {
        match self {
            CharacterGlyphIndexMappingSubtable::Format_0(subtable) => subtable.mapping(),
            CharacterGlyphIndexMappingSubtable::Format_2(subtable) => subtable.mapping(),
            CharacterGlyphIndexMappingSubtable::Format_4(subtable) => subtable.mapping(),
            CharacterGlyphIndexMappingSubtable::Format_6(subtable) => subtable.mapping(),
            CharacterGlyphIndexMappingSubtable::Format_8(subtable) => subtable.mapping(),
            CharacterGlyphIndexMappingSubtable::Format_10(subtable) => subtable.mapping(),
            CharacterGlyphIndexMappingSubtable::Format_12(subtable) => subtable.mapping(),
            CharacterGlyphIndexMappingSubtable::Format_13(subtable) => subtable.mapping(),
            CharacterGlyphIndexMappingSubtable::Format_14(subtable) => subtable.mapping(),
        }
    }
}

/// This is the Apple standard character to glyph index mapping table.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CharacterGlyphIndexMappingSubtable0<'otf> {
    language: u16,
    glyph_id_array: &'otf[u8]
}

impl<'otf> CharacterGlyphIndexMappingSubtable0<'otf> {
    /// For requirements on use of the language field.
    pub fn language(&self) -> u16 {
        self.language
    }

    /// An array that maps character codes to glyph index values.
    pub fn glyph_id_array(&self) -> &'otf[u8] {
        &self.glyph_id_array
    }

    pub fn get_glyph_id(&self, character_code: u8) -> GlyphId {
        debug_assert!(self.glyph_id_array.len() == 256);
        GlyphId::from(self.glyph_id_array[usize::from(character_code)])
    }

    pub fn mapping(&self) -> HashMap<u32, GlyphId> {
        let mut mapping = HashMap::new();
        for (i, glyph_id) in self.glyph_id_array.iter().enumerate() {
            mapping.insert(i as u32, GlyphId::from(*glyph_id));
        }
        mapping
    }
}

/// This subtable is useful for the national character code standards used for Japanese, Chinese,
/// and Korean characters. These code standards use a mixed 8-/16-bit encoding, in which certain
/// byte values signal the first byte of a 2-byte character (but these values are also legal as
/// the second byte of a 2-byte character).
///
/// In addition, even for the 2-byte characters, the mapping of character codes to glyph index
/// values depends heavily on the first byte. Consequently, the table begins with an array that
/// maps the first byte to a SubHeader record. For 2-byte character codes, the SubHeader is used
/// to map the second byte’s value through a subArray, as described below. When processing mixed
/// 8-/16-bit text, SubHeader 0 is special: it is used for single-byte character codes. When
/// SubHeader 0 is used, a second byte is not needed; the single byte value is mapped through
/// the subArray.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CharacterGlyphIndexMappingSubtable2 {
    language: u16,
    sub_header_keys: Vec<u16>
    // TODO: subHeaders and glyphIndexArray
}

impl CharacterGlyphIndexMappingSubtable2 {
    /// For requirements on use of the language field.
    pub fn language(&self) -> u16 {
        self.language
    }

    /// Array that maps high bytes to subHeaders: value is subHeader index × 8.
    pub fn sub_header_keys(&self) -> &[u16] {
        &self.sub_header_keys
    }

    pub fn mapping(&self) -> HashMap<u32, GlyphId> {
        unimplemented!()
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CharacterGlyphIndexMappingSubtable2SubHeaderRecord {
    first_code: u16,
    entry_count: u16,
    id_delta: i16,
    id_range_offset: u16
}

/// The firstCode and entryCount values specify a subrange that begins at firstCode and has a
/// length equal to the value of entryCount. This subrange stays within the 0-255 range of the
/// byte being mapped. Bytes outside of this subrange are mapped to glyph index 0 (missing glyph).
/// The offset of the byte within this subrange is then used as index into a corresponding subarray
/// of glyphIndexArray. This subarray is also of length entryCount. The value of the idRangeOffset
/// is the number of bytes past the actual location of the idRangeOffset word where the
/// glyphIndexArray element corresponding to firstCode appears.
///
/// Finally, if the value obtained from the subarray is not 0 (which indicates the missing glyph),
/// you should add idDelta to it in order to get the glyphIndex. The value idDelta permits the same
/// subarray to be used for several different subheaders. The idDelta arithmetic is modulo 65536.
impl CharacterGlyphIndexMappingSubtable2SubHeaderRecord {
    /// First valid low byte for this SubHeader.
    pub fn first_code(&self) -> u16 {
        self.first_code
    }

    /// Number of valid low bytes for this SubHeader.
    pub fn entry_count(&self) -> u16 {
        self.entry_count
    }

    /// Id delta.
    pub fn id_delta(&self) -> i16 {
        self.id_delta
    }

    /// The value of the idRangeOffset is the number of bytes past the actual location of the
    /// idRangeOffset word where the glyphIndexArray element corresponding to firstCode appears.
    pub fn id_range_offset(&self) -> u16 {
        self.id_range_offset
    }
}

/// This is the standard character-to-glyph-index mapping table for the Windows platform for fonts
/// that support Unicode BMP characters.
///
/// This format is used when the character codes for the characters represented by a font fall into
/// several contiguous ranges, possibly with holes in some or all of the ranges (that is, some of
/// the codes in a range may not have a representation in the font). The format-dependent data is
/// divided into three parts, which must occur in the following order:
/// - A four-word header gives parameters for an optimized search of the segment list;
/// - Four parallel arrays describe the segments (one segment for each contiguous range of codes);
/// - A variable-length array of glyph IDs (unsigned words).
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CharacterGlyphIndexMappingSubtable4<'otf> {
    language: u16,
    seg_count: u16,
    search_range: u16,
    entry_selector: u16,
    range_shift: u16,
    end_code: Vec<u16>,
    start_code: Vec<u16>,
    id_delta: Vec<i16>,
    id_range_offset: Vec<u16>,
    glyph_id_array: &'otf[u8]
}

impl<'otf> CharacterGlyphIndexMappingSubtable4<'otf> {
    /// For requirements on use of the language field.
    pub fn language(&self) -> u16 {
        self.language
    }

    /// The number of segments.
    pub fn seg_count(&self) -> u16 {
        self.seg_count
    }

    /// 2 × (2**floor(log2(segCount)))
    pub fn search_range(&self) -> u16 {
        self.search_range
    }

    /// Log2(searchRange / 2)
    pub fn entry_selector(&self) -> u16 {
        self.entry_selector
    }

    /// 2 × segCount - searchRange
    pub fn range_shift(&self) -> u16 {
        self.range_shift
    }

    /// End characterCode for each segment, last=0xFFFF.
    pub fn end_code(&self) -> &[u16] {
        &self.end_code
    }

    /// Start character code for each segment.
    pub fn start_code(&self) -> &[u16] {
        &self.start_code
    }

    /// Delta for all character codes in segment.
    pub fn id_delta(&self) -> &[i16] {
        &self.id_delta
    }

    /// Offsets into glyphIdArray or 0.
    pub fn id_range_offset(&self) -> &[u16] {
        &self.id_range_offset
    }

    pub fn get_glyph_id(&self, _character_code: u16) -> Option<GlyphId> {
        None
    }

    pub fn mapping(&self) -> HashMap<u32, GlyphId> {
        let mut mapping = HashMap::new();
        for (i, tuple) in self.start_code.iter()
            .zip(self.end_code.iter())
            .zip(self.id_delta.iter())
            .zip(self.id_range_offset.iter())
            .enumerate() {
            let (((&start_code, &end_code), &id_delta), &id_range_offset) = tuple;

            for j in start_code..(end_code + 1) {
                let glyph_id = if id_range_offset > 0 {
                    let offset = (id_range_offset / 2 + (j - start_code)) - (self.seg_count - i as u16) as u16;
                    match self.glyph_id_array.get(offset as usize) {
                        Some(&glyph_id) => glyph_id as u16,
                        _ => continue
                    }
                } else {
                    id_delta.wrapping_add(j as i16) as u16
                };
                mapping.insert(j as u32, glyph_id);
            }
        }
        mapping
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CharacterGlyphIndexMappingSubtable6<'otf> {
    language: u16,
    first_code: u16,
    entry_count: u16,
    glyph_id_array: &'otf[u8]
}

impl<'otf> CharacterGlyphIndexMappingSubtable6<'otf> {
    /// For requirements on use of the language field.
    pub fn language(&self) -> u16 {
        self.language
    }

    /// First character code of subrange.
    pub fn first_code(&self) -> u16 {
        self.first_code
    }

    /// Number of character codes in subrange.
    pub fn entry_count(&self) -> u16 {
        self.entry_count
    }

    /// Array of glyph index values for character codes in the range.
    pub fn glyph_id_array(&self) -> &'otf[u8] {
        &self.glyph_id_array
    }

    pub fn get_glyph_id(&self, character_code: u16) -> Option<GlyphId> {
        if character_code < self.first_code || character_code >= self.first_code + self.entry_count {
            return None;
        }

        let res = do_parse!(self.glyph_id_array,
            take!((character_code - self.first_code) * 2) >>
            glyph_id: be_u16 >>
            (
                glyph_id
            )
        );

        match res {
            Ok((_, glyph_id)) => Some(glyph_id),
            _ => None
        }
    }

    pub fn mapping(&self) -> HashMap<u32, GlyphId> {
        let mut mapping = HashMap::new();
        for (i, glyph_id) in self.glyph_id_array.iter().enumerate() {
            mapping.insert(self.first_code as u32 + i as u32, GlyphId::from(*glyph_id));
        }
        mapping
    }
}

/// Format 8 is similar to format 2, in that it provides for mixed-length character codes. Instead
/// of allowing for 8- and 16-bit character codes, however, it allows for 16- and 32-bit character
/// codes.
///
/// If a font contains Unicode supplementary-plane characters (U+10000 to U+10FFFF), then it’s
/// likely that it will also include Unicode BMP characters (U+0000 to U+FFFF) as well. Hence,
/// there is a need to map a mixture of 16-bit and 32-bit character codes. A simplifying assumption
/// is made: namely, that there are no 32-bit character codes which share the same first 16 bits as
/// any 16-bit character code. (Since the Unicode code space extends only to U+1FFFFF, a potential
/// conflict exists only for characters U+0000 to U+001F, which are non-printing control
/// characters.) This means that the determination as to whether a particular 16-bit value is a
/// standalone character code or the start of a 32-bit character code can be made by looking at the
/// 16-bit value directly, with no further information required.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CharacterGlyphIndexMappingSubtable8<'otf> {
    language: u16,
    is32: &'otf[u8],
    groups: Vec<SequentialMapGroup>
}

impl<'otf> CharacterGlyphIndexMappingSubtable8<'otf> {
    /// For requirements on use of the language field.
    pub fn language(&self) -> u16 {
        self.language
    }

    /// Indicate whether the particular 16-bit (index) value is the start of a 32-bit
    /// character code.
    pub fn is32(&self, index: usize) -> u8 {
        self.is32[index]
    }

    /// Array of SequentialMapGroup records.
    pub fn groups(&self) -> &Vec<SequentialMapGroup> {
        &self.groups
    }

    pub fn mapping(&self) -> HashMap<u32, GlyphId> {
        unimplemented!()
    }
}

/// Each sequential map group record specifies a character range and the starting glyph ID mapped
/// from the first character.
///
/// A few notes here. The endCharCode is used, rather than a count, because comparisons for group
/// matching are usually done on an existing character code, and having the endCharCode be there
/// explicitly saves the necessity of an addition per group. Groups must be sorted by increasing
/// startCharCode. A group’s endCharCode must be less than the startCharCode of the following
/// group, if any.
///
/// To determine if a particular word (cp) is the first half of 32-bit code points, one can use an
/// expression such as ( is32[ cp / 8 ] & ( 1 << ( 7 - ( cp % 8 ) ) ) ). If this is non-zero, then
/// the word is the first half of a 32-bit code point.
///
/// 0 is not a special value for the high word of a 32-bit code point. A font may not have both a
/// glyph for the code point 0x0000 and glyphs for code points with a high word of 0x0000.
///
/// The presence of the packed array of bits indicating whether a particular 16-bit value is the
/// start of a 32-bit character code is useful even when the font contains no glyphs for a
/// particular 16-bit start value. This is because the system software often needs to know how
/// many bytes ahead the next character begins, even if the current character maps to the missing
/// glyph. By including this information explicitly in this table, no “secret” knowledge needs to
/// be encoded into the OS.
///
/// Although this format was created to support Unicode supplementary-plane characters, it is not
/// widely supported or used. Also, no character encoding other than Unicode uses mixed 16-/32-bit
/// characters. The use of this format is discouraged.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SequentialMapGroup {
    start_char_code: u32,
    end_char_code: u32,
    start_glyph_id: u32
}

impl SequentialMapGroup {
    /// First character code in this group; note that if this group is for one or more 16-bit
    /// character codes (which is determined from the is32 array), this 32-bit value will have the
    /// high 16-bits set to zero.
    pub fn start_char_code(&self) -> u32 {
        self.start_char_code
    }

    /// Last character code in this group; same condition as listed above for the startCharCode.
    pub fn end_char_code(&self) -> u32 {
        self.end_char_code
    }

    /// Glyph index corresponding to the starting character code.
    pub fn start_glyph_id(&self) -> u32 {
        self.start_glyph_id
    }
}

/// The constant map group record has the same structure as the sequential map group record, with
/// start and end character codes and a mapped glyph ID. However, the same glyph ID applies to all
/// characters in the specified range rather than sequential glyph IDs.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ConstantMapGroup {
    start_char_code: u32,
    end_char_code: u32,
    glyph_id: u32
}

impl ConstantMapGroup {
    /// First character code in this group.
    pub fn start_char_code(&self) -> u32 {
        self.start_char_code
    }

    /// Last character code in this group.
    pub fn end_char_code(&self) -> u32 {
        self.end_char_code
    }

    /// Glyph index to be used for all the characters in the group’s range.
    pub fn glyph_id(&self) -> u32 {
        self.glyph_id
    }
}

/// Format 10 is similar to format 6, in that it defines a trimmed array for a tight range of
/// character codes. It differs, however, in that is uses 32-bit character codes.
///
/// This format is not widely used and is not supported by Microsoft. It would be most suitable
/// for fonts that support only a contiguous range of Unicode supplementary-plane characters,
/// but such fonts are rare.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CharacterGlyphIndexMappingSubtable10<'otf> {
    language: u16,
    start_char_code: u32,
    glyphs: &'otf[u8]
}

impl<'otf> CharacterGlyphIndexMappingSubtable10<'otf> {
    /// For requirements on use of the language field.
    pub fn language(&self) -> u16 {
        self.language
    }

    /// First character code covered.
    pub fn start_char_code(&self) -> u32 {
        self.start_char_code
    }

    /// Array of glyph indices for the character codes covered.
    pub fn glyphs(&self) -> &'otf[u8] {
        self.glyphs
    }

    pub fn mapping(&self) -> HashMap<u32, GlyphId> {
        let mut mapping = HashMap::new();
        for (i, glyph_id) in self.glyphs.iter().enumerate() {
            mapping.insert(self.start_char_code as u32 + i as u32, GlyphId::from(*glyph_id));
        }
        mapping
    }
}

/// This is the standard character-to-glyph-index mapping table for the Windows platform for fonts
/// supporting Unicode supplementary-plane characters (U+10000 to U+10FFFF). See Windows platform
/// (platform ID = 3) above for additional details regarding subtable formats for Unicode encoding
/// on the Windows platform.
///
/// Format 12 is similar to format 4 in that it defines segments for sparse representation. It
/// differs, however, in that it uses 32-bit character codes.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CharacterGlyphIndexMappingSubtable12 {
    language: u16,
    groups: Vec<SequentialMapGroup>
}

impl CharacterGlyphIndexMappingSubtable12 {
    /// For requirements on use of the language field.
    pub fn language(&self) -> u16 {
        self.language
    }

    /// Array of SequentialMapGroup records.
    pub fn groups(&self) -> &Vec<SequentialMapGroup> {
        &self.groups
    }

    pub fn mapping(&self) -> HashMap<u32, GlyphId> {
        let mut mapping = HashMap::new();
        for group in &self.groups {
            for i in 0..(group.end_char_code() - group.start_char_code() + 1) {
                mapping.insert(
                    group.start_char_code() + i,
                    group.start_glyph_id() as u16 + i as u16,
                );
            }
        }
        mapping
    }
}

/// This subtable provides for situations in which the same glyph is used for hundreds or even
/// thousands of consecutive characters spanning across multiple ranges of the code space. This
/// subtable format may be useful for “last resort” fonts, although these fonts may use other
/// suitable subtable formats as well.
///
/// Note: Subtable format 13 has the same structure as format 12; it differs only in the
/// interpretation of the startGlyphID/glyphID fields.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CharacterGlyphIndexMappingSubtable13 {
    language: u16,
    groups: Vec<ConstantMapGroup>
}

impl CharacterGlyphIndexMappingSubtable13 {
    /// For requirements on use of the language field.
    pub fn language(&self) -> u16 {
        self.language
    }

    /// Array of ConstantMapGroup records.
    pub fn groups(&self) -> &Vec<ConstantMapGroup> {
        &self.groups
    }

    pub fn mapping(&self) -> HashMap<u32, GlyphId> {
        let mut mapping = HashMap::new();
        for group in &self.groups {
            for i in 0..(group.end_char_code() - group.start_char_code() + 1) {
                mapping.insert(
                    group.start_char_code() + i,
                    group.glyph_id() as u16,
                );
            }
        }
        mapping
    }
}

/// Subtable format 14 specifies the Unicode Variation Sequences (UVSes) supported by the font.
/// A Variation Sequence, according to the Unicode Standard, comprises a base character followed
/// by a variation selector. For example, <U+82A6, U+E0101>.
///
/// The subtable partitions the UVSes supported by the font into two categories: “default” and
/// “non-default” UVSes. Given a UVS, if the glyph obtained by looking up the base character of
/// that sequence in the Unicode 'cmap' subtable (i.e. the BMP subtable or the BMP +
/// supplementary-planes subtable) is the glyph to use for that sequence, then the sequence is a
/// default UVS; otherwise it is a non-default UVS, and the glyph to use for that sequence is
/// specified in the format 14 subtable itself.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CharacterGlyphIndexMappingSubtable14 {
    var_selector: Vec<VariationSelectorRecord>
}

impl CharacterGlyphIndexMappingSubtable14 {
    /// Array of VariationSelector records.
    pub fn var_selector(&self) -> &Vec<VariationSelectorRecord> {
        &self.var_selector
    }

    pub fn mapping(&self) -> HashMap<u32, GlyphId> {
        unimplemented!()
    }
}

/// Each variation selector records specifies a variation selector character, and offsets to
/// default and non-default tables used to map variation sequences using that variation selector.
///
/// The Variation Selector Records are sorted in increasing order of varSelector. No two records
/// may have the same varSelector.
///
/// A Variation Selector Record and the data its offsets point to specify those UVSes supported by
/// the font for which the variation selector is the varSelector value of the record. The base
/// characters of the UVSes are stored in the tables pointed to by the offsets. The UVSes are
/// partitioned by whether they are default or non-default UVSes.
///
/// Glyph IDs to be used for non-default UVSes are specified in the Non-Default UVS table.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct VariationSelectorRecord {
    var_selector: u32,
    default_uvs_offset: u32,
    non_default_uvs_offset: u32
}

impl VariationSelectorRecord {
    /// Variation selector.
    pub fn var_selector(&self) -> u32 {
        self.var_selector
    }

    /// Offset from the start of the format 14 subtable to Default UVS Table. May be 0.
    pub fn default_uvs_offset(&self) -> u32 {
        self.default_uvs_offset
    }

    /// Offset from the start of the format 14 subtable to Non-Default UVS Table. May be 0.
    pub fn non_default_uvs_offset(&self) -> u32 {
        self.non_default_uvs_offset
    }
}

/// A Default UVS Table is simply a range-compressed list of Unicode scalar values, representing
/// the base characters of the default UVSes which use the varSelector of the associated Variation
/// Selector Record.
///
/// For example, the range U+4E4D – U+4E4F (3 values) will set startUnicodeValue to 0x004E4D and
/// additionalCount to 2. A singleton range will set additionalCount to 0.
///
/// The sum (startUnicodeValue + additionalCount) must not exceed 0xFFFFFF.
///
/// The Unicode Value Ranges are sorted in increasing order of startUnicodeValue. The ranges must
/// not overlap; i.e., (startUnicodeValue + additionalCount) must be less than the
/// startUnicodeValue of the following range (if any).
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DefaultUVSTable {
    ranges: Vec<UnicodeRangeRecord>
}

impl DefaultUVSTable {
    /// Array of UnicodeRange records.
    pub fn ranges(&self) -> &Vec<UnicodeRangeRecord> {
        &self.ranges
    }
}

/// Each Unicode range record specifies a contiguous range of Unicode values.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct UnicodeRangeRecord {
    start_unicode_value: u32,
    additional_count: u8
}

impl UnicodeRangeRecord {
    /// First value in this range.
    pub fn start_unicode_value(&self) -> u32 {
        self.start_unicode_value
    }

    /// Number of additional values in this range.
    pub fn additional_count(&self) -> u8 {
        self.additional_count
    }
}

/// A Non-Default UVS Table is a list of pairs of Unicode scalar values and glyph IDs. The Unicode
/// values represent the base characters of all non-default UVSes which use the varSelector of
/// the associated Variation Selector Record, and the glyph IDs specify the glyph IDs to use for
/// the UVSes.
///
/// The UVS Mappings are sorted in increasing order of unicodeValue. No two mappings in this table
/// may have the same ‘unicodeValue’ values.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NonDefaultUVSTable {
    uvs_mappings: Vec<UVSMappingRecord>
}

impl NonDefaultUVSTable {
    /// Array of UVSMapping records.
    pub fn uvs_mappings(&self) -> &Vec<UVSMappingRecord> {
        &self.uvs_mappings
    }
}

/// Each UVSMapping record provides a glyph ID mapping for one base Unicode character, when that
/// base character is used in a variation sequence with the current variation selector.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct UVSMappingRecord {
    unicode_value: u32,
    glyph_id: u16
}

impl UVSMappingRecord {
    /// Base Unicode value of the UVS.
    pub fn unicode_value(&self) -> u32 {
        self.unicode_value
    }

    /// Glyph ID of the UVS.
    pub fn glyph_id(&self) -> u16 {
        self.glyph_id
    }
}

named!(pub parse_character_glyph_index_mapping_table<&[u8],CharacterGlyphIndexMappingTable>,
    do_parse!(
        verify!(be_u16, |version| version == 0) >>
        num_tables: be_u16 >>
        (
            CharacterGlyphIndexMappingTable {
                num_tables
            }
        )
    )
);

named!(pub parse_encoding_record<&[u8],EncodingRecord>,
    do_parse!(
        platform_id: be_u16 >>
        encoding_id: be_u16 >>
        offset: be_u32 >>
        character_to_glyph_index_mapping_subtable: peek!(preceded!(take!(offset), parse_character_to_glyph_index_mapping_subtable)) >>
        platform: expr_opt!({
            let language_id = character_to_glyph_index_mapping_subtable.language();

            // The language field must be set to zero for all 'cmap' subtables whose platform IDs
            // are other than Macintosh (platform ID 1). For 'cmap' subtables whose platform IDs
            // are Macintosh, set this field to the Macintosh language ID of the 'cmap' subtable
            // plus one, or to zero if the 'cmap' subtable is not language-specific. For example,
            // a Mac OS Turkish 'cmap' subtable must set this field to 18, since the Macintosh
            // language ID for Turkish is 17. A Mac OS Roman 'cmap' subtable must set this field to
            // 0, since Mac OS Roman is not a language-specific encoding.
            let language_opt = match platform_id {
                1 if language_id > 0 => {
                    Some(language_id - 1)
                },
                _ => None
            };

            Platform::new(platform_id, encoding_id, language_opt)
        }) >>
        (
            EncodingRecord {
                platform,
                character_to_glyph_index_mapping_subtable
            }
        )
    )
);

named!(pub parse_character_to_glyph_index_mapping_subtable<&[u8],CharacterGlyphIndexMappingSubtable>,
    alt!(parse_character_to_glyph_index_mapping_subtable_0 |
         parse_character_to_glyph_index_mapping_subtable_2 |
         parse_character_to_glyph_index_mapping_subtable_4 |
         parse_character_to_glyph_index_mapping_subtable_6 |
         parse_character_to_glyph_index_mapping_subtable_8 |
         parse_character_to_glyph_index_mapping_subtable_10 |
         parse_character_to_glyph_index_mapping_subtable_12 |
         parse_character_to_glyph_index_mapping_subtable_13 |
         parse_character_to_glyph_index_mapping_subtable_14)
);

named!(parse_character_to_glyph_index_mapping_subtable_0<&[u8],CharacterGlyphIndexMappingSubtable>,
    do_parse!(
        verify!(be_u16, |format| format == 0) >>
        _length: be_u16 >>
        language: be_u16 >>
        glyph_id_array: take!(256) >>
        (
            CharacterGlyphIndexMappingSubtable::Format_0(CharacterGlyphIndexMappingSubtable0 {
                language,
                glyph_id_array
            })
        )
    )
);

named!(parse_character_to_glyph_index_mapping_subtable_2<&[u8],CharacterGlyphIndexMappingSubtable>,
    do_parse!(
        verify!(be_u16, |format| format == 2) >>
        _length: be_u16 >>
        language: be_u16 >>
        sub_header_keys: count!(be_u16, 256) >>
        (
            CharacterGlyphIndexMappingSubtable::Format_2(CharacterGlyphIndexMappingSubtable2 {
                language,
                sub_header_keys
            })
        )
    )
);

named!(parse_character_to_glyph_index_mapping_subtable_2_sub_header_record<&[u8],CharacterGlyphIndexMappingSubtable2SubHeaderRecord>,
    do_parse!(
        first_code: be_u16 >>
        entry_count: be_u16 >>
        id_delta: be_i16 >>
        id_range_offset: be_u16 >>
        (
            CharacterGlyphIndexMappingSubtable2SubHeaderRecord {
                first_code,
                entry_count,
                id_delta,
                id_range_offset
            }
        )
    )
);

fn get_glyph_id_count(seg_count: u16, start_code: &Vec<u16>, end_code: &Vec<u16>, id_range_offset: &Vec<u16>) -> Option<usize> {
    // The final start code and end code values must be 0xFFFF
    if start_code.last() != Some(&0xffff) || end_code.last() != Some(&0xffff) {
        return None
    }

    let mut length: u16 = 0;

    for (i, tuple) in start_code.iter()
        .zip(end_code.iter())
        .zip(id_range_offset.iter())
        .enumerate() {
        let ((&start_code, &end_code), &id_range_offset) = tuple;

        for j in start_code..(end_code + 1) {
            if id_range_offset > 0 {
                let end = (id_range_offset / 2 + (j - start_code)) - (seg_count - i as u16) as u16 + 1;
                if end > length {
                    length = end;
                }
            }
        }
    }

    Some(length as usize)
}

named!(pub parse_character_to_glyph_index_mapping_subtable_4<&[u8],CharacterGlyphIndexMappingSubtable>,
    do_parse!(
        verify!(be_u16, |format| format == 4) >>
        _length: be_u16 >>
        language: be_u16 >>
        seg_count: map!(verify!(be_u16, |val| val > 0 && val % 2 == 0),
            |seg_count_x2| seg_count_x2 << 1) >>
        search_range: be_u16 >>
        entry_selector: be_u16 >>
        range_shift: be_u16 >>
        end_code: count!(be_u16, seg_count as usize) >>
        // reservedPad
        take!(2) >>
        start_code: count!(be_u16, seg_count as usize) >>
        id_delta: count!(be_i16, seg_count as usize) >>
        id_range_offset: count!(be_u16, seg_count as usize) >>
        glyph_id_count: expr_opt!(get_glyph_id_count(seg_count, &start_code, &end_code, &id_range_offset)) >>
        glyph_id_array: take!(glyph_id_count) >>
        (
            CharacterGlyphIndexMappingSubtable::Format_4(CharacterGlyphIndexMappingSubtable4 {
                language,
                seg_count,
                search_range,
                entry_selector,
                range_shift,
                end_code,
                start_code,
                id_delta,
                id_range_offset,
                glyph_id_array
            })
        )
    )
);

named!(parse_character_to_glyph_index_mapping_subtable_6<&[u8],CharacterGlyphIndexMappingSubtable>,
    do_parse!(
        verify!(be_u16, |format| format == 6) >>
        _length: be_u16 >>
        language: be_u16 >>
        first_code: be_u16 >>
        entry_count: be_u16 >>
        glyph_id_array: take!(entry_count * 2) >>
        (
            CharacterGlyphIndexMappingSubtable::Format_6(CharacterGlyphIndexMappingSubtable6 {
                language,
                first_code,
                entry_count,
                glyph_id_array
            })
        )
    )
);

named!(parse_character_to_glyph_index_mapping_subtable_8<&[u8],CharacterGlyphIndexMappingSubtable>,
    do_parse!(
        verify!(be_u16, |format| format == 8) >>
        // Reserved; set to 0
        take!(2) >>
        _length: be_u32 >>
        language: be_u32 >>
        is32: take!(8192) >>
        groups: length_count!(be_u32, parse_sequential_map_group) >>
        (
            CharacterGlyphIndexMappingSubtable::Format_8(CharacterGlyphIndexMappingSubtable8 {
                language: language as u16,
                is32,
                groups
            })
        )
    )
);

named!(parse_sequential_map_group<&[u8],SequentialMapGroup>,
    do_parse!(
        start_char_code: be_u32 >>
        end_char_code: be_u32 >>
        start_glyph_id: be_u32 >>
        (
            SequentialMapGroup {
                start_char_code,
                end_char_code,
                start_glyph_id
            }
        )
    )
);

named!(parse_constant_map_group<&[u8],ConstantMapGroup>,
    do_parse!(
        start_char_code: be_u32 >>
        end_char_code: be_u32 >>
        glyph_id: be_u32 >>
        (
            ConstantMapGroup {
                start_char_code,
                end_char_code,
                glyph_id
            }
        )
    )
);

named!(parse_character_to_glyph_index_mapping_subtable_10<&[u8],CharacterGlyphIndexMappingSubtable>,
    do_parse!(
        verify!(be_u16, |format| format == 10) >>
        // Reserved; set to 0
        take!(2) >>
        _length: be_u32 >>
        language: be_u32 >>
        start_char_code: be_u32 >>
        num_chars: be_u32 >>
        glyphs: take!(num_chars * 2) >>
        (
            CharacterGlyphIndexMappingSubtable::Format_10(CharacterGlyphIndexMappingSubtable10 {
                language: language as u16,
                start_char_code,
                glyphs
            })
        )
    )
);

named!(parse_character_to_glyph_index_mapping_subtable_12<&[u8],CharacterGlyphIndexMappingSubtable>,
    do_parse!(
        verify!(be_u16, |format| format == 12) >>
        // Reserved; set to 0
        take!(2) >>
        _length: be_u32 >>
        language: be_u32 >>
        groups: length_count!(be_u32, parse_sequential_map_group) >>
        (
            CharacterGlyphIndexMappingSubtable::Format_12(CharacterGlyphIndexMappingSubtable12 {
                language: language as u16,
                groups
            })
        )
    )
);

named!(parse_character_to_glyph_index_mapping_subtable_13<&[u8],CharacterGlyphIndexMappingSubtable>,
    do_parse!(
        verify!(be_u16, |format| format == 13) >>
        // Reserved; set to 0
        take!(2) >>
        _length: be_u32 >>
        language: be_u32 >>
        groups: length_count!(be_u32, parse_constant_map_group) >>
        (
            CharacterGlyphIndexMappingSubtable::Format_13(CharacterGlyphIndexMappingSubtable13 {
                language: language as u16,
                groups
            })
        )
    )
);

named!(parse_character_to_glyph_index_mapping_subtable_14<&[u8],CharacterGlyphIndexMappingSubtable>,
    do_parse!(
        verify!(be_u16, |format| format == 14) >>
        _length: be_u32 >>
        var_selector: length_count!(be_u32, parse_variation_selector_record) >>
        (
            CharacterGlyphIndexMappingSubtable::Format_14(CharacterGlyphIndexMappingSubtable14 {
                var_selector
            })
        )
    )
);

named!(parse_variation_selector_record<&[u8],VariationSelectorRecord>,
    do_parse!(
        var_selector: be_u24 >>
        default_uvs_offset: be_u32 >>
        non_default_uvs_offset: be_u32 >>
        (
            VariationSelectorRecord {
                var_selector,
                default_uvs_offset,
                non_default_uvs_offset
            }
        )
    )
);

named!(parse_default_uvs_table<&[u8],DefaultUVSTable>,
    do_parse!(
        ranges: length_count!(be_u32, parse_unicode_range_record) >>
        (
            DefaultUVSTable {
                ranges
            }
        )
    )
);

named!(parse_unicode_range_record<&[u8],UnicodeRangeRecord>,
    do_parse!(
        start_unicode_value: be_u24 >>
        additional_count: be_u8 >>
        (
            UnicodeRangeRecord {
                start_unicode_value,
                additional_count
            }
        )
    )
);

named!(parse_non_default_uvs_table<&[u8],NonDefaultUVSTable>,
    do_parse!(
        uvs_mappings: length_count!(be_u32, parse_uvs_mapping_record) >>
        (
            NonDefaultUVSTable {
                uvs_mappings
            }
        )
    )
);

named!(parse_uvs_mapping_record<&[u8],UVSMappingRecord>,
    do_parse!(
        unicode_value: be_u24 >>
        glyph_id: be_u16 >>
        (
            UVSMappingRecord {
                unicode_value,
                glyph_id
            }
        )
    )
);