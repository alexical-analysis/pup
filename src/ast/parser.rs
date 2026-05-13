mod decl;
mod infix_expr;
mod prefix_expr;

use std::collections::HashMap;
use std::rc::Rc;

use bumpalo::Bump;

use crate::ast::ast::{BlockExpr, Decl, Expr};
use crate::ast::lexer::{Pos, Token, Ty};
use crate::ast::parser::decl::parse_decl;
use crate::ast::parser::infix_expr::{InfixExprParselet, Precedence, expr_infix_parselets};
use crate::ast::parser::prefix_expr::parse_prefix_expr;
use crate::compiler::context::Context;
use crate::compiler::str_store::MStr;
use crate::types::unchecked_ty::{UncheckedTy, UncheckedTyValue};

use super::ast::{DeclValue, ExprValue};
use super::lexer::Lexer;

pub struct AstModule<'ctx> {
    // needed for string interring
    ctx: &'ctx mut Context,

    decls: Vec<DeclValue>,
    decl_start: HashMap<Decl, Pos>,

    exprs: Vec<ExprValue>,
    expr_start: HashMap<Expr, Pos>,

    // manage unchecked types
    type_arena: Bump,
    type_map: HashMap<&'static UncheckedTyValue, UncheckedTy>,
    types: Vec<&'static UncheckedTyValue>,

    // the acctual ast produced from parsing
    ast: Vec<Decl>,
}

impl<'ctx> AstModule<'ctx> {
    pub fn new(ctx: &'ctx mut Context) -> Self {
        Self {
            ctx,
            decls: vec![],
            decl_start: HashMap::new(),
            exprs: vec![],
            expr_start: HashMap::new(),
            type_arena: Bump::new(),
            type_map: HashMap::new(),
            types: vec![],
            ast: vec![],
        }
    }

    pub fn create_parser<'m>(&'m mut self) -> Parser<'m, 'ctx> {
        Parser {
            module: self,
            infix_parselets: expr_infix_parselets(),
        }
    }

    fn get_decl(&mut self, start: Token, decl: DeclValue) -> Decl {
        let idx = self.decls.len();
        self.decls.push(decl);

        let decl = Decl::from(idx);
        self.decl_start.insert(decl, start.pos);

        decl
    }

    fn get_expr(&mut self, start: Token, expr: ExprValue) -> Expr {
        let idx = self.exprs.len();
        self.exprs.push(expr);

        let expr = Expr::from(idx);
        self.expr_start.insert(expr, start.pos);

        expr
    }

    fn get_ty(&mut self, ty: &UncheckedTyValue) -> UncheckedTy {
        if let Some(&id) = self.type_map.get(ty) {
            return id;
        }

        // SAFETY: Bump allocations live in heap-allocated chunks that never
        // move. We never expose these references beyond the lifetime of `self`.
        let interned: &'static UncheckedTyValue =
            unsafe { std::mem::transmute(self.type_arena.alloc(ty)) };
        let idx = self.types.len();
        let ty = UncheckedTy::from(idx);

        self.types.push(interned);
        self.type_map.insert(interned, ty);

        ty
    }

    pub fn type_i64(&mut self) -> UncheckedTy {
        self.get_ty(&UncheckedTyValue::I64)
    }

    pub fn type_f64(&mut self) -> UncheckedTy {
        self.get_ty(&UncheckedTyValue::F64)
    }

    pub fn type_unit(&mut self) -> UncheckedTy {
        self.get_ty(&UncheckedTyValue::Unit)
    }

    pub fn type_bool(&mut self) -> UncheckedTy {
        self.get_ty(&UncheckedTyValue::Bool)
    }
}

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
        let mut lexer = Lexer::new(self.module.ctx, source);

        let mut decls = vec![];
        loop {
            let token = lexer.peek().clone();
            if token.ty == Ty::Eof {
                break;
            }

            let decl = parse_decl(self, &mut lexer, token);
            decls.push(decl);

            // expect a semicolon after each declaration
            let end = lexer.next(self.ctx_mut());
            if end.ty != Ty::Semicolon {
                lexer.recover_until_decl(self.ctx_mut());
                let decl = self.get_decl(
                    end,
                    DeclValue::Invalid("missing semicolon/new line after declaration"),
                );
                decls.push(decl);
            }
        }
    }

    fn parse_expr(&mut self, lexer: &mut Lexer, min_precedence: Precedence) -> Expr {
        let token = lexer.next(self.ctx_mut());

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

            let token = lexer.next(self.ctx_mut());
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
        match self.module.ctx.get_str(s) {
            "i64" => self.module.type_i64(),
            "f64" => self.module.type_f64(),
            "none" => self.module.type_unit(),
            "bool" => self.module.type_bool(),
            _ => todo!("parse_types()"),
        }
    }

    fn ctx_mut(&mut self) -> &mut Context {
        &mut self.module.ctx
    }
}
