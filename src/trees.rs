pub mod raw;
pub mod literal;
pub mod typed;
pub mod symbols;
pub mod types;
pub mod gen;
pub mod props;

pub enum MaybeChanged {
    Unchanged,
    Changed
}