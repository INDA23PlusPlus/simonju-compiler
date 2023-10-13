use std::iter::from_fn;

use crate::token::Token;

#[derive(Debug, Clone, PartialEq)]
pub enum LexerError{
    InvalidInteger,
    InvalidCharacter(String),
}

pub fn tokenize_program(data: &str) -> Result<Vec<Token>, LexerError> {
    let mut tokens = vec![];
    let mut reader = data.chars().peekable();

    while let Some(c) = reader.next() {
        let token = match c {
            c if c.is_whitespace() => continue,
            // Comments
            '#' => {
                loop {
                    if let None = reader.next_if(|n| *n != '\n') {
                        break;
                    }
                }
                
                continue
            }
            // Arithmetic
            '+' => Token::Add,
            '-' => Token::Sub,
            '*' => Token::Mul,
            '/' => Token::Div,
            // Logic
            '?' => Token::Is,
            '!' => Token::Not,
            '&' => Token::And,
            '|' => Token::Or,
            // Brackets & Semicolon
            '(' => Token::LParen,
            ')' => Token::RParen,
            '{' => Token::LBrace,
            '}' => Token::RBrace,
            ';' => Token::Semicolon,
            // Comparison
            '=' => Token::Equals,
            '>' => Token::Greater,
            '<' => Token::Less,
            // Literals
            '0'..='9' => {
                let sub_reader = std::iter::once(c);

                let digits: String = sub_reader.chain(from_fn(|| reader.next_if(|n| n.is_ascii_digit()))).collect();

                match digits.parse::<i32>() { 
                    Ok(i) => Token::Constant(i),
                    Err(_) => return Err(LexerError::InvalidInteger),
                }
            },
            // Keywords & Identifiers
            'A'..='Z' | 'a'..='z' | '_' => {
                let sub_reader = std::iter::once(c);

                let string: String = sub_reader.chain(from_fn(|| reader.next_if(|l| l.is_ascii_alphabetic() || *l == '_'))).collect();

                match string.as_str() {
                    "let"   => Token::Let,
                    "be"    => Token::Be,
                    "set"   => Token::Set,
                    "to"    => Token::To,
                    "rep"   => Token::Rep,
                    "print" => Token::Print,
                    _ => Token::Variable(string),
                }
            }
            x => return Err(LexerError::InvalidCharacter(x.to_string())),
        };

        tokens.push(token);
    }

    Ok(tokens)
}