use std::rc::Rc;
use jati::parse_string;
use jati::parse::parsers::id::RustIdParser;
use jati::parse::PResult;
use jati::parse::parsers::script::ScriptParser;
use jati::parse::parsers::white::RustWhiteSpaceParser;

fn print_error<T>(result: PResult<T>) -> PResult<T> {
    if let Err(error) = &result {
        println!("{}", error)
    }
    result
}

fn script_parser() -> ScriptParser {
    let ws_parser = Rc::new(RustWhiteSpaceParser::new());
    let id_parser = Rc::new(RustIdParser::new());
    ScriptParser::new(ws_parser, id_parser)
}

#[test]
fn script1() {
    let script_parser = script_parser();
    const SCRIPT: &'static str = "do_stuff();";
    let result = parse_string(script_parser, SCRIPT);
    assert!(print_error(result).is_ok());
}

