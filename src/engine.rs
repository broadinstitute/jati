use crate::failure::Failure;
use crate::parser::Parser;
use crate::state::State;
use crate::result::Valid::{Active, Complete};

fn parse_bytes<A, C, S: State<A, C>>(mut byte_iter: Box<dyn Iterator<Item=u8>>,
                        parser: Box<dyn Parser<A, C, S>>)
                        -> Result<C, Failure> {
    let mut state = parser.new_state();
    loop {
        match byte_iter.next() {
            None => { return state.push_end() }
            Some(byte) => {
                match state.push_byte(byte) {
                    Ok(Active(_)) => {}
                    Ok(Complete(c)) => { return Err(state.get_leftover_input_failure(&c)) }
                    Err(failure) => { return Err(failure) }
                }
            }
        }
    }
}
