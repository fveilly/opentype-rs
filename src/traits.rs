use error::Error;
use table_record::TableRecord;

pub trait Parser<'otf> {
    type Item;

    fn parse(buf: &'otf[u8]) -> Result<Self::Item, Error>;
}

pub trait TableParser<'otf> : Parser<'otf> {
    fn parse_table(table_record: &TableRecord<'otf>) -> Result<Self::Item, Error> {
        Self::parse(table_record.get_table_as_slice()?)
    }
}