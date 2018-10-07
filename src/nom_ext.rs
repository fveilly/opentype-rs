use nom::types::CompleteByteSlice;
use nom::{Err, ErrorKind, IResult, Context};

#[inline]
pub fn be_u32_c(i: CompleteByteSlice) -> IResult<CompleteByteSlice, u32> {
    if i.len() < 4 {
        Err(Err::Error(Context::Code(i, ErrorKind::Eof)))
    } else {
        let res = ((i[0] as u32) << 24) + ((i[1] as u32) << 16) + ((i[2] as u32) << 8) + i[3] as u32;
        Ok((CompleteByteSlice(&i[4..]), res))
    }
}