use std::fmt::{Display, DebugList};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Program {
    pub block: Block,
}

impl Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "program:\n\u{02514}{}", self.block)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Block {
    pub statements: Vec<Statement>,
}

impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut string = "block:\n \u{02514}".to_string();

        for statement in &self.statements {
            string.push_str(&format!("{statement}"));
        };

        write!(f, "{string}")
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Statement {
    LetBe(LetBe),
    SetTo(SetTo),
    Rep(Rep),
    Print(Print),
}

impl Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Statement::LetBe(let_be) => write!(f, "statement: {}", let_be),
            Statement::SetTo(set_to) => write!(f, "statement: {}", set_to),
            Statement::Rep(rep) => write!(f, "statement: {}", rep),
            Statement::Print(print) => write!(f, "statement: {}", print),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct LetBe {
    pub variable: String,
    pub expr: Expr,
}

impl Display for LetBe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "let:\n- {}\n- be \n- {}", self.variable, self.expr)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SetTo {
    pub variable: String,
    pub expr: Expr,
}

impl Display for SetTo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "set:\n- {}\n- to \n- {}", self.variable, self.expr)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Rep {
    pub expr: Expr,
    pub block: Block,
}

impl Display for Rep {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "rep:\n- {}\n- {{\n- {}\n- }}", self.expr, self.block)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Print {
    pub expr: Expr,
}

impl Display for Print {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "print({})", self.expr)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Expr {
    And(Box<Expr>, Sent),
    Or(Box<Expr>, Sent),
    Sent(Sent),
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::And(expr, sent) => write!(f, "expr:\n- {}\n- & \n- {}", expr, sent),
            Expr::Or(expr, sent) => write!(f, "expr:\n- {}\n- | \n- {}", expr, sent),
            Expr::Sent(sent) => write!(f, "expr:\n- {}", sent),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Sent {
    Equals(Box<Sent>, Comp),
    Greater(Box<Sent>, Comp),
    Less(Box<Sent>, Comp),
    Comp(Comp),
}

impl Display for Sent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Sent::Equals(sent, comp) => write!(f, "sent:\n- {}\n- = \n- {}", sent, comp),
            Sent::Greater(sent, comp) => write!(f, "sent:\n- {}\n- > \n- {}", sent, comp),
            Sent::Less(sent, comp) => write!(f, "sent:\n- {}\n- < \n- {}", sent, comp),
            Sent::Comp(comp) => write!(f, "sent:\n- {}", comp),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Comp {
    Add(Box<Comp>, Term),
    Sub(Box<Comp>, Term),
    Term(Term),
}

impl Display for Comp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Comp::Add(comp, term) => write!(f, "comp:\n- {}\n- + \n- {}", comp, term),
            Comp::Sub(comp, term) => write!(f, "comp:\n- {}\n- - \n- {}", comp, term),
            Comp::Term(term) => write!(f, "comp:\n- {}", term),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Term {
    Mul(Box<Term>, Fact),
    Div(Box<Term>, Fact),
    Fact(Fact),
}

impl Display for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Term::Mul(term, fact) => write!(f, "term:\n- {}\n- * \n- {}", term, fact),
            Term::Div(term, fact) => write!(f, "term:\n- {}\n- / \n- {}", term, fact),
            Term::Fact(fact) => write!(f, "term:\n- {}", fact),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Fact {
    Is(Prim),
    Not(Prim),
    Pos(Prim),
    Neg(Prim),
    Prim(Prim),
}

impl Display for Fact {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Fact::Is(p) => write!(f, "fact:\n- ?{}", p),
            Fact::Not(p) => write!(f, "fact\n- !{}", p),
            Fact::Pos(p) => write!(f, "fact\n- +{}", p),
            Fact::Neg(p) => write!(f, "fact\n- -{}", p),
            Fact::Prim(p) => write!(f, "fact\n- {}", p),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Prim {
    Expr(Box<Expr>),
    Constant(i32),
    Variable(String),
}

impl Display for Prim {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Prim::Expr(e) => write!(f, "prim:\n- ({})", **e),
            Prim::Constant(c) => write!(f, "prim:\n- constant: {}", c),
            Prim::Variable(v) => write!(f, "prim:\n- variable: {}", v),
        }
    }
}

/* INTEGER PRIMITIVE */

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Integer {
    pub value: i32,
}

impl Display for Integer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<i32> for Integer {
    fn from(value: i32) -> Self {
        Integer { value }
    }
}