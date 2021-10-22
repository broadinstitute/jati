use crate::state::State;

pub trait Parser<A, C, S: State<A, C>> {
    fn new_state() -> S;
}
