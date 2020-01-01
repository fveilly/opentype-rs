use nom::IResult;

pub trait Parse {
    type Item;

    fn parse(input: &[u8]) -> IResult<&[u8], Self::Item>;
}

/// Macro to implement Parse for a given type.
macro_rules! impl_parse {
    (
        $(#[$attr:meta])*
        $type_name:ty,
        $parser_name:ident
    ) => {
        use parser::Parse;

        impl<'otf> Parse for $type_name {
            type Item = $type_name;

            $(#[$attr])*
            fn parse(input: &[u8]) -> IResult<&[u8], Self::Item> {
                $parser_name(input)
            }
        }
    }
}