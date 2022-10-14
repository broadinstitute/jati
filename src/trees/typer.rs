use crate::runtime::fun::Fun;
use crate::runtime::var::Var;
use crate::error::Error;
use crate::trees::symbols::Symbols;
use crate::trees::raw::tree::Tree as RawTree;
use crate::trees::raw::call::Call as RawCall;
use crate::trees::typed::tree::Tree as TypedTree;
use crate::trees::typed::call::Call as TypedCall;
use crate::trees::typed::var_ref::VarRef as VarRef;
use crate::trees::raw::id::Id;
use crate::trees::types::Type;

pub(crate) fn to_typed_tree<V, F, S>(symbols: &mut S, raw_tree: RawTree)
                                     -> Result<TypedTree<V, F>, Error>
    where V: Var, F: Fun, S: Symbols<V, F> {
    match raw_tree {
        RawTree::Call(raw_call) => { Ok(TypedTree::Call(to_type_call(symbols, raw_call)?)) }
        RawTree::Var(id) => { Ok(TypedTree::Var(to_var_ref(symbols, id)?)) }
        RawTree::Lit(literal) => { Ok(TypedTree::Lit(literal)) }
    }
}


pub(crate) fn to_type_call<V, F, S>(symbols: &mut S, raw_call: RawCall)
                                    -> Result<TypedCall<V, F>, Error>
    where V: Var, F: Fun, S: Symbols<V, F> {
    let RawCall { callee, args: args_raw } = raw_call;
    let name = callee.string;
    let mut args: Vec<TypedTree<V, F>> = Vec::new();
    let mut arg_types: Vec<Type> = Vec::new();
    for arg_raw in args_raw {
        let arg = to_typed_tree(symbols, arg_raw)?;
        arg_types.push(arg.tpe());
        args.push(arg);
    }
    let fun = symbols.get_fun(name.as_str(), arg_types)?;
    Ok(TypedCall { name, fun, args })
}

pub(crate) fn to_var_ref<V, F, S>(symbols: &mut S, id: Id) -> Result<VarRef<V>, Error>
    where V: Var, F: Fun, S: Symbols<V, F> {
    let name = id.string;
    let var = symbols.get_var(name.as_str())?;
    Ok(VarRef { name, var })
}