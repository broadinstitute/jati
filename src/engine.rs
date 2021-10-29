use crate::failure::Failure;
use crate::parser::Parser;
use crate::state::State;
use crate::result::Valid::{Active, Complete};

pub(crate) fn parse_bytes<A, C, S: State<A, C>>(mut byte_iter: Box<dyn Iterator<Item=u8>>,
                                                parser: &dyn Parser<A, C, S>)
                                                -> Result<C, Failure> {
    let mut state = parser.new_state();
    loop {
        match byte_iter.next() {
            None => { return state.push_end(); }
            Some(byte) => {
                match state.push_byte(byte) {
                    Ok(Active(_)) => {}
                    Ok(Complete(_)) => {
                        return Err(Failure::for_expected_end(byte, state.pos()));
                    }
                    Err(failure) => { return Err(failure); }
                }
            }
        }
    }
}
