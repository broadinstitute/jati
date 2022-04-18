use crate::error::Error;
use crate::rule_set::RuleSet;

pub trait Parser<I> {
    fn compile(&self) -> Result<RuleSet<I>, Error>;
}
