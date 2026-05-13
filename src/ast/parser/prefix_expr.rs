use crate::ast::ast::{Expr, ExprValue, IdentifierExpr};
use crate::ast::lexer::{Lexer, Token, Ty};
use crate::ast::parser::Parser;

fn parse_identifier(parser: &mut Parser, lexer: &mut Lexer, token: Token) -> Expr {
    let is_mod = lexer.peek();
    if is_mod.ty != Ty::ModuleOperator {
        return parser.get_expr(
            token,
            ExprValue::Identifier(IdentifierExpr {
                module: None,
                name: token.lexeme,
            }),
        );
    }

    lexer.next(parser.ctx_mut());
    let ident_name = lexer.next(parser.ctx_mut());
    if ident_name.ty != Ty::Identifier {
        lexer.recover_until_expr(parser.ctx_mut());

        return parser.get_expr(ident_name, ExprValue::Invalid(""));
    }

    parser.get_expr(
        token,
        ExprValue::Identifier(IdentifierExpr {
            module: Some(token.lexeme),
            name: (ident_name.lexeme),
        }),
    )
}

// pub struct ParamExprParselet;
// pub struct BlockExprParselet;
// pub struct ReturnExprParselet;
// pub struct IfExprParselet;
// pub struct LoopExprParselet;
// pub struct RangeExprParselet;
// pub struct BreakExprParselet;
// pub struct BinaryExprExprParselet;
// pub struct IntLiteralExprParselet;
// pub struct FloatLiteralExprParselet;
// pub struct BoolLiteralExprParselet;

pub fn parse_prefix_expr(parser: &mut Parser, lexer: &mut Lexer, token: Token) -> Expr {
    match token.ty {
        Ty::Identifier => parse_identifier(parser, lexer, token),
        _ => todo!("parse_prefix_expr"),
    }
}
