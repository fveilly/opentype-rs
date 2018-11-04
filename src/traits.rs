use error::Error;
use table::Table;

pub trait Parser<'otf> {
    type Item;

    fn parse(buf: &'otf[u8]) -> Result<Self::Item, Error>;
}

pub trait TableParser<'otf> : Parser<'otf> {
    fn parse_table(table_record: &Table<'otf>) -> Result<Self::Item, Error> {
        Self::parse(table_record.get_table_as_slice()?)
    }
}