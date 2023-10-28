use std::{iter::Peekable, borrow::Cow, fmt::Display};

use crate::{token::Token, ast::*};

#[derive(Debug, Clone, PartialEq)]
pub enum ParserError {
    UnexpectedToken {
        expected: Token,
        received: Token, 
    },
    UnexpectedEnd,
    InvalidToken(Token),
}

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output: Cow<str> = match self {
            ParserError::UnexpectedToken { expected, received } => 
                Cow::Owned(format!("unexpected token: {received} expected: {expected}")),
            ParserError::UnexpectedEnd => "unexpected end".into(),
            ParserError::InvalidToken(token) => 
                Cow::Owned(format!("invalid token: {token}")),
        };

        write!(f, "{output}")
    }
}

pub fn parse_program(tokens: Vec<Token>) -> Result<Program, ParserError> {
    let mut iter = tokens.into_iter().peekable();

    let program = parse_block(&mut iter, false)?;

    Ok(Program{ block: program })
}

fn parse_block<T>(tokens: &mut Peekable<T>, is_brace_enclosed: bool) -> Result<Block, ParserError>  where T: Iterator<Item = Token> {
    let mut statements = vec![];

    if is_brace_enclosed {
        match tokens.next() {
            Some(token) => match token {
                Token::LBrace => (),
                _ => return Err(ParserError::UnexpectedToken { expected: Token::LBrace, received: token })
            },
            None => return Err(ParserError::UnexpectedEnd),
        }
    }

    'statements: loop {
        match tokens.peek() {
            Some(token) if *token == Token::RBrace && is_brace_enclosed => {
                tokens.next();
                break 'statements
            },
            None => if is_brace_enclosed {
                return Err(ParserError::UnexpectedEnd)
            } else {
                break 'statements
            },
            _ => (),
        }

        match parse_statement(tokens) {
            Ok(statement) => {
                statements.push(statement);
            }
            Err(e) => return Err(e),
        }
    }

    Ok(Block{ statements })
}

fn parse_statement<T>(tokens: &mut Peekable<T>) -> Result<Statement, ParserError> where T: Iterator<Item = Token> {
    let statement = match tokens.next() {
        Some(token) => match token {
            Token::Let      => parse_let_be(tokens)?,
            Token::Set      => parse_set_to(tokens)?,
            Token::Rep      => parse_rep(tokens)?,
            Token::Print    => parse_print(tokens)?,
            t => return Err(ParserError::InvalidToken(t)),
        },
        None => return Err(ParserError::UnexpectedEnd),
    };

    Ok(statement)
}

fn parse_let_be<T>(tokens: &mut Peekable<T>) -> Result<Statement, ParserError> where T: Iterator<Item = Token> {
    let variable = match tokens.next() {
        Some(token) => match token {
            Token::Variable(variable) => variable,
            _ => return Err(ParserError::UnexpectedToken { expected: Token::Variable("".to_string()), received: token })
        },
        None => return Err(ParserError::UnexpectedEnd),
    };

    match tokens.next() {
        Some(token) => match token {
            Token::Be => (),
            _ => return Err(ParserError::UnexpectedToken { expected: Token::Be, received: token })
        },
        None => return Err(ParserError::UnexpectedEnd),
    }

    let expr = match tokens.peek() {
        Some(_) => parse_expr(tokens)?,
        None => return Err(ParserError::UnexpectedEnd),
    };

    match tokens.next() {
        Some(token) => match token {
            Token::Semicolon => (),
            _ => return Err(ParserError::UnexpectedToken { expected: Token::Semicolon, received: token })
        },
        None => return Err(ParserError::UnexpectedEnd),
    };

    Ok(Statement::LetBe(LetBe { variable, expr }))
}

fn parse_set_to<T>(tokens: &mut Peekable<T>) -> Result<Statement, ParserError> where T: Iterator<Item = Token> {
    let variable = match tokens.next() {
        Some(token) => match token {
            Token::Variable(variable) => variable,
            _ => return Err(ParserError::UnexpectedToken { expected: Token::Variable("".to_string()), received: token })
        },
        None => return Err(ParserError::UnexpectedEnd),
    };

    match tokens.next() {
        Some(token) => match token {
            Token::To => (),
            _ => return Err(ParserError::UnexpectedToken { expected: Token::Be, received: token })
        },
        None => return Err(ParserError::UnexpectedEnd),
    }

    let expr = match tokens.peek() {
        Some(_) => parse_expr(tokens)?,
        None => return Err(ParserError::UnexpectedEnd),
    };

    match tokens.next() {
        Some(token) => match token {
            Token::Semicolon => (),
            _ => return Err(ParserError::UnexpectedToken { expected: Token::Semicolon, received: token })
        },
        None => return Err(ParserError::UnexpectedEnd),
    };

    Ok(Statement::SetTo(SetTo { variable, expr }))
}

fn parse_rep<T>(tokens: &mut Peekable<T>) -> Result<Statement, ParserError> where T: Iterator<Item = Token> {
    let expr = match tokens.peek() {
        Some(_) => parse_expr(tokens)?,
        None => return Err(ParserError::UnexpectedEnd),
    };

    let block = match tokens.peek() {
        Some(_) => parse_block(tokens, true)?,
        None => return Err(ParserError::UnexpectedEnd),
    };

    Ok(Statement::Rep(Rep { expr, block }))
}

