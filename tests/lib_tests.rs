use jati::error::Error;
use jati::runtime::Runtime;
use jati::symbols::id::Id;
use jati::symbols::ops::OpSig;
use jati::symbols::symbol_table::{BasicSymbolTable, SymbolTable};
use jati::trees::types::Type;
use jati::trees::values::Value;

struct TestRuntime {}

impl Runtime for TestRuntime {
    type S = BasicSymbolTable<Self>;
    type E = Error;

    fn request_stop(&mut self) {}

    fn stop_requested(&self) -> bool { false }
}

#[test]
fn script1() {
    let mut symbols = BasicSymbolTable::<TestRuntime>::new();
    symbols.define_fun(Id::new("do_stuff".to_string()),
                       OpSig::Fixed { tpe: Type::Unit, arg_types: vec![] },
                       |_, _, _| Ok(Value::Unit)).unwrap();
}

