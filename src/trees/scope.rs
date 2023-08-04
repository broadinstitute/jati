pub struct Scope {
    parent: Option<Box<Scope>>,

}

pub enum Locals {
    Empty,
    Single,
    Multi
}

pub struct SingleLocal {

}

pub struct Namespace {

}