use std::sync::Arc;
use jati::error::Error;
use jati::parse_string;
use jati::parse::parsers::id::RustIdParser;
use jati::parse::parsers::script::ScriptParser;
use jati::parse::parsers::white::DefaultWhiteSpaceParser;
use jati::trees::symbols::{ArgsFailure, SymbolError};
use jati::symbols::fun::{FunKey, FunSig, FunTag};
use jati::symbols::id::Id;
use jati::symbols::symbol_table::SymbolTable;
use jati::symbols::var::VarTag;
use jati::trees::types::Type;

fn print_error<T>(result: Result<T, Error>) -> Result<T, Error> {
    if let Err(error) = &result {
        println!("{}", error)
    }
    result
}

fn script_parser() -> ScriptParser {
    let ws_parser = DefaultWhiteSpaceParser::new();
    let id_parser = RustIdParser::new();
    ScriptParser::new(ws_parser, id_parser)
}


struct MockSymbols {
    do_stuff_tag: FunTag,
}

impl MockSymbols {
    pub fn new() -> MockSymbols {
        let key = FunKey::next();
        let sig = Arc::new(FunSig::Fixed { tpe: Type::Unit, arg_types: vec![] });
        let do_stuff_tag = FunTag { key, sig };
        MockSymbols { do_stuff_tag }
    }
}

impl SymbolTable for MockSymbols {
    fn get_var(&mut self, _id: &Id) -> Result<Option<VarTag>, SymbolError> {
        Ok(None)
    }

    fn get_fun(&mut self, id: &Id, args: &[Type]) -> Result<Option<FunTag>, SymbolError> {
        if id.string.as_str() == "do_stuff" {
            if args.is_empty() {
                Ok(Some(self.do_stuff_tag.clone()))
            } else {
                let args_failure =
                    ArgsFailure::new_wrong_number(args.len(), 0);
                Err(SymbolError::args_issue(id, args_failure))?
            }
        } else {
            Ok(None)
        }
    }
}

#[test]
fn script1() {
    let script_parser = script_parser();
    const SCRIPT: &str = "do_stuff();";
    let parse_result = parse_string(script_parser, SCRIPT);
    if let Ok(raw_tree) = parse_result {
        let mut symbols = MockSymbols::new();
        let typed_tree = raw_tree.into_typed(&mut symbols).unwrap();
        assert_eq!(typed_tree.tpe(), Type::Unit);
    } else {
        assert!(print_error(parse_result).is_ok());
    }
}

