pub mod raw;
pub mod literal;
pub mod typed;
pub mod symbols;
pub mod types;
pub mod gen;

pub enum MaybeChanged {
    Unchanged,
    Changed
}