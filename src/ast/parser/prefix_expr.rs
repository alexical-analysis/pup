use crate::ast::ast::{Expr, ExprValue, IdentifierExpr, IfExpr, ReturnExpr};
use crate::ast::lexer::{Lexer, Token, Ty};
use crate::ast::parser::Parser;
use crate::ast::parser::infix_expr::Precedence;

fn parse_identifier_expr(parser: &mut Parser, lexer: &mut Lexer, token: Token) -> Expr {
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

fn parse_return_expr(parser: &mut Parser, lexer: &mut Lexer, token: Token) -> Expr {
    if lexer.peek().ty == Ty::Semicolon {
        return parser.get_expr(token, ExprValue::Return(ReturnExpr { value: None }));
    }

    let value = parser.parse_expr(lexer, Precedence::Base);
    return parser.get_expr(token, ExprValue::Return(ReturnExpr { value: Some(value) }));
}

fn parse_if_expr(parser: &mut Parser, lexer: &mut Lexer, token: Token) -> Expr {
    let check = parser.parse_expr(lexer, Precedence::Base);

    let open_brace = lexer.next(parser.ctx_mut());
    if open_brace.ty != Ty::OpenBrace {
        lexer.recover_until_expr(parser.ctx_mut());
        return parser.get_expr(open_brace, ExprValue::Invalid("if expression is missing a body"));
    }

    let success = parser.parse_body(lexer);

    parser.get_expr(token, ExprValue::If(IfExpr { check, success }))
}
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
        Ty::Identifier => parse_identifier_expr(parser, lexer, token),
        Ty::ReturnKeyword => parse_return_expr(parser, lexer, token),
        Ty::IfKeyword => parse_if_expr(parser, lexer, token),
        _ => todo!("parse_prefix_expr"),
    }
}
