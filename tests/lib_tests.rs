use jati::error::Error;
use jati::parse::parsers::id::RustIdParser;
use jati::parse::parsers::script::ScriptParser;
use jati::parse::parsers::white::DefaultWhiteSpaceParser;
use jati::parse_string;
use jati::runtime::Runtime;
use jati::symbols::id::Id;
use jati::symbols::ops::{OpFn, OpKey, OpSig};
use jati::symbols::symbol_table::{BasicSymbolTable, SymbolTable};
use jati::symbols::var::VarKey;
use jati::trees::types::Type;
use jati::trees::values::Value;

struct TestRuntime {}

impl Runtime for TestRuntime {
    type S = BasicSymbolTable<Self>;
    type E = Error;

    fn request_stop(&mut self) {}

    fn stop_requested(&self) -> bool { false }

    fn set_var_value(&mut self, _key: &VarKey, _value: Value) -> Result<(), Self::E> {
        Ok(())
    }

    fn get_var_value(&self, _key: &VarKey) -> Result<Value, Self::E> {
        Ok(Value::Unit)
    }

    fn get_op_func(&self, _key: &OpKey) -> Result<OpFn<Self>, Self::E> {
        Ok(OpFn::new(|_, _, _| Ok(Value::Unit)))
    }
}
fn script_parser() -> ScriptParser {
    let ws_parser = DefaultWhiteSpaceParser::new();
    let id_parser = RustIdParser::new();
    ScriptParser::new(ws_parser, id_parser)
}

#[test]
fn script1() {
    let script_parser = script_parser();
    const SCRIPT: &str = "do_stuff();";
    let raw_tree = parse_string(script_parser, SCRIPT).unwrap();
    let mut symbols = BasicSymbolTable::<TestRuntime>::new();
    symbols.define_fun(Id::new("do_stuff".to_string()),
                       OpSig::Fixed { tpe: Type::Unit, arg_types: vec![] },
                       |_, _, _| Ok(Value::Unit)).unwrap();
    let typed_tree = raw_tree.into_typed(&mut symbols).unwrap();
    assert_eq!(typed_tree.tpe(), Type::Unit);
}

