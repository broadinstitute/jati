pub mod raw;
pub mod literal;
pub mod typed;
pub mod symbols;
pub mod types;
pub mod scope;
pub(crate) mod values;

pub enum MaybeChanged {
    Unchanged,
    Changed
}