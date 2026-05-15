mod decl;
mod infix_expr;
mod prefix_expr;

use std::collections::HashMap;
use std::rc::Rc;

use crate::ast::ast::{BlockExpr, Decl, Expr};
use crate::ast::lexer::{Token, Ty};
use crate::ast::parser::decl::parse_decl;
use crate::ast::parser::infix_expr::{InfixExprParselet, Precedence};
use crate::ast::parser::prefix_expr::parse_prefix_expr;
use crate::compiler::module::AstModule;
use crate::compiler::str_store::{MStr, StrStore};
use crate::types::unchecked_ty::UncheckedTy;

use super::ast::{DeclValue, ExprValue};
use super::lexer::Lexer;

pub struct Parser<'m, 'ctx> {
    module: &'m mut AstModule<'ctx>,
    infix_parselets: HashMap<Ty, Rc<dyn InfixExprParselet>>,
}

impl<'m, 'ctx> Parser<'m, 'ctx> {
    pub fn get_decl(&mut self, start: Token, decl: DeclValue) -> Decl {
        self.module.get_decl(start, decl)
    }

    pub fn get_expr(&mut self, start: Token, expr: ExprValue) -> Expr {
        self.module.get_expr(start, expr)
    }

    pub fn parse(&mut self, source: &str) {
        let mut lexer = Lexer::new(self.str_store(), source);

        let mut decls = vec![];
        loop {
            let token = lexer.peek().clone();
            if token.ty == Ty::Eof {
                break;
            }

            let decl = parse_decl(self, &mut lexer, token);
            decls.push(decl);

            // expect a semicolon after each declaration
            let end = lexer.next(self.str_store());
            if end.ty != Ty::Semicolon {
                lexer.recover_until_decl(self.str_store());
                let decl = self.get_decl(
                    end,
                    DeclValue::Invalid("missing semicolon/new line after declaration"),
                );
                decls.push(decl);
            }
        }
    }

    fn parse_expr(&mut self, lexer: &mut Lexer, min_precedence: Precedence) -> Expr {
        let token = lexer.next(self.str_store());

        let mut left = parse_prefix_expr(self, lexer, token);

        loop {
            let token = lexer.peek();
            if token.ty == Ty::Eof || token.ty == Ty::Semicolon {
                break;
            }

            let infix_parselet = match self.infix_parselets.get(&token.ty) {
                Some(parselet) => parselet.clone(),
                None => break,
            };

            if infix_parselet.precedence() <= min_precedence {
                break;
            }

            let token = lexer.next(self.str_store());
            left = infix_parselet.parse(self, lexer, left, token);
        }

        left
    }

    fn parse_body(&mut self, lexer: &mut Lexer) -> BlockExpr {
        let mut exprs = vec![];

        loop {
            let close = lexer.peek();
            match close.ty {
                Ty::CloseBrace => {}
                Ty::Eof => {
                    let error =
                        self.get_expr(*close, ExprValue::Invalid("unclosed block expression"));
                    exprs.push(error);
                    break;
                }
                _ => {
                    let expr = self.parse_expr(lexer, Precedence::Base);
                    exprs.push(expr);
                }
            }
        }

        BlockExpr { exprs }
    }

    pub fn parse_type(&mut self, s: MStr) -> UncheckedTy {
        match self.get_str(s) {
            "i64" => self.module.type_i64(),
            "f64" => self.module.type_f64(),
            "none" => self.module.type_unit(),
            "bool" => self.module.type_bool(),
            _ => todo!("parse_types()"),
        }
    }

    fn get_str(&mut self, s: MStr) -> &str {
        self.module.str_store.get_str(s)
    }

    fn str_store(&mut self) -> &mut StrStore {
        self.module.str_store
    }
}
