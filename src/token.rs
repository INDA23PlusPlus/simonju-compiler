use std::{fmt::Display, borrow::Cow};

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    /* Arithmetic Operators */
    Add,    // +
    Sub,    // -
    Mul,    // *
    Div,    // /
    // Mod,      // %

    /* Logical Operators */
    Not,    // !
    And,    // &
    Or,     // |

    /* Comparison Operators */
    Equals,     // =
    Less,       // <
    Greater,    // >

    /* Keywords */
    Let,    // Declare
    Eat,    // Move
    Be,     // Copy
    If,     // Branch
    Do,     // Loop
    Out,    // Print

    /* Control & Ordering */
    LParen,  // (
    RParen, // )
    LBrace,  // {
    RBrace, // }
    Semicolon,  // ;

    /* Literals */
    Integer(i32),   // ex. 20

    /* Variables */
    Identifier(String),  // ex. x

    /* Helpers */
    EOF,
}

impl From<i32> for Token {
    fn from(value: i32) -> Self {
        Self::Integer(value)
    }
}

impl From<String> for Token {
    fn from(value: String) -> Self {
        Self::Identifier(value)
    }
}

impl<'a> From<&'a str> for Token {
    fn from(other: &'a str) -> Self {
        Self::Identifier(other.to_string())
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let integer: i32;
        let output: Cow<str> = match self {
            Token::Add => "add".into(),
            Token::Sub => "sub".into(),
            Token::Mul => "mul".into(),
            Token::Div => "div".into(),
            Token::Not => "not".into(),
            Token::And => "and".into(),
            Token::Or => "or".into(),
            Token::Equals => "equals".into(),
            Token::Less => "less".into(),
            Token::Greater => "greater".into(),
            Token::Let => "let".into(),
            Token::Eat => "eat".into(),
            Token::Be => "be".into(),
            Token::If => "if".into(),
            Token::Do => "do".into(),
            Token::Out => "out".into(),
            Token::LParen => "openparen".into(),
            Token::RParen => "closeparen".into(),
            Token::LBrace => "openbrace".into(),
            Token::RBrace => "closebrace".into(),
            Token::Semicolon => "semicolon".into(),
            Token::Integer(i) => Cow::Owned(format!("integer({})", i)),
            Token::Identifier(s) => Cow::Owned(format!("identifier({})", s)),
            Token::EOF => "eof".into(),
        };

        write!(f, "<{output}>")
    }
}