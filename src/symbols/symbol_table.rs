use std::collections::BTreeMap;
use std::sync::Arc;

use uuid::Uuid;

use crate::symbols::fun::{FunKey, FunSig, FunTag};
use crate::symbols::id::Id;
use crate::symbols::var::VarTag;
use crate::trees::symbols::{ArgsError, SymbolError};
use crate::trees::types::Type;

pub trait SymbolTable {
    fn get_var(&mut self, id: &Id) -> Result<Option<VarTag>, SymbolError>;
    fn get_fun(&mut self, id: &Id, args: &[Type]) -> Result<Option<FunTag>, SymbolError>;
}

pub struct PreDefFun {
    pub name: &'static str,
    pub uuid: Uuid,
    pub sig: FunSig
}

pub struct PreDefFunTable {
    funs: BTreeMap<&'static str, FunTag>
}

impl PreDefFunTable {
    pub fn new(pre_def_funs: &'static [PreDefFun]) -> Self {
        let mut funs: BTreeMap<&'static str, FunTag> = BTreeMap::new();
        for pre_def_fun in pre_def_funs {
            let key = FunKey::new(pre_def_fun.uuid);
            let sig = Arc::new(pre_def_fun.sig.clone());
            let tag = FunTag { key, sig };
            funs.insert(pre_def_fun.name, tag);
        }
        PreDefFunTable { funs }
    }
}

impl SymbolTable for PreDefFunTable {
    fn get_var(&mut self, _id: &Id) -> Result<Option<VarTag>, SymbolError> {
        Ok(None)
    }

    fn get_fun(&mut self, id: &Id, args: &[Type]) -> Result<Option<FunTag>, SymbolError> {
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