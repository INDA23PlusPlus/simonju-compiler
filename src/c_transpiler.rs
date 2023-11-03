use crate::ast::*;

pub fn transpile_program_to_c(program: &Program, ) -> String {
    let c_program = transpile_block(&program.block);

    format!("#include <stdlib.h>\n#include <stdio.h>\n\nint sgn(int x) {{\n\treturn (x > 0) - (x < 0);\n}}\n\nint main() {c_program}")
}

fn transpile_block(block: &Block) -> String {
    let mut c_block = String::new();

    for statement in &block.statements {
        c_block.push_str(&transpile_statement(&statement));
    }

    format!("{{ {c_block} }}")
}

fn transpile_statement(statement: &Statement) -> String {
    match statement {
        Statement::LetBe(let_be) => 
            format!("int {} = {}; ", let_be.variable, transpile_expr(&let_be.expr)),
        Statement::SetTo(set_to) => 
            format!("{} = {}; ", set_to.variable, transpile_expr(&set_to.expr)),
        Statement::Rep(rep) => 
            format!("for (int i = 0; i < abs({}); ++i) {}", transpile_expr(&rep.expr), transpile_block(&rep.block)),
        Statement::Print(print) => 
            format!("printf(\"%i\", {}); ", transpile_expr(&print.expr)),
    }
}

fn transpile_expr(expr: &Expr) -> String {
    match expr {
        Expr::And(expr, sent) => 
            format!("({} && {})", transpile_expr(&expr), transpile_sent(&sent)),
        Expr::Or(expr, sent) =>
            format!("({} || {})", transpile_expr(&expr), transpile_sent(&sent)),
        Expr::Sent(sent) => 
            format!("{}", transpile_sent(&sent)),
    }
}

fn transpile_sent(sent: &Sent) -> String {
    match sent {
        Sent::Equals(sent, comp) => 
            format!("({} == {})", transpile_sent(&sent), transpile_comp(&comp)),
        Sent::Greater(sent, comp) => 
            format!("({} > {})", transpile_sent(&sent), transpile_comp(&comp)),
        Sent::Less(sent, comp) => 
            format!("({} < {})", transpile_sent(&sent), transpile_comp(&comp)),
        Sent::Comp(comp) =>
            format!("{}", transpile_comp(&comp)),
    }
}

fn transpile_comp(comp: &Comp) -> String {
    match comp {
        Comp::Add(comp, term) => 
            format!("({} + {})", transpile_comp(&comp), transpile_term(&term)),
        Comp::Sub(comp, term) => 
            format!("({} - {})", transpile_comp(&comp), transpile_term(&term)),
        Comp::Term(term) =>
            format!("{}", transpile_term(&term)),
    }
}

fn transpile_term(term: &Term) -> String {
    match term {
        Term::Mul(term, fact) =>
        format!("({} * {})", transpile_term(&term), transpile_fact(&fact)),
        Term::Div(term, fact) =>
        format!("({} / {})", transpile_term(&term), transpile_fact(&fact)),
        Term::Fact(fact) =>
        format!("{}", transpile_fact(&fact)),
    }
}

fn transpile_fact(fact: &Fact) -> String {
    match fact {
        Fact::Is(prim) => format!("sgn({})", transpile_prim(prim)),
        Fact::Not(prim) => format!("(!{})", transpile_prim(prim)),
        Fact::Pos(prim) => format!("(+{})", transpile_prim(prim)),
        Fact::Neg(prim) => format!("(-{})", transpile_prim(prim)),
        Fact::Prim(prim) => format!("{}", transpile_prim(prim)),
    }
}

fn transpile_prim(prim: &Prim) -> String {
    match prim {
        Prim::Expr(expr) => format!("({})", transpile_expr(&expr)),
        Prim::Constant(constant) => format!("{constant}"),
        Prim::Variable(variable) => format!("{variable}"),
    }
}