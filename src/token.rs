use std::{fmt::Display, borrow::Cow};

#[derive(Debug, Clone, PartialEq,)]
pub enum Token {
    /* Arithmetic Operators */
    Add,    // +
    Sub,    // -
    Mul,    // *
    Div,    // /
    // Mod,      // %

    /* Logical Operators */
    Is,     // ?
    Not,    // !
    And,    // &
    Or,     // |

    /* Comparison Operators */
    Equals,     // =
    Less,       // <
    Greater,    // >

    /* Keywords */
    Let, Be,    // Declare
    Set, To,    // Assign
    Rep,        // Loop
    Print,      // Print
    // Eat      // Drop

    /* Control & Ordering */
    LParen,     // (
    RParen,     // )
    LBrace,     // {
    RBrace,     // }
    Semicolon,  // ;

    /* Data */
    Constant(i32),      // ex. 20
    Variable(String),   // ex. x
}

impl From<i32> for Token {
    fn from(value: i32) -> Self {
        Self::Constant(value)
    }
}

impl From<String> for Token {
    fn from(value: String) -> Self {
        Self::Variable(value)
    }
}

impl<'a> From<&'a str> for Token {
    fn from(other: &'a str) -> Self {
        Self::Variable(other.to_string())
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output: Cow<str> = match self {
            Token::Add => "add".into(),
            Token::Sub => "sub".into(),
            Token::Mul => "mul".into(),
            Token::Div => "div".into(),
            Token::Is => "is".into(),
            Token::Not => "not".into(),
            Token::And => "and".into(),
            Token::Or => "or".into(),
            Token::Equals => "equals".into(),
            Token::Less => "less".into(),
            Token::Greater => "greater".into(),
            Token::Let => "let".into(),
            Token::Be => "be".into(),
            Token::Set => "set".into(),
            Token::To => "to".into(),
            Token::Rep => "rep".into(),
            Token::Print => "print".into(),
            Token::LParen => "l_paren".into(),
            Token::RParen => "r_paren".into(),
            Token::LBrace => "l_brace".into(),
            Token::RBrace => "r_brace".into(),
            Token::Semicolon => "semicolon".into(),
            Token::Constant(i) => Cow::Owned(format!("constant({})", i)),
            Token::Variable(s) => Cow::Owned(format!("variable({})", s)),
        };

        write!(f, "<{output}>")
    }
}