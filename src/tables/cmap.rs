use nom::{be_i16, be_u16, be_u32};
use tables::FontTable;

/// This table defines the mapping of character codes to the glyph index values used in the font.
/// It may contain more than one subtable, in order to support more than one character encoding
/// scheme.
pub struct Cmap {
    encoding_records: Vec<EncodingRecord>
}

named!(pub parse_cmap<&[u8],FontTable>,
    do_parse!(
        tag!(&[0x00, 0x00]) >>
        num_tables: be_u16 >>
        encoding_records: count!(parse_encoding_record, num_tables as usize) >>
        (
            FontTable::Cmap(Cmap{
                encoding_records
            })
        )
    )
);

pub struct EncodingRecord {
    platform_id: u16,
    encoding_id: u16,
    subtable: SubtableFormat
}

named!(pub parse_encoding_record<&[u8],EncodingRecord>,
    do_parse!(
        platform_id: be_u16 >>
        encoding_id: be_u16 >>
        offset: be_u32 >>
        subtable: peek!(preceded!(take!(offset), parse_subtable_format)) >>
        (
            EncodingRecord {
                platform_id,
                encoding_id,
                subtable
            }
        )
    )
);

pub enum SubtableFormat {
    Format0(SubtableFormat0),
    // TODO: Format 2 High-byte mapping through table
    Format4(SubtableFormat4),
}

named!(pub parse_subtable_format<&[u8],SubtableFormat>,
    switch!(be_u16,
        0 => call!(parse_subtable_format_0) |
        4 => call!(parse_subtable_format_4)
    )
);

/// This is the Apple standard character to glyph index mapping table.
/// TODO: Implement Debug deriv for glyph_id_array
#[derive(Copy, Clone)]
pub struct SubtableFormat0 {
    language: u16,

    /// An array that maps character codes to glyph index values
    glyph_id_array: [u8; 256]
}

named!(pub parse_subtable_format_0<&[u8],SubtableFormat>,
    do_parse!(
        length: be_u16 >>
        language: be_u16 >>
        glyph_id_array: map!(take!(256), |o| {
            let mut arr: [u8; 256] = [0; 256];
            arr.copy_from_slice(o);
            arr
        }) >>
        (
            SubtableFormat::Format0(SubtableFormat0 {
                language,
                glyph_id_array
            })
        )
    )
);

/// This is the standard character-to-glyph-index mapping table for the Windows platform for fonts
/// that support Unicode BMP characters.
pub struct SubtableFormat4 {
    language: u16,

    /// Segments count
    seg_count: u16,

    /// 2 × (2**floor(log2(segCount)))
    search_range: u16,

    /// log2(searchRange/2)
    entry_selector: u16,

    /// 2 × segCount - searchRange
    range_shift: u16,

    /// End characterCode for each segment, last=0xFFFF
    end_code: Vec<u16>,

    /// Start character code for each segment
    start_code:  Vec<u16>,

    /// Delta for all character codes in segment
    id_delta:  Vec<i16>,

    /// Offsets into glyphIdArray or 0
    id_range_offset: Vec<u16>,

    /// Glyph index array (arbitrary length)
    glyph_id_array: Vec<u16>
}

named!(pub parse_subtable_format_4<&[u8],SubtableFormat>,
    do_parse!(
        length: be_u16 >>
        language: be_u16 >>
        seg_count: map!(verify!(be_u16, |val| val > 0 && val % 2 == 0),
            |seg_count_x2| seg_count_x2 << 1) >>
        search_range: be_u16 >>
        entry_selector: be_u16 >>
        range_shift: be_u16 >>
        end_code: count!(be_u16, seg_count as usize) >>
        tag!(&[0x00, 0x00]) >>
        start_code: count!(be_u16, seg_count as usize) >>
        id_delta: count!(be_i16, seg_count as usize) >>
        id_range_offset: count!(be_u16, seg_count as usize) >>
        glyph_id_array: count!(be_u16, seg_count as usize) >>
        (
            SubtableFormat::Format4(SubtableFormat4 {
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
