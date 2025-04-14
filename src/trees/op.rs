use std::fmt::{Display, Formatter};
use crate::error::Error;
use crate::runtime::Runtime;
use crate::symbols::id::Id;
use crate::symbols::ops::{OpTag};
use crate::symbols::symbol_table::SymbolTable;
use crate::trees::props::{Props, Raw, Typed};
use crate::trees::symbols::SymbolError;
use crate::trees::tree::Tree;
use crate::trees::types::Type;

#[derive(Clone)]
pub enum OpOld<P: Props> {
    NonId(NonIdOpOld),
    Id(IdOpOld<P>),
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum NonIdOpOld {
    BlockYieldingLastValue,
    BlockYieldingUnit
}

#[derive(Clone)]
pub struct IdOpOld<P: Props> {
    pub id: Id,
    pub syntax: OpSyntaxOld,
    pub op_tag: P::OT
}


#[derive(Clone, Ord, Eq, PartialEq, PartialOrd)]
pub enum OpSyntaxOld {
    Call, Prefix, Postfix, Infix,
}

pub struct OpExpression<P: Props> {
    pub op_tag: P::OT,
    pub op_structure: OpStructure<P>
}

pub enum OpStructure<P: Props> {
    Unary(UnaryStructure<P>),
    Binary(BinaryStructure<P>),
    Tertiary(TertiaryStructure<P>),
    Polyadic(PolyadicStructure<P>),
}

impl<P: Props> OpExpression<P> {
    pub fn into_tree(self) -> Tree<P> {
        Tree::Op(self)
    }
    pub fn into_typed<R: Runtime>(self, symbols: &mut dyn SymbolTable<R>)
                                  -> Result<OpExpression<Typed>, Error> {
        match self.op_structure {
            OpStructure::Unary(unary_structure) => {
                let UnaryStructure { phrasing, kid} = unary_structure;
                let typed_kid = kid.into_typed(symbols)?;
                let op_tag = symbols.resolve_op(&Phrasing::Unary(phrasing), &[typed_kid.tpe()])?
                    .ok_or_else(|| SymbolError::no_such_op(&Phrasing::Unary(phrasing)))?;
                let op_structure =
                    OpStructure::Unary(UnaryStructure { phrasing, kid: Box::new(typed_kid) });
                let typed_op_expression = OpExpression { op_tag, op_structure };
                Ok(typed_op_expression)
            }
            OpStructure::Binary(binary_structure) => {
                let BinaryStructure { phrasing, left, right } = binary_structure;
                let typed_left = left.into_typed(symbols)?;
                let typed_right = right.into_typed(symbols)?;
                let op_tag = symbols.resolve_op(&Phrasing::Binary(phrasing),
                                                &[typed_left.tpe(), typed_right.tpe()])?
                    .ok_or_else(|| SymbolError::no_such_op(&Phrasing::Binary(phrasing)))?;
                let op_structure =
                    OpStructure::Binary(BinaryStructure { phrasing, left: Box::new(typed_left),
                                                         right: Box::new(typed_right) });
                let typed_op_expression = OpExpression { op_tag, op_structure };
                Ok(typed_op_expression)
            }
            OpStructure::Tertiary(tertiary_structure) => {
                let TertiaryStructure {
                    phrasing, left, middle, right
                } = tertiary_structure;
                let typed_left = left.into_typed(symbols)?;
                let typed_middle = middle.into_typed(symbols)?;
                let typed_right = right.into_typed(symbols)?;
                let op_tag = symbols.resolve_op(&Phrasing::Tertiary(phrasing),
                                                &[typed_left.tpe(), typed_middle.tpe(),
                                                  typed_right.tpe()])?
                    .ok_or_else(|| SymbolError::no_such_op(&Phrasing::Tertiary(phrasing)))?;
                let op_structure =
                    OpStructure::Tertiary(TertiaryStructure { phrasing, left: Box::new(typed_left),
                                                              middle: Box::new(typed_middle),
                                                              right: Box::new(typed_right) });
                let typed_op_expression = OpExpression { op_tag, op_structure };
                Ok(typed_op_expression)
            }
            OpStructure::Polyadic(polyadic_structure) => {
                let PolyadicStructure { phrasing, kids }
                    = polyadic_structure;
                let mut typed_kids: Vec<Tree<Typed>> = Vec::new();
                for kid in kids.into_iter() {
                    typed_kids.push(kid.into_typed(symbols)?)
                }
                let kid_types: Vec<Type> = typed_kids.iter().map(|kid| kid.tpe()).collect();
                let op_tag = symbols.resolve_op(&Phrasing::Polyadic(phrasing), &kid_types)?
                    .ok_or_else(|| SymbolError::no_such_op(&Phrasing::Polyadic(phrasing)))?;
                let op_structure =
                    OpStructure::Polyadic(PolyadicStructure { phrasing, kids: typed_kids });
                let typed_op_expression =
                    OpExpression { op_tag, op_structure };
                Ok(typed_op_expression)
            }
        }
    }
}

#[derive(Clone)]
pub enum Phrasing {
    Unary(UnaryPhrasing),
    Binary(BinaryPhrasing),
    Tertiary(TertiaryPhrasing),
    Polyadic(PolyadicPhrasing),
}

pub struct UnaryPhrasing {
    pub id: Id,
    pub syntax: UnaryOpSyntax,
}

pub struct BinaryPhrasing {
    pub id: Id,
    pub syntax: BinaryOpSyntax,
}

pub struct TertiaryPhrasing {
    pub id: Id,
    pub syntax: TertiaryOpSyntax,
}

pub struct PolyadicPhrasing {
    pub id: Id,
    pub syntax: PolyadicOpSyntax,
}
pub enum OpSyntax {
    Unary(UnaryOpSyntax),
    Binary(BinaryOpSyntax),
    Tertiary(TertiaryOpSyntax),
    Polyadic(PolyadicOpSyntax),
}

pub enum UnaryOpSyntax {
    Prefix,
    Postfix,
    Circumfix
}

pub enum BinaryOpSyntax {
    Infix,
    Circumfix
}

pub enum TertiaryOpSyntax {
    Infix,
    Circumfix
}

pub enum PolyadicOpSyntax {
    Infix,
    Circumfix
}

pub struct UnaryStructure<P: Props> {
    pub phrasing: UnaryPhrasing,
    pub kid: Box<Tree<P>>,
}

pub struct BinaryStructure<P: Props> {
    pub phrasing: BinaryPhrasing,
    pub left: Box<Tree<P>>,
    pub right: Box<Tree<P>>,
}

pub struct TertiaryStructure<P: Props> {
    pub phrasing: TertiaryPhrasing,
    pub left: Box<Tree<P>>,
    pub middle: Box<Tree<P>>,
    pub right: Box<Tree<P>>,
}

pub struct PolyadicStructure<P: Props> {
    pub phrasing: PolyadicPhrasing,
    pub kids: Vec<Tree<P>>,
}

pub struct OpExpressionOld<P: Props> {
    pub op: OpOld<P>,
    pub kids: Vec<Tree<P>>,
}

impl NonIdOpOld {
    pub fn tpe(&self, kids: &[Tree<Typed>]) -> Type {
        match self {
            NonIdOpOld::BlockYieldingLastValue => {
                kids.last().map_or(Type::Unit, |kid| kid.tpe())
            }
            NonIdOpOld::BlockYieldingUnit => {
                Type::Unit
            }
        }
    }
}

impl IdOpOld<Raw> {
    pub fn new(id: Id, syntax: OpSyntaxOld) -> IdOpOld<Raw> {
        let op_tag = ();
        IdOpOld { id, syntax, op_tag }
    }
}

impl IdOpOld<Typed> {
    pub fn new(id: Id, syntax: OpSyntaxOld, op_tag: OpTag) -> IdOpOld<Typed> {
        IdOpOld::<Typed> { id, syntax, op_tag }
    }
    pub fn tpe(&self) -> Type { self.op_tag.sig.tpe() }
    pub fn key(&self) -> &OpSyntaxOld { &self.op_tag.key }
}

impl<P: Props> OpExpressionOld<P> {
    pub fn new(op: OpOld<P>, kids: Vec<Tree<P>>) -> OpExpressionOld<P> {
        OpExpressionOld { op, kids }
    }

