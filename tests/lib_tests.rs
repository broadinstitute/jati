use jati::parse_string;
use jati::parse::parsers::id::RustIdParser;
use jati::parse::parsers::script::ScriptParser;
use jati::parse::parsers::white::DefaultWhiteSpaceParser;
use jati::trees::symbols::Symbols;
use jati::engine::var::Var;
use jati::engine::fun::Fun;
use jati::error::Error;
use jati::trees::symbols::errors::{no_such_fun, no_such_var};
use jati::trees::types::Type;
use jati::trees::typed::tree::Tree as TypedTree;

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

struct MockVar {
    tpe: Type,
}

impl Var for MockVar {
    fn tpe(&self) -> Type { self.tpe }
}

struct MockFun {
    name: String,
    tpe: Type,
}

impl MockFun {
    pub fn new(name: String, tpe: Type) -> MockFun { MockFun { name, tpe } }
}

impl Fun for MockFun {
    fn tpe(&self) -> Type { self.tpe }
}

struct MockSymbols {}

impl MockSymbols {
    pub fn new() -> MockSymbols {
        MockSymbols {}
    }
}

impl Symbols<MockVar, MockFun> for MockSymbols {
    fn get_var(&mut self, name: &str) -> Result<MockVar, Error> { Err(no_such_var(name)) }

    fn get_fun(&mut self, name: &str, args: Vec<Type>) -> Result<MockFun, Error> {
        if name == "do_stuff" && args.is_empty() {
            Ok(MockFun::new(String::from("do_stuff"), Type::Unit))
        } else {
            Err(no_such_fun(name))
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
        if let TypedTree::Call(call) = typed_tree {
            assert_eq!(call.name, "do_stuff");
            assert_eq!(call.fun.name, "do_stuff")
        } else {
            panic!("Tree is not a call.")
        }
    } else {
        assert!(print_error(parse_result).is_ok());
    }
}

