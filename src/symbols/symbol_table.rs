use std::collections::BTreeMap;
use std::sync::Arc;

use crate::runtime::Runtime;
use crate::symbols::id::Id;
use crate::symbols::ops::{Func, OpKey, OpSig, OpTag};
use crate::symbols::var::{VarKey, VarSig, VarTag};
use crate::trees::symbols::{ArgsError, SymbolError};
use crate::trees::types::Type;
use crate::trees::values::Value;

pub trait SymbolTable<R: Runtime + ?Sized> {
    fn define_var(&mut self, id: Id, value: Value) -> Result<VarKey, SymbolError>;
    fn define_fun(&mut self, id: Id, sig: OpSig, func: Func<R>) -> Result<OpKey, SymbolError>;
    fn resolve_var(&mut self, id: &Id) -> Result<Option<VarTag>, SymbolError>;
    fn resolve_fun(&mut self, id: &Id, args: &[Type]) -> Result<Option<OpTag>, SymbolError>;
    fn read_var(&self, key: &VarKey) -> Option<&Value>;
    fn read_fun(&self, key: &OpKey) -> Option<&Func<R>>;
}

pub struct BasicSymbolTable<R: Runtime> {
    var_defs: BTreeMap<Id, VarTag>,
    fun_defs: BTreeMap<Id, OpTag>,
    values: BTreeMap<VarKey, Value>,
    funcs: BTreeMap<OpKey, Func<R>>,
}

impl<R: Runtime> BasicSymbolTable<R> {
    pub fn new() -> Self {
        BasicSymbolTable {
            var_defs: BTreeMap::new(),
            fun_defs: BTreeMap::new(),
            values: BTreeMap::new(),
            funcs: BTreeMap::new(),
        }
    }
}

impl<R: Runtime> Default for BasicSymbolTable<R> {
    fn default() -> Self { Self::new() }
}

impl<R: Runtime> SymbolTable<R> for BasicSymbolTable<R> {
    fn define_var(&mut self, id: Id, value: Value) -> Result<VarKey, SymbolError> {
        let key = VarKey::create_random();
        let tpe = value.tpe();
        let sig = VarSig { tpe };
        let tag = VarTag { key, sig };
        self.var_defs.insert(id, tag.clone());
        self.values.insert(key, value);
        Ok(key)
    }

    fn define_fun(&mut self, id: Id, sig: OpSig, func: Func<R>) -> Result<OpKey, SymbolError> {
        let key = OpKey::create_random();
        self.funcs.insert(key, func);
        let sig = Arc::new(sig);
        let tag = OpTag { key, sig };
        self.fun_defs.insert(id, tag);
        Ok(key)
    }

    fn resolve_var(&mut self, id: &Id) -> Result<Option<VarTag>, SymbolError> {
        match self.var_defs.get(id) {
            None => { Ok(None) }
            Some(tag) => { Ok(Some(tag.clone())) }
        }
    }

    fn resolve_fun(&mut self, id: &Id, args: &[Type]) -> Result<Option<OpTag>, SymbolError> {
        match self.fun_defs.get(id) {
            None => { Ok(None) }
            Some(tag) => {
                tag.sig.check_arg_types(args)
                    .map_err(|arg_fail| ArgsError::new(id.clone(), arg_fail))?;
                Ok(Some(tag.clone()))
            }
        }
    }

    fn read_var(&self, key: &VarKey) -> Option<&Value> { self.values.get(key) }
    fn read_fun(&self, key: &OpKey) -> Option<&Func<R>> { self.funcs.get(key) }
}
