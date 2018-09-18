extern crate opentype_rs as otf;

use otf::{OpenTypeFontFile, TableTag};
use otf::parser::tables::FontTable;

fn main() {
    let buf = include_bytes!("../fonts/Roboto/Roboto-Regular.ttf") as &[u8];
    let otff = OpenTypeFontFile::parse(buf).unwrap();

    for font in otff {
        let iter = font.iter().filter(|table| {
            table.tag() == TableTag::Head || table.tag() == TableTag::Name
        });

        for table in iter {
            match table.parse() {
                Ok(FontTable::Head(head_table)) => {
                    let bounding_box = head_table.bounding_box();
                    // ...
                },
                Ok(FontTable::Name(name_table)) => {
                    let name_records = name_table.name_records();
                    // ...
                },
                _ => {}
            }
        }
    }
}