extern crate opentype_rs as otf;

use otf::OpenTypeFontFile;
use otf::tables::TableTag;
use otf::tables::head::FontHeaderTable;
use otf::tables::name::NamingTable;
use otf::parser::Parse;

fn main() {
    let buf = include_bytes!("../fonts/Roboto/Roboto-Regular.ttf") as &[u8];
    let otff = OpenTypeFontFile::parse(buf).unwrap();

    for font in otff {
        for table in font.iter() {
            match table.tag() {
                TableTag::Head => {
                    let font_header_table = FontHeaderTable::parse(table.as_slice().unwrap_or_default()).unwrap().1;
                    assert_eq!(font_header_table.font_revision(), 140050);
                },
                TableTag::Name => {
                    let naming_table = NamingTable::parse(table.as_slice().unwrap_or_default()).unwrap().1;
                    assert_eq!(naming_table.string_offset(), 318);
                },
                _ => {}
            }
        }
    }
}