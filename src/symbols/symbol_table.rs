use std::collections::BTreeMap;
use std::sync::Arc;
use crate::run::RunState;

use crate::symbols::fun::{OpKey, OpTag, OpPreDef};
use crate::symbols::id::Id;
use crate::symbols::var::VarTag;
use crate::trees::symbols::{ArgsError, SymbolError};
use crate::trees::types::Type;

pub trait SymbolTable {
    fn get_var(&mut self, id: &Id) -> Result<Option<VarTag>, SymbolError>;
    fn get_fun(&mut self, id: &Id, args: &[Type]) -> Result<Option<OpTag>, SymbolError>;
}

pub struct PreDefFunTable {
    funs: BTreeMap<String, OpTag>
}

impl PreDefFunTable {
    pub fn new<R: RunState<E>, E: std::error::Error>(pre_def_funs: &[OpPreDef<R, Self, E>]) -> Self {
        let mut funs: BTreeMap<String, OpTag> = BTreeMap::new();
        for pre_def_fun in pre_def_funs {
            let key = OpKey::new(pre_def_fun.uuid);
            let sig = Arc::new(pre_def_fun.sig.clone());
            let tag = OpTag { key, sig };
            funs.insert(pre_def_fun.name.to_string(), tag);
        }
        PreDefFunTable { funs }
    }
}

impl SymbolTable for PreDefFunTable {
    fn get_var(&mut self, _id: &Id) -> Result<Option<VarTag>, SymbolError> {
        Ok(None)
    }

    fn get_fun(&mut self, id: &Id, args: &[Type]) -> Result<Option<OpTag>, SymbolError> {
        match self.funs.get(id.string.as_str()) {
            None => { Ok(None)}
            Some(tag) => {
                tag.sig.check_arg_types(args)
                    .map_err(|arg_fail| ArgsError::new(id.clone(), arg_fail))?;
                Ok(Some(tag.clone()))
            }
        }
    }
}