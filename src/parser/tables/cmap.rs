use nom::{be_u8, be_i16, be_u16, be_u32};
use parser::tables::name::Platform;
use std::fmt;

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

impl CharacterGlyphIndexMappingTable {
    pub fn num_tables(&self) -> u16 {
        self.num_tables
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
pub struct EncodingRecord {
    platform: Platform,
    character_to_glyph_index_mapping_subtable: CharacterGlyphIndexMappingSubtable
}

impl EncodingRecord {
    /// The platform, encoding and language IDs.
    pub fn platform(&self) -> Platform {
        self.platform
    }

    /// Subtable for this encoding.
    pub fn character_to_glyph_index_mapping_subtable(&self) -> &CharacterGlyphIndexMappingSubtable {
        &self.character_to_glyph_index_mapping_subtable
    }
}

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

#[derive(Debug, Clone, Eq, PartialEq)]
#[allow(non_camel_case_types)]
pub enum CharacterGlyphIndexMappingSubtable {
    Format_0(CharacterGlyphIndexMappingSubtable0),
    // TODO: Format 2 High-byte mapping through table
    Format_4(CharacterGlyphIndexMappingSubtable4),
}

impl CharacterGlyphIndexMappingSubtable {
    pub fn language(&self) -> u16 {
        match self {
            CharacterGlyphIndexMappingSubtable::Format_0(subtable) => subtable.language(),
            CharacterGlyphIndexMappingSubtable::Format_4(subtable) => subtable.language(),
        }
    }
}

named!(pub parse_character_to_glyph_index_mapping_subtable<&[u8],CharacterGlyphIndexMappingSubtable>,
    alt!(parse_character_to_glyph_index_mapping_subtable_0 |
         parse_character_to_glyph_index_mapping_subtable_4)
);

/// This is the Apple standard character to glyph index mapping table.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CharacterGlyphIndexMappingSubtable0 {
    language: u16,
    glyph_id_array: Vec<u8>
}

impl CharacterGlyphIndexMappingSubtable0 {
    /// For requirements on use of the language field.
    pub fn language(&self) -> u16 {
        self.language
    }
}

named!(parse_character_to_glyph_index_mapping_subtable_0<&[u8],CharacterGlyphIndexMappingSubtable>,
    do_parse!(
        verify!(be_u16, |format| format == 0) >>
        length: be_u16 >>
        language: be_u16 >>
        glyph_id_array: count!(be_u8, 256) >>
        (
            CharacterGlyphIndexMappingSubtable::Format_0(CharacterGlyphIndexMappingSubtable0 {
                language,
                glyph_id_array
            })
        )
    )
);

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
pub struct CharacterGlyphIndexMappingSubtable4 {
    language: u16,
    seg_count: u16,
    search_range: u16,
    entry_selector: u16,
    range_shift: u16,
    end_code: Vec<u16>,
    start_code: Vec<u16>,
    id_delta: Vec<i16>,
    id_range_offset: Vec<u16>,
    glyph_id_array: Vec<u16>
}

impl CharacterGlyphIndexMappingSubtable4 {
    /// For requirements on use of the language field.
    pub fn language(&self) -> u16 {
        self.language
    }
}

named!(pub parse_character_to_glyph_index_mapping_subtable_4<&[u8],CharacterGlyphIndexMappingSubtable>,
    do_parse!(
        verify!(be_u16, |format| format == 4) >>
        length: be_u16 >>
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
        glyph_id_array: count!(be_u16, seg_count as usize) >>
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