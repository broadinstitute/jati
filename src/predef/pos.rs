use crate::parser::Parser;
use crate::state::State;
use crate::pos::Pos;
use crate::result::{ParseResultOngoing, ParseResultFinal};
use crate::failure::Failure;
use crate::result::Valid::Active;

struct PosParser {}

struct LineEndState {
    at_end_with_lf: bool,
    at_end_with_cr: bool,
    at_first_of_line: bool,
}

struct PosParserState {
    line_end_state: LineEndState,
    pos: Pos
}

impl LineEndState {
    fn new() -> LineEndState {
        LineEndState { at_end_with_cr: false, at_end_with_lf: false, at_first_of_line: true }
    }
    fn push_non_end(&self) -> LineEndState {
        let at_end_with_lf = false;
        let at_end_with_cr = false;
        let at_first_of_line = self.at_end_with_lf || self.at_end_with_cr;
        LineEndState { at_end_with_lf, at_end_with_cr, at_first_of_line }
    }
    fn push_lf(&self) -> LineEndState {
        let at_end_with_lf = true;
        let (at_end_with_cr, at_first_of_line) =
            if self.at_end_with_lf {
                (false, true)
            } else {
                (self.at_end_with_cr, false)
            };
        LineEndState { at_end_with_lf, at_end_with_cr, at_first_of_line }
    }
    fn push_cr(&self) -> LineEndState {
        let at_end_with_cr = true;
        let (at_end_with_lf, at_first_of_line) =
            if self.at_end_with_cr {
                (false, true)
            } else {
                (self.at_end_with_lf, false)
            };
        LineEndState { at_end_with_lf, at_end_with_cr, at_first_of_line }
    }
}

impl Parser<Pos, Pos, PosParserState> for PosParser {
    fn new_state(&self) -> PosParserState {
        PosParserState { line_end_state: LineEndState::new(), pos: Pos::new() }
    }
}

impl State<Pos, Pos> for PosParserState {
    fn push_byte(&mut self, byte: u8) -> ParseResultOngoing<Pos, Pos> {
        match byte {
            b'\n' => { self.line_end_state = self.line_end_state.push_lf(); }
            b'\r' => { self.line_end_state = self.line_end_state.push_cr(); }
            _ => { self.line_end_state = self.line_end_state.push_non_end(); }
        }
        if self.line_end_state.at_first_of_line {
            self.pos = self.pos.next_line();
        } else {
            self.pos = self.pos.next_in_line();
        }
        Ok(Active(self.pos.clone()))
    }

    fn push_end(&mut self) -> ParseResultFinal<Pos> {
        Ok(self.pos.clone())
    }

    fn get_leftover_input_failure(&self, c: &Pos) -> Failure {
        todo!()
    }
}