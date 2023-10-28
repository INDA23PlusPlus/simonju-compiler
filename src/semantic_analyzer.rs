// Scope: variables have lifetime scope.
// Lifetime: variables live from declaration to program end unless explicitly destroyed.

use std::{collections::HashSet, fmt::Display, borrow::Cow};
use crate::ast::*;

#[derive(Debug, Clone, PartialEq)]
pub enum SemanticAnalyzerError {
    RedeclaredVariable(String),
    UndeclaredVariable(String),
    DeclaredLocalVariable(String), // temporary solution to scope: everythin must be a global
}

impl Display for SemanticAnalyzerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output: Cow<str> = match self {
            SemanticAnalyzerError::RedeclaredVariable(variable) => 
                Cow::Owned(format!("redeclared variable: {variable}")),
            SemanticAnalyzerError::UndeclaredVariable(variable) => 
                Cow::Owned(format!("undeclared variable: {variable}")),
            SemanticAnalyzerError::DeclaredLocalVariable(variable) => 
                Cow::Owned(format!("declared local variable: {variable}")),

        };

        write!(f, "{output}")
    }
}

pub fn validate_program(program: &Program) -> Result<(), SemanticAnalyzerError> {
    let mut symbol_table = HashSet::new();

    validate_block(&program.block, &mut symbol_table, true)?;

    Ok(())
}

fn validate_block(block: &Block, symbol_table: &mut HashSet<String>, is_global_scope: bool) -> Result<(), SemanticAnalyzerError> {
    for statement in &block.statements {
        validate_statement(&statement, symbol_table, is_global_scope)?;
    }

    Ok(())
}

fn validate_statement(statement: &Statement, symbol_table: &mut HashSet<String>, is_global_scope: bool) -> Result<(), SemanticAnalyzerError> {
    match statement {
        Statement::LetBe(let_be) => validate_let_be(let_be, symbol_table, is_global_scope)?,
        Statement::SetTo(set_to) => validate_set_to(set_to, symbol_table)?,
        Statement::Rep(rep)        => validate_rep(rep, symbol_table)?,
        Statement::Print(print)  => validate_print(print, symbol_table)?,
    }

    Ok(())
}

fn validate_let_be(let_be: &LetBe, symbol_table: &mut HashSet<String>, is_global_scope: bool) -> Result<(), SemanticAnalyzerError> {
    if symbol_table.contains(&let_be.variable) {
        return Err(SemanticAnalyzerError::RedeclaredVariable(let_be.variable.to_owned()));
    }

    if !is_global_scope {
        return Err(SemanticAnalyzerError::DeclaredLocalVariable(let_be.variable.to_owned()));
    }

    validate_expr(&let_be.expr, symbol_table)?;

    symbol_table.insert(let_be.variable.to_owned());

    Ok(())
}

fn validate_set_to(set_to: &SetTo, symbol_table: &mut HashSet<String>) -> Result<(), SemanticAnalyzerError> {
    if !symbol_table.contains(&set_to.variable) {
        return Err(SemanticAnalyzerError::UndeclaredVariable(set_to.variable.to_owned()));
    }

    validate_expr(&set_to.expr, symbol_table)?;

    Ok(())
}

fn validate_rep(rep: &Rep, symbol_table: &mut HashSet<String>) -> Result<(), SemanticAnalyzerError> {
    validate_expr(&rep.expr, symbol_table)?;

    validate_block(&rep.block, symbol_table, false)?;

    Ok(())
}

fn validate_print(print: &Print, symbol_table: &mut HashSet<String>) -> Result<(), SemanticAnalyzerError> {
    validate_expr(&print.expr, symbol_table)?;

    Ok(())
}

fn validate_expr(expr: &Expr, symbol_table: &mut HashSet<String>) -> Result<(), SemanticAnalyzerError> {
    match expr {
        Expr::And(expr, sent) | 
        Expr::Or(expr, sent) => {
            validate_expr(&expr, symbol_table)?;
            validate_sent(&sent, symbol_table)?;
        },
        Expr::Sent(sent) => validate_sent(&sent, symbol_table)?,
    }

    Ok(())
}

fn validate_sent(sent: &Sent, symbol_table: &mut HashSet<String>) -> Result<(), SemanticAnalyzerError> {
    match sent {
        Sent::Equals(sent, comp) |
        Sent::Greater(sent, comp) |
        Sent::Less(sent, comp) => {
            validate_sent(&sent, symbol_table)?;
            validate_comp(&comp, symbol_table)?;
        },
        Sent::Comp(comp) => validate_comp(&comp, symbol_table)?,
    }

    Ok(())
}

fn validate_comp(comp: &Comp, symbol_table: &mut HashSet<String>) -> Result<(), SemanticAnalyzerError> {
    match comp {
        Comp::Add(comp, term) |
        Comp::Sub(comp, term) => {
            validate_comp(&comp, symbol_table)?;
            validate_term(&term, symbol_table)?;
        },
        Comp::Term(term) => validate_term(&term, symbol_table)?,
    }

    Ok(())
}

fn validate_term(term: &Term, symbol_table: &mut HashSet<String>) -> Result<(), SemanticAnalyzerError> {
    match term {
        Term::Mul(term, fact) |
        Term::Div(term, fact) => {
            validate_term(&term, symbol_table)?;
            validate_fact(&fact, symbol_table)?;
        },
        Term::Fact(fact) => validate_fact(&fact, symbol_table)?,
    }

    Ok(())
}

fn validate_fact(fact: &Fact, symbol_table: &mut HashSet<String>) -> Result<(), SemanticAnalyzerError> {
    match fact {
        Fact::Is(prim) |
        Fact::Not(prim) |
        Fact::Pos(prim) |
        Fact::Neg(prim) |
        Fact::Prim(prim) => validate_prim(&prim, symbol_table)?,
    }

    Ok(())
}

fn validate_prim(prim: &Prim, symbol_table: &mut HashSet<String>) -> Result<(), SemanticAnalyzerError> {
    match prim {
        Prim::Expr(expr) => validate_expr(&expr, symbol_table)?,
        Prim::Constant(_) => (),
        Prim::Variable(variable) => {
            if !symbol_table.contains(variable) {
                return Err(SemanticAnalyzerError::UndeclaredVariable(variable.to_owned()));
            }
        }
    }

    Ok(())
}

