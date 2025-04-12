use std::fmt::{write, Display, Formatter};
use crate::error::Error;
use crate::runtime::Runtime;
use crate::symbols::id::Id;
use crate::symbols::ops::{OpKey, OpTag};
use crate::symbols::symbol_table::SymbolTable;
use crate::trees::props::{Props, Raw, Typed};
use crate::trees::symbols::SymbolError;
use crate::trees::tree::Tree;
use crate::trees::types::Type;

#[derive(Clone)]
pub enum Op<P: Props> {
    NonId(NonIdOp),
    Id(IdOp<P>),
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum NonIdOp {
    BlockYieldingLastValue,
    BlockYieldingUnit
}

#[derive(Clone)]
pub struct IdOp<P: Props> {
    pub id: Id,
    pub syntax: OpSyntax,
    pub op_tag: P::OT
}


#[derive(Clone, Ord, Eq, PartialEq, PartialOrd)]
pub enum OpSyntax {
    Call, Prefix, Postfix, Infix,
}

pub struct OpExpression<P: Props> {
    pub op: Op<P>,
    pub kids: Vec<Tree<P>>,
}

impl NonIdOp {
    pub fn tpe(&self, kids: &[Tree<Typed>]) -> Type {
        match self {
            NonIdOp::BlockYieldingLastValue => {
                kids.last().map_or(Type::Unit, |kid| kid.tpe())
            }
            NonIdOp::BlockYieldingUnit => {
                Type::Unit
            }
        }
    }
}

impl IdOp<Raw> {
    pub fn new(id: Id, syntax: OpSyntax) -> IdOp<Raw> {
        let op_tag = ();
        IdOp { id, syntax, op_tag }
    }
}

impl IdOp<Typed> {
    pub fn new(id: Id, syntax: OpSyntax, op_tag: OpTag) -> IdOp<Typed> {
        IdOp::<Typed> { id, syntax, op_tag }
    }
    pub fn tpe(&self) -> Type { self.op_tag.sig.tpe() }
    pub fn key(&self) -> &OpKey { &self.op_tag.key }
}

impl<P: Props> OpExpression<P> {
    pub fn new(op: Op<P>, kids: Vec<Tree<P>>) -> OpExpression<P> {
        OpExpression { op, kids }
    }

    pub fn into_tree(self) -> Tree<P> {
        Tree::Op(self)
    }

    pub(crate) fn into_typed<R: Runtime>(self, symbols: &mut dyn SymbolTable<R>)
                                         -> Result<OpExpression<Typed>, Error> {
        let mut typed_kids: Vec<Tree<Typed>> = Vec::new();
        let OpExpression { op, kids } = self;
        for kid in kids.into_iter() {
            typed_kids.push(kid.into_typed(symbols)?)
        }
        match op {
            Op::NonId(non_id_op) => {
                match non_id_op {
                    NonIdOp::BlockYieldingLastValue => {
                        let non_id_op = NonIdOp::BlockYieldingLastValue;
                        let op = Op::NonId(non_id_op);
                        Ok(OpExpression::new(op, typed_kids))
                    }
                    NonIdOp::BlockYieldingUnit => {
                        let non_id_op = NonIdOp::BlockYieldingUnit;
                        let op = Op::NonId(non_id_op);
                        Ok(OpExpression::new(op, typed_kids))
                    }
                }
            }
            Op::Id(id_op) => {
                let IdOp { id, syntax, .. } = id_op;
                let kid_types: Vec<Type>=
                    typed_kids.iter().map(|kid| kid.tpe()).collect();
                let fun =
                    symbols.resolve_fun(&id, &kid_types)?
                        .ok_or_else(|| SymbolError::no_such_fun(&id))?;
                let typed_id_op = IdOp::<Typed>::new(id, syntax, fun);
                let typed_op = Op::Id(typed_id_op);
                let typed_op_expression = OpExpression::new(typed_op, typed_kids);
                Ok(typed_op_expression)
            }
        }
    }
}

impl OpExpression<Typed> {
    pub fn tpe(&self) -> Type {
        match &self.op {
            Op::NonId(non_id_op) => non_id_op.tpe(&self.kids),
            Op::Id(id_op) => id_op.tpe()
        }
    }
}

impl<P: Props> Display for OpExpression<P> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.op {
            Op::NonId(non_id_op) => {
                match non_id_op {
                    NonIdOp::BlockYieldingLastValue => {
                        print_joined(f, &self.kids, ";\n")?;
                        writeln!(f)
                    }
                    NonIdOp::BlockYieldingUnit => {
                        print_ended(f, &self.kids, ";\n")
                    }
                }
            }
            Op::Id(id_op) => {
                match id_op.syntax {
                    OpSyntax::Call => {
                        write!(f, "{}(", id_op.id)?;
                        print_joined(f, &self.kids, ", ")?;
                        write!(f, ")")
                    }
                    OpSyntax::Prefix => { }
                    OpSyntax::Postfix => {}
                    OpSyntax::Infix => {}
                }
            }
        }
    }
}

fn print_joined<D: Display>(f: &mut Formatter<'_>, list: &[D], sep: &str) -> std::fmt::Result {
    let mut iter = list.iter();
    match iter.next() {
        None => { Ok(()) }
        Some(first) => {
            write!(f, "{first}")?;
            for item in iter {
                write!(f, "{sep}{item}")?;
            }
            Ok(())
        }
    }
}

fn print_ended<D: Display>(f: &mut Formatter<'_>, list: &[D], sep: &str) -> std::fmt::Result {
    for item in list {
        write!(f, "{item}{sep}")?;
    }
    Ok(())
}






