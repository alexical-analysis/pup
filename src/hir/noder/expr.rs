use crate::ast::ast;
use crate::ast::lexer::Pos;
use crate::hir::hir;
use crate::hir::noder::Noder;

fn node_call_expr(noder: &mut Noder, start: Pos, expr: &ast::CallExpr) -> hir::Expr {
    let func = node_expr(noder, expr.func);
    let mut args = vec![];
    for &a in &expr.args {
        let expr = node_expr(noder, a);
        args.push(expr);
    }
    noder.get_expr(start, hir::ExprValue::Call(hir::CallExpr { func, args }))
}

fn node_loop_expr(noder: &mut Noder, start: Pos, expr: &ast::LoopExpr) -> hir::Expr {
    let mut exprs = vec![];
    for &e in &expr.body.exprs {
        exprs.push(node_expr(noder, e));
    }
    noder.get_expr(
        start,
        hir::ExprValue::Loop(hir::LoopExpr {
            body: hir::BlockExpr { exprs },
        }),
    )
}

fn node_binary_expr(noder: &mut Noder, start: Pos, expr: &ast::BinaryExpr) -> hir::Expr {
    let left = node_expr(noder, expr.left);
    let right = node_expr(noder, expr.right);
    let operator = match expr.operator {
        ast::Operator::Plus => hir::Operator::Plus,
        ast::Operator::Minus => hir::Operator::Minus,
        ast::Operator::Multiply => hir::Operator::Multiply,
        ast::Operator::Divide => hir::Operator::Divide,
        ast::Operator::Equal => hir::Operator::Equal,
        ast::Operator::LessThan => hir::Operator::LessThan,
    };
    noder.get_expr(
        start,
        hir::ExprValue::Binary(hir::BinaryExpr { left, operator, right }),
    )
}

fn node_range_expr(noder: &mut Noder, start: Pos, expr: &ast::RangeExpr) -> hir::Expr {
    let start_expr = node_expr(noder, expr.start);
    let end_expr = node_expr(noder, expr.end);
    noder.get_expr(
        start,
        hir::ExprValue::Range(hir::RangeExpr {
            start: start_expr,
            end: end_expr,
        }),
    )
}

fn node_if_expr(noder: &mut Noder, start: Pos, expr: &ast::IfExpr) -> hir::Expr {
    let check = node_expr(noder, expr.check);
    let mut exprs = vec![];
    for &e in &expr.success.exprs {
        exprs.push(node_expr(noder, e));
    }
    noder.get_expr(
        start,
        hir::ExprValue::If(hir::IfExpr {
            check,
            success: hir::BlockExpr { exprs },
        }),
    )
}

fn node_return_expr(noder: &mut Noder, start: Pos, expr: &ast::ReturnExpr) -> hir::Expr {
    let value = expr.value.map(|e| node_expr(noder, e));
    noder.get_expr(start, hir::ExprValue::Return(hir::ReturnExpr { value }))
}

fn node_block_expr(noder: &mut Noder, start: Pos, expr: &ast::BlockExpr) -> hir::Expr {
    let mut exprs = vec![];
    for &e in &expr.exprs {
        exprs.push(node_expr(noder, e));
    }
    noder.get_expr(start, hir::ExprValue::Block(hir::BlockExpr { exprs }))
}

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
        ast::ExprValue::Call(v) => node_call_expr(noder, start, v),
        ast::ExprValue::Block(v) => node_block_expr(noder, start, v),
        ast::ExprValue::Return(v) => node_return_expr(noder, start, v),
        ast::ExprValue::If(v) => node_if_expr(noder, start, v),
        ast::ExprValue::Loop(v) => node_loop_expr(noder, start, v),
        ast::ExprValue::Range(v) => node_range_expr(noder, start, v),
        ast::ExprValue::Break => noder.get_expr(start, hir::ExprValue::Break),
        ast::ExprValue::Binary(v) => node_binary_expr(noder, start, v),
        ast::ExprValue::IntLiteral(v) => noder.get_expr(start, hir::ExprValue::IntLiteral(*v)),
        ast::ExprValue::FloatLiteral(v) => noder.get_expr(start, hir::ExprValue::FloatLiteral(*v)),
        ast::ExprValue::BoolLiteral(v) => noder.get_expr(start, hir::ExprValue::BoolLiteral(*v)),
    }
}
