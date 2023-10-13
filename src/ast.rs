use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Program {
    pub block: Block,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Block {
    pub statements: Vec<Statement>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Statement {
    LetBe(LetBe),
    SetTo(SetTo),
    Rep(Rep),
    Print(Print),
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct LetBe {
    pub variable: String,
    pub expr: Expr,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SetTo {
    pub variable: String,
    pub expr: Expr,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Rep {
    pub expr: Expr,
    pub block: Block,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Print {
    pub expr: Expr,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Expr {
    And(Box<Expr>, Sent),
    Or(Box<Expr>, Sent),
    Sent(Sent),
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Comp {
    Add(Box<Comp>, Term),
    Sub(Box<Comp>, Term),
    Term(Term),
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Sent {
    Equals(Box<Sent>, Comp),
    Greater(Box<Sent>, Comp),
    Less(Box<Sent>, Comp),
    Comp(Comp),
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Term {
    Mul(Box<Term>, Fact),
    Div(Box<Term>, Fact),
    Fact(Fact),
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Fact {
    Is(Prim),
    Not(Prim),
    Pos(Prim),
    Neg(Prim),
    Prim(Prim),
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Prim {
    Expr(Box<Expr>),
    Constant(Integer),
    Variable(String),
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Integer {
    pub value: i32,
}