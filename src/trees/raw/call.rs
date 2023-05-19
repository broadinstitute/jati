use crate::Error;
use crate::symbols::id::Id;
use crate::symbols::symbol_table::SymbolTable;
use crate::trees::raw::tree::Tree as RawTree;
use crate::trees::symbols::SymbolError;
use crate::trees::typed::call::Call as TypedCall;
use crate::trees::typed::tree::Tree as TypedTree;
use crate::trees::types::Type;

pub struct Call {
    pub(crate) id: Id,
    pub(crate) args: Vec<Box<dyn RawTree>>,
}

impl Call {
    pub(crate) fn new(id: Id, args: Vec<Box<dyn RawTree>>) -> Call { Call { id, args } }
    pub(crate) fn into_typed_call(self, symbols: &mut dyn SymbolTable) -> Result<TypedCall, Error> {
        let Call { id, args: args_raw } = self;
        let mut args_typed: Vec<Box<dyn TypedTree>> = Vec::new();
        let mut arg_types: Vec<Type> = Vec::new();
        for arg_raw in args_raw.into_iter() {
            let arg = arg_raw.into_typed(symbols)?;
            arg_types.push(arg.tpe());
            args_typed.push(arg);
        }
        let fun =
            symbols.get_fun(&id, &arg_types)?
                .ok_or_else(|| SymbolError::no_such_fun(&id))?;
        Ok(TypedCall { id, fun, args: args_typed })
    }
}

impl RawTree for Call {
    fn into_typed(self: Box<Self>, symbols: &mut dyn SymbolTable)
        -> Result<Box<dyn TypedTree>, Error> {
        Ok(Box::new(self.into_typed_call(symbols)?))
    }
}
