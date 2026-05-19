use crate::ast::ast;
use crate::ast::lexer::Pos;
use crate::hir::hir;
use crate::hir::noder::Noder;

fn node_identifer_expr(noder: &mut Noder, start: Pos, expr: &ast::IdentifierExpr) -> hir::Expr {
    let module = match expr.module {
        Some(name) => noder.find_module(name),
        None => noder.module.module,
    };

    noder.get_expr(
        start,
        hir::ExprValue::Identifier(hir::IdentifierExpr {
            module,
            name: expr.name,
        }),
    )
}

pub fn node_expr(noder: &mut Noder, expr: ast::Expr) -> hir::Expr {
    let start = noder.get_expr_start(expr);
    let expr_value = noder
        .module
        .ast_store
        .exprs
        .get(expr)
        .expect("failed to get ast expr value");

    match expr_value {
        ast::ExprValue::Invalid(v) => noder.get_expr(start, hir::ExprValue::Invalid(v)),
        ast::ExprValue::Identifier(v) => node_identifer_expr(noder, start, v),
        ast::ExprValue::Call(v) => todo!(""),
        ast::ExprValue::Block(v) => todo!(""),
        ast::ExprValue::Return(v) => todo!(""),
        ast::ExprValue::If(v) => todo!(""),
        ast::ExprValue::Loop(v) => todo!(""),
        ast::ExprValue::Range(v) => todo!(""),
        ast::ExprValue::Break => noder.get_expr(start, hir::ExprValue::Break),
        ast::ExprValue::Binary(v) => todo!(""),
        ast::ExprValue::IntLiteral(v) => noder.get_expr(start, hir::ExprValue::IntLiteral(*v)),
        ast::ExprValue::FloatLiteral(v) => noder.get_expr(start, hir::ExprValue::FloatLiteral(*v)),
        ast::ExprValue::BoolLiteral(v) => noder.get_expr(start, hir::ExprValue::BoolLiteral(*v)),
    }
}
