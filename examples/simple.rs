extern crate opentype_rs as otf;

use otf::OpenTypeFontFile;
use otf::TableTag;
use otf::parser::tables::FontTable;

fn main() {
    let buf = include_bytes!("../fonts/Roboto/Roboto-Regular.ttf") as &[u8];
    let otff = OpenTypeFontFile::parse(buf).unwrap();

    for font in otff {
        println!("FONT sfnt_version={}", font.sfnt_version());
        for table in font.iter() {
            println!("  TABLE tag='{}' checksum={}", table.tag(), if table.validate() { "OK" } else { "ERROR" });

            match table.tag() {
                TableTag::Head | TableTag::Hhea | TableTag::Maxp | TableTag::Os2 => {
                    println!("    BUF={}", table);

                    match table.parse().unwrap() {
                        FontTable::Head(o) => println!("    OBJ={:?}", o),
                        FontTable::Hhea(o) => println!("    OBJ={:?}", o),
                        FontTable::Maxp(o) => println!("    OBJ={:?}", o),
                        FontTable::Os2(o) => println!("    OBJ={:?}", o),
                        _ => {}
                    }
                },
                _ => {}
            }
        }
    }
}