fn parse_print<T>(tokens: &mut Peekable<T>) -> Result<Statement, ParserError> where T: Iterator<Item = Token> {
    let expr = match tokens.peek() {
        Some(_) => parse_expr(tokens)?,
        None => return Err(ParserError::UnexpectedEnd),
    };

    match tokens.next() {
        Some(token) => match token {
            Token::Semicolon => (),
            _ => return Err(ParserError::UnexpectedToken { expected: Token::Semicolon, received: token })
        },
        None => return Err(ParserError::UnexpectedEnd),
    };

    Ok(Statement::Print(Print { expr } ))
}

fn parse_expr<T>(tokens: &mut Peekable<T>) -> Result<Expr, ParserError> where T: Iterator<Item = Token> {
    let first_expr = match tokens.peek() {
        Some(_) => Expr::Sent(parse_sent(tokens)?),
        None => return Err(ParserError::UnexpectedEnd),
    };

    let expr = match tokens.peek() {
        Some(token) => match token {
            Token::And => {
                tokens.next();
                Expr::And(Box::new(first_expr), parse_sent(tokens)?)
            },
            Token::Or => {
                tokens.next();
                Expr::Or(Box::new(first_expr), parse_sent(tokens)?)
            },
            _ => first_expr,
        },
        None => first_expr,
    };

    Ok(expr)
}

fn parse_sent<T>(tokens: &mut Peekable<T>) -> Result<Sent, ParserError> where T: Iterator<Item = Token> {
    let comp = match tokens.peek() {
        Some(_) => Sent::Comp(parse_comp(tokens)?),
        None => return Err(ParserError::UnexpectedEnd),
    };

    let sent = match tokens.peek() {
        Some(token) => match token {
                Token::Equals => {
                    tokens.next();
                    Sent::Equals(Box::new(comp), parse_comp(tokens)?)
                },
                Token::Greater => {
                    tokens.next();
                    Sent::Greater(Box::new(comp), parse_comp(tokens)?)
                },
                Token::Less => {
                    tokens.next();
                    Sent::Less(Box::new(comp), parse_comp(tokens)?)
                },
                _ => comp,
        },
        None => comp,
    };

    Ok(sent)
}

fn parse_comp<T>(tokens: &mut Peekable<T>) -> Result<Comp, ParserError> where T: Iterator<Item = Token> {
    let term = match tokens.peek() {
        Some(_) => Comp::Term(parse_term(tokens)?),
        None => return Err(ParserError::UnexpectedEnd),
    };

    let comp = match tokens.peek() {
        Some(token) => match token {
            Token::Add => {
                tokens.next();
                Comp::Add(Box::new(term), parse_term(tokens)?)
            },
            Token::Sub => {
                tokens.next();
                Comp::Sub(Box::new(term), parse_term(tokens)?)
            },
            _ => term,
        },
        None => term,
    };

    Ok(comp)
}

fn parse_term<T>(tokens: &mut Peekable<T>) -> Result<Term, ParserError> where T: Iterator<Item = Token> {
    let fact = match tokens.peek() {
        Some(_) => Term::Fact(parse_fact(tokens)?),
        None => return Err(ParserError::UnexpectedEnd),
    };

    let term = match tokens.peek() {
        Some(token) => match token {
            Token::Mul => {
                tokens.next();
                Term::Mul(Box::new(fact), parse_fact(tokens)?)
            },
            Token::Div => {
                tokens.next();
                Term::Div(Box::new(fact), parse_fact(tokens)?)
            },
            _ => fact,
        },
        None => fact,
    };

    Ok(term)
}

fn parse_fact<T>(tokens: &mut Peekable<T>) -> Result<Fact, ParserError> where T: Iterator<Item = Token> {
    let fact = match tokens.peek() {
        Some(token) => match token {
            Token::Is => {
                tokens.next();
                Fact::Is(parse_prim(tokens)?)
            },
            Token::Not => {
                tokens.next();
                Fact::Not(parse_prim(tokens)?)
            },
            Token::Add => {
                tokens.next();
                Fact::Pos(parse_prim(tokens)?)
            },
            Token::Sub => {
                tokens.next();
                Fact::Neg(parse_prim(tokens)?)
            },
            Token::LParen | Token::Constant(_) | Token::Variable(_) => Fact::Prim(parse_prim(tokens)?),
            _ => return Err(ParserError::InvalidToken(token.clone())),
        }
        None => return Err(ParserError::UnexpectedEnd),
    };

    Ok(fact)
}

fn parse_prim<T>(tokens: &mut Peekable<T>) -> Result<Prim, ParserError> where T: Iterator<Item = Token> {
    let prim = match tokens.next() {
        Some(token) => match token {
            Token::LParen => {
                let expr = parse_expr(tokens)?;

                match tokens.next() {
                    Some(token) => match token {
                        Token::RParen => (),
                        _ => return Err(ParserError::UnexpectedToken{expected: Token::RParen, received: token}), 
                    },
                    None => return Err(ParserError::UnexpectedEnd),
                }

                Prim::Expr(Box::new(expr))
            },
            Token::Constant(constant) => {
                Prim::Constant(constant)
            },
            Token::Variable(variable) => Prim::Variable(variable.to_string()),
            _ => return Err(ParserError::InvalidToken(token.clone())),
        }
        None => return Err(ParserError::UnexpectedEnd),
    };

    Ok(prim)
}