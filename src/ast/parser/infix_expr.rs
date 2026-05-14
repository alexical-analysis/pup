use std::collections::HashMap;
use std::rc::Rc;

use crate::ast::ast::{BinaryExpr, CallExpr, Expr, ExprValue, Operator, RangeExpr};
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

pub struct BinaryExprExprParselet {
    operator: Operator,
    precedence: Precedence,
}

impl InfixExprParselet for BinaryExprExprParselet {
    fn parse(&self, parser: &mut Parser, lexer: &mut Lexer, left: Expr, token: Token) -> Expr {
        let right = parser.parse_expr(lexer, Precedence::Base);

        parser.get_expr(
            token,
            ExprValue::Binary(BinaryExpr {
                left,
                operator: self.operator,
                right,
            }),
        )
    }

    fn precedence(&self) -> Precedence {
        self.precedence
    }
}

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
        (
            Ty::Plus,
            Rc::new(BinaryExprExprParselet {
                operator: Operator::Plus,
                precedence: Precedence::Addition,
            }) as Rc<dyn InfixExprParselet>,
        ),
        (
            Ty::Minus,
            Rc::new(BinaryExprExprParselet {
                operator: Operator::Minus,
                precedence: Precedence::Addition,
            }) as Rc<dyn InfixExprParselet>,
        ),
        (
            Ty::Multiply,
            Rc::new(BinaryExprExprParselet {
                operator: Operator::Multiply,
                precedence: Precedence::Multiplication,
            }) as Rc<dyn InfixExprParselet>,
        ),
        (
            Ty::Divide,
            Rc::new(BinaryExprExprParselet {
                operator: Operator::Divide,
                precedence: Precedence::Multiplication,
            }) as Rc<dyn InfixExprParselet>,
        ),
        (
            Ty::EqualEqual,
            Rc::new(BinaryExprExprParselet {
                operator: Operator::Equal,
                precedence: Precedence::Equality,
            }) as Rc<dyn InfixExprParselet>,
        ),
        (
            Ty::LessThan,
            Rc::new(BinaryExprExprParselet {
                operator: Operator::LessThan,
                precedence: Precedence::Comparison,
            }) as Rc<dyn InfixExprParselet>,
        ),
    ])
}
