use crate::Error;
use crate::symbols::id::Id;
use crate::symbols::symbol_table::SymbolTable;
use crate::trees::raw::tree::Tree as RawTree;
use crate::trees::symbols::Symbols;
use crate::trees::typed::call::Call as TypedCall;
use crate::trees::typed::tree::Tree as TypedTree;
use crate::trees::types::Type;

pub struct Call {
    pub(crate) callee: Id,
    pub(crate) args: Vec<Box<dyn RawTree>>,
}

impl Call {
    pub(crate) fn new(callee: Id, args: Vec<Box<dyn RawTree>>) -> Call { Call { callee, args } }
}

impl RawTree for Call {
    fn into_typed(self, symbols: &mut SymbolTable) -> Result<Box<dyn TypedTree>, Error> {
        let Call { callee, args: args_raw } = self;
        let name = callee.string;
        let mut args_typed: Vec<Box<dyn TypedTree>> = Vec::new();
        let mut arg_types: Vec<Type> = Vec::new();
        for arg_raw in args_raw {
            let arg = arg_raw.into_typed(symbols)?;
            arg_types.push(arg.tpe());
            args_typed.push(arg);
        }
        let fun = symbols.get_fun(name.as_str(), arg_types)?;
        Ok(TypedCall { name, fun, args: args_typed })
    }
}
