use crate::rule::Rule;

pub struct RuleSet<I> {
    rules: Vec<Box<dyn Rule<I>>>
}