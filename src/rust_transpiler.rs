use crate::ast::*;

pub fn transpile_program_to_rust(program: &Program, ) -> String {
    let rust_program = transpile_block(&program.block);

    format!("fn program() {rust_program}")
}

fn transpile_block(block: &Block) -> String {
    let mut rust_block = String::new();

    for statement in &block.statements {
        rust_block.push_str(&transpile_statement(&statement));
    }

    format!("{{ {rust_block} }}")
}

fn transpile_statement(statement: &Statement) -> String {
    match statement {
        Statement::LetBe(let_be) => 
            format!("let mut {} = {}; ", let_be.variable, transpile_expr(&let_be.expr)),
        Statement::SetTo(set_to) => 
            format!("{} = {}; ", set_to.variable, transpile_expr(&set_to.expr)),
        Statement::Rep(rep) => 
            format!("for _ in 0..({} as i32).abs() {} ", transpile_expr(&rep.expr), transpile_block(&rep.block)),
        Statement::Print(print) => 
            format!("println!(\"{{}}\", {}); ", transpile_expr(&print.expr)),
    }
}

fn transpile_expr(expr: &Expr) -> String {
    match expr {
        Expr::And(expr, sent) => 
            format!("(({} != 0 && {} != 0) as i32)", transpile_expr(&expr), transpile_sent(&sent)),
        Expr::Or(expr, sent) =>
            format!("(({} != 0 || {} != 0) as i32)", transpile_expr(&expr), transpile_sent(&sent)),
        Expr::Sent(sent) => 
            format!("({})", transpile_sent(&sent)),
    }
}

fn transpile_sent(sent: &Sent) -> String {
    match sent {
        Sent::Equals(sent, comp) => 
            format!("(({} == {}) as i32)", transpile_sent(&sent), transpile_comp(&comp)),
        Sent::Greater(sent, comp) => 
            format!("(({} > {}) as i32)", transpile_sent(&sent), transpile_comp(&comp)),
        Sent::Less(sent, comp) => 
            format!("(({} < {}) as i32)", transpile_sent(&sent), transpile_comp(&comp)),
        Sent::Comp(comp) =>
            format!("({})", transpile_comp(&comp)),
    }
}

fn transpile_comp(comp: &Comp) -> String {
    match comp {
        Comp::Add(comp, term) => 
            format!("({} + {})", transpile_comp(&comp), transpile_term(&term)),
        Comp::Sub(comp, term) => 
            format!("({} - {})", transpile_comp(&comp), transpile_term(&term)),
        Comp::Term(term) =>
            format!("({})", transpile_term(&term)),
    }
}

fn transpile_term(term: &Term) -> String {
    match term {
        Term::Mul(term, fact) =>
        format!("({} * {})", transpile_term(&term), transpile_fact(&fact)),
        Term::Div(term, fact) =>
        format!("({} / {})", transpile_term(&term), transpile_fact(&fact)),
        Term::Fact(fact) =>
        format!("({})", transpile_fact(&fact)),
    }
}

fn transpile_fact(fact: &Fact) -> String {
    match fact {
        Fact::Is(prim) => format!("(({} as i32).signum())", transpile_prim(prim)),
        Fact::Not(prim) => format!("(!{} as i32)", transpile_prim(prim)),
        Fact::Pos(prim) => format!("({})", transpile_prim(prim)),
        Fact::Neg(prim) => format!("(-{})", transpile_prim(prim)),
        Fact::Prim(prim) => format!("({})", transpile_prim(prim)),
    }
}

fn transpile_prim(prim: &Prim) -> String {
    match prim {
        Prim::Expr(expr) => format!("({})", transpile_expr(&expr)),
        Prim::Constant(constant) => format!("({constant})"),
        Prim::Variable(variable) => format!("({variable})"),
    }
}