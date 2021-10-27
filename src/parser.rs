use crate::state::State;
use crate::failure::Failure;
use crate::engine;

pub trait Parser<A, C, S: State<A, C>> {
    fn new_state(&self) -> S;
    fn parse(&self, byte_iter: Box<dyn Iterator<Item=u8>>) -> Result<C, Failure>
        where Self: Sized{
        engine::parse_bytes::<A, C, S>(byte_iter, self)
    }
}
