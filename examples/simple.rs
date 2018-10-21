extern crate opentype_rs as otf;

use otf::{OpenTypeFontFile, TableTag, FontHeaderTable, NamingTable};
use otf::traits::TableParser;

fn main() {
    let buf = include_bytes!("../fonts/Roboto/Roboto-Regular.ttf") as &[u8];
    let otff = OpenTypeFontFile::parse(buf).unwrap();

    for font in otff {
        for table_record in font.iter() {
            match table_record.tag() {
                TableTag::Head => {
                    let font_header_table = FontHeaderTable::parse_table(&table_record).unwrap();
                    assert_eq!(font_header_table.font_revision(), 140050);
                },
                TableTag::Name => {
                    let naming_table = NamingTable::parse_table(&table_record).unwrap();
                    assert_eq!(naming_table.string_offset(), 318);
                },
                _ => {}
            }
        }
    }
}