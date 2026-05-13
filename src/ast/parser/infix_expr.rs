use std::collections::HashMap;
use std::rc::Rc;

use crate::ast::ast::{CallExpr, Expr, ExprValue, RangeExpr};
use crate::ast::lexer::{Lexer, Token, Ty};
use crate::ast::parser::Parser;

pub trait InfixExprParselet {
    fn parse(&self, parser: &mut Parser, lexer: &mut Lexer, left: Expr, token: Token) -> Expr;

    fn precedence(&self) -> Precedence;
}

// Operator precedence levels.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Precedence {
    Base,
    Equality,
    Range,
    Comparison,
    Addition,
    Multiplication,
    Call,
}

pub struct CallExprParselet;

impl InfixExprParselet for CallExprParselet {
    fn parse(&self, parser: &mut Parser, lexer: &mut Lexer, left: Expr, token: Token) -> Expr {
        let mut args = vec![];

        loop {
            let next = lexer.peek().clone();
            if next.ty == Ty::CloseParen {
                lexer.next(parser.ctx_mut());
                break;
            }

            if next.ty == Ty::Eof {
                lexer.recover_until_expr(parser.ctx_mut());
                return parser.get_expr(next, ExprValue::Invalid("unclosed argument list"));
            }

            let arg = parser.parse_expr(lexer, Precedence::Base);
            args.push(arg);

            let sep = lexer.peek().clone();
            match sep.ty {
                Ty::Comma => {
                    lexer.next(parser.ctx_mut());
                }
                Ty::CloseParen => {
                    lexer.next(parser.ctx_mut());
                    break;
                }
                _ => {
                    lexer.recover_until_expr(parser.ctx_mut());
                    return parser.get_expr(
                        sep,
                        ExprValue::Invalid("expected ',' or ')' in argument list"),
                    );
                }
            }
        }

        parser.get_expr(token, ExprValue::Call(CallExpr { func: left, args }))
    }

    fn precedence(&self) -> Precedence {
        Precedence::Call
    }
}

pub struct RangeExprParselet;

impl InfixExprParselet for RangeExprParselet {
    fn parse(&self, parser: &mut Parser, lexer: &mut Lexer, left: Expr, token: Token) -> Expr {
        let end = parser.parse_expr(lexer, Precedence::Range);
        parser.get_expr(token, ExprValue::Range(RangeExpr { start: left, end }))
    }

    fn precedence(&self) -> Precedence {
        Precedence::Range
    }
}

pub struct BinaryExprExprParselet;

pub fn expr_infix_parselets() -> HashMap<Ty, Rc<dyn InfixExprParselet>> {
    HashMap::from([
        (
            Ty::OpenParen,
            Rc::new(CallExprParselet {}) as Rc<dyn InfixExprParselet>,
        ),
        (
            Ty::RangeOperator,
            Rc::new(RangeExprParselet {}) as Rc<dyn InfixExprParselet>,
        ),
    ])
}
