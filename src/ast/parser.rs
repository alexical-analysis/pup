mod decl;
mod expr;

use std::collections::HashMap;

use bumpalo::Bump;

use crate::ast::ast::{BlockExpr, Decl, Expr};
use crate::compiler::context::Context;
use crate::compiler::str_store::MStr;
use crate::types::unchecked_ty::{UncheckedTy, UncheckedTyValue};

use super::ast::{DeclValue, ExprValue};
use super::lexer::Lexer;

pub struct Module<'ctx> {
    // needed for string interring
    ctx: &'ctx mut Context,

    decls: Vec<DeclValue>,
    exprs: Vec<ExprValue>,

    // manage unchecked types
    type_arena: Bump,
    type_map: HashMap<&'static UncheckedTyValue, UncheckedTy>,
    types: Vec<&'static UncheckedTyValue>,

    // the acctual ast produced from parsing
    ast: Vec<Decl>,
}

impl<'ctx> Module<'ctx> {
    pub fn new(ctx: &'ctx mut Context) -> Self {
        Self {
            ctx,
            decls: vec![],
            exprs: vec![],
            type_arena: Bump::new(),
            type_map: HashMap::new(),
            types: vec![],
            ast: vec![],
        }
    }

    pub fn create_parser<'m>(&'m mut self) -> Parser<'m, 'ctx> {
        Parser { module: self }
    }

    fn get_decl(&mut self, decl: DeclValue) -> Decl {
        let idx = self.decls.len();
        self.decls.push(decl);

        Decl::from(idx)
    }

    fn get_expr(&mut self, expr: ExprValue) -> Expr {
        let idx = self.exprs.len();
        self.exprs.push(expr);

        Expr::from(idx)
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

    pub fn type_i32(&mut self) -> UncheckedTy {
        self.get_ty(&UncheckedTyValue::I32)
    }

    pub fn type_i64(&mut self) -> UncheckedTy {
        self.get_ty(&UncheckedTyValue::I64)
    }

    pub fn type_f32(&mut self) -> UncheckedTy {
        self.get_ty(&UncheckedTyValue::F32)
    }

    pub fn type_f64(&mut self) -> UncheckedTy {
        self.get_ty(&UncheckedTyValue::F64)
    }

    pub fn type_unit(&mut self) -> UncheckedTy {
        self.get_ty(&UncheckedTyValue::Unit)
    }
}

pub struct Parser<'m, 'ctx> {
    module: &'m mut Module<'ctx>,
}

impl<'m, 'ctx> Parser<'m, 'ctx> {
    pub fn get_decl(&mut self, decl: DeclValue) -> Decl {
        self.module.get_decl(decl)
    }

    pub fn get_expr(&mut self, expr: ExprValue) -> Expr {
        self.module.get_expr(expr)
    }

    pub fn parse(&mut self, source: &str) {
        let lexer = Lexer::new(self.module.ctx, source);
    }

    fn parse_body(&mut self) -> BlockExpr {
        let expr = self
            .module
            .get_expr(ExprValue::Invalid("TODO: parse_body not yet implemeted"));
        BlockExpr { exprs: vec![expr] }
    }

    pub fn parse_type(&mut self, s: MStr) -> UncheckedTy {
        match self.module.ctx.get_str(s) {
            "i32" => self.module.type_i32(),
            "i64" => self.module.type_i64(),
            "f32" => self.module.type_f32(),
            "f64" => self.module.type_f64(),
            _ => todo!("parse_types()"),
        }
    }

    fn ctx_mut(&mut self) -> &mut Context {
        &mut self.module.ctx
    }
}
