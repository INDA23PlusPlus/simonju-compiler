use std::iter::from_fn;

use crate::token::Token;

#[derive(Debug, Clone, PartialEq)]
pub enum LexerError{
    InvalidInteger,
    InvalidCharacter,
}

pub fn tokenize(data: &str) -> Result<Vec<Token>, LexerError> {
    let mut tokens = vec![];
    let mut reader = data.chars().peekable();

    while let Some(c) = reader.next() {
        println!("reading character");
        let token = match c {
            c if c.is_whitespace() => continue,
            // Arithmetic
            '+' => Token::Add,
            '-' => Token::Sub,
            '*' => Token::Mul,
            '/' => Token::Div,
            // Logic
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
            '1'..='9' => {
                let sub_reader = std::iter::once(c);

                let digits: String = sub_reader.chain(from_fn(|| reader.next_if(|n| n.is_ascii_digit()))).collect();

                match digits.parse::<i32>() { 
                    Ok(i) => Token::Integer(i),
                    Err(_) => return Err(LexerError::InvalidInteger),
                }
            },
            // Keywords & Identifiers
            'A'..='Z' | 'a'..='z' | '_' => {
                let sub_reader = std::iter::once(c);

                let string: String = sub_reader.chain(from_fn(|| reader.next_if(|l| l.is_ascii_alphabetic() || *l == '_'))).collect();

                match string.as_str() {
                    "let" => Token::Let,
                    "eat" => Token::Eat,
                    "be" => Token::Be,
                    "if" => Token::If,
                    "do" => Token::Do,
                    "out" => Token::Out,
                    _ => Token::Identifier(string),
                }
            }
            _ => return Err(LexerError::InvalidCharacter),
        };

        tokens.push(token);
    }

    tokens.push(Token::EOF);

    Ok(tokens)
}