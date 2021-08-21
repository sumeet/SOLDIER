use crate::parser::{Assignment, Expr, FunctionCall, Program, Ref, While};

pub fn assemble_program(program: &Program) -> String {
    let mut assembled = String::new();
    assemble_expr(&mut assembled, &Expr::Block(program.block.clone()));
    assembled
}

fn assemble_expr(assembled: &mut String, expr: &Expr) {
    match expr {
        Expr::Block(block) => {
            for expr in &block.0 {
                assemble_expr(assembled, expr);
                assembled.push_str("\n");
            }
        }
        Expr::Comment(ref body) => {
            let mut lines = body.lines().peekable();
            while let Some(line) = lines.next() {
                assembled.push_str("// ");
                assembled.push_str(line);
                if lines.peek().is_some() {
                    assembled.push_str("\n");
                }
            }
        }
        Expr::Assignment(Assignment { name, expr }) => {
            assembled.push_str("let ");
            assembled.push_str(name);
            assembled.push_str(" = ");
            assemble_expr(assembled, expr);
        }
        Expr::IntLiteral(n) => assembled.push_str(&n.to_string()),
        Expr::Ref(r#ref) => assemble_ref(r#ref, assembled),
        Expr::FunctionCall(FunctionCall { r#ref, args }) => {
            assemble_ref(r#ref, assembled);
            assembled.push_str("(");
            if let Some((last, init)) = args.split_last() {
                for arg in init {
                    assemble_expr(assembled, arg);
                    assembled.push_str(",");
                }
                assemble_expr(assembled, last);
            }
            assembled.push_str(")");
        }
        Expr::While(While { cond, block }) => {
            assembled.push_str("while (");
            assemble_expr(assembled, cond);
            assembled.push_str(") {\n");
            assemble_expr(assembled, &Expr::Block(block.clone()));
            assembled.push_str("\n}");
        }
    }
}

fn assemble_ref(r#ref: &Ref, assembled: &mut String) {
    match r#ref {
        Ref::CommentRef(s) => {
            assembled.push_str("#");
            assembled.push_str(s);
        }
        Ref::VarRef(s) => assembled.push_str(s),
    }
}
