use crate::failure::Failure;
use crate::parser::Parser;

fn parse_bytes<A, C, S>(byte_iter: Box<dyn Iterator<Item=u8>>, parser: Box<dyn Parser<A, C, S>>)
                        -> Result<C, Failure> {

    todo!()
}
