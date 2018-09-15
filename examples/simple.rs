extern crate opentype_rs as otf;

use otf::OpenTypeFontFile;
use otf::TableTag;

fn main() {
    let buf = include_bytes!("../fonts/Roboto/Roboto-Regular.ttf") as &[u8];
    let otff = OpenTypeFontFile::parse(buf).unwrap();

    for font in otff {
        println!("FONT sfnt_version={}", font.sfnt_version());
        for table in font.iter() {
            println!("  TABLE tag='{}' checksum={}", table.tag(), if table.validate() { "OK" } else { "ERROR" });

            let table_tag = TableTag::parse(table.tag());

            match table_tag {
                Some(TableTag::Head) => println!("{}", table),
                Some(TableTag::Hhea) => println!("{}", table),
                Some(TableTag::Maxp) => println!("{}", table),
                _ => {}
            }
        }
    }
}