    pub fn into_tree(self) -> Tree<P> {
        Tree::Op(self)
    }

    pub(crate) fn into_typed<R: Runtime>(self, symbols: &mut dyn SymbolTable<R>)
                                         -> Result<OpExpressionOld<Typed>, Error> {
        let mut typed_kids: Vec<Tree<Typed>> = Vec::new();
        let OpExpressionOld { op, kids } = self;
        for kid in kids.into_iter() {
            typed_kids.push(kid.into_typed(symbols)?)
        }
        match op {
            OpOld::NonId(non_id_op) => {
                match non_id_op {
                    NonIdOpOld::BlockYieldingLastValue => {
                        let non_id_op = NonIdOpOld::BlockYieldingLastValue;
                        let op = OpOld::NonId(non_id_op);
                        Ok(OpExpressionOld::new(op, typed_kids))
                    }
                    NonIdOpOld::BlockYieldingUnit => {
                        let non_id_op = NonIdOpOld::BlockYieldingUnit;
                        let op = OpOld::NonId(non_id_op);
                        Ok(OpExpressionOld::new(op, typed_kids))
                    }
                }
            }
            OpOld::Id(id_op) => {
                let IdOpOld { id, syntax, .. } = id_op;
                let kid_types: Vec<Type>=
                    typed_kids.iter().map(|kid| kid.tpe()).collect();
                let fun =
                    symbols.resolve_fun(&id, &kid_types)?
                        .ok_or_else(|| SymbolError::no_such_fun(&id))?;
                let typed_id_op = IdOpOld::<Typed>::new(id, syntax, fun);
                let typed_op = OpOld::Id(typed_id_op);
                let typed_op_expression = OpExpressionOld::new(typed_op, typed_kids);
                Ok(typed_op_expression)
            }
        }
    }
}

impl OpExpressionOld<Typed> {
    pub fn tpe(&self) -> Type {
        match &self.op {
            OpOld::NonId(non_id_op) => non_id_op.tpe(&self.kids),
            OpOld::Id(id_op) => id_op.tpe()
        }
    }
}

impl<P: Props> Display for OpExpressionOld<P> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.op {
            OpOld::NonId(non_id_op) => {
                match non_id_op {
                    NonIdOpOld::BlockYieldingLastValue => {
                        print_joined(f, &self.kids, ";\n")?;
                        writeln!(f)
                    }
                    NonIdOpOld::BlockYieldingUnit => {
                        print_ended(f, &self.kids, ";\n")
                    }
                }
            }
            OpOld::Id(id_op) => {
                match id_op.syntax {
                    OpSyntaxOld::Call => {
                        write!(f, "{}(", id_op.id)?;
                        print_joined(f, &self.kids, ", ")?;
                        write!(f, ")")
                    }
                    OpSyntaxOld::Prefix => { }
                    OpSyntaxOld::Postfix => {}
                    OpSyntaxOld::Infix => {}
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






