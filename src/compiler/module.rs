use std::path::PathBuf;

use crate::ast::ast::{Decl, DeclValue, Expr, ExprValue};
use crate::ast::lexer::Token;
use crate::compiler::ast_store::AstStore;
use crate::compiler::gen_store::GenStore;
use crate::compiler::hir_store::HirStore;
use crate::compiler::mir_store::MirStore;
use crate::compiler::str_store::StrStore;
use crate::types::unchecked_ty::{UncheckedTy, UncheckedTyValue};

pub struct ModuleValue {
    pub path: PathBuf,
    pub ast_store: AstStore,
    pub hir_store: HirStore,
    pub mir_store: MirStore,
    pub gen_store: GenStore,
}

impl ModuleValue {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            ast_store: AstStore::new(),
            hir_store: HirStore::new(),
            mir_store: MirStore::new(),
            gen_store: todo!(),
        }
    }
}

pub struct Module(pub u32);

impl From<usize> for Module {
    fn from(value: usize) -> Self {
        Module(value as u32)
    }
}

pub struct AstModule<'s> {
    pub str_store: &'s mut StrStore,
    pub ast_store: &'s mut AstStore,
}

impl<'s> AstModule<'s> {
    fn get_decl(&mut self, token: Token, decl_value: DeclValue) -> Decl {
        self.ast_store.get_decl(token, decl_value)
    }

    fn get_expr(&mut self, token: Token, expr_value: ExprValue) -> Expr {
        self.ast_store.get_expr(token, expr_value)
    }

    pub fn type_i64(&mut self) -> UncheckedTy {
        self.ast_store.get_ty(&UncheckedTyValue::I64)
    }

    pub fn type_f64(&mut self) -> UncheckedTy {
        self.ast_store.get_ty(&UncheckedTyValue::F64)
    }

    pub fn type_unit(&mut self) -> UncheckedTy {
        self.ast_store.get_ty(&UncheckedTyValue::Unit)
    }

    pub fn type_bool(&mut self) -> UncheckedTy {
        self.ast_store.get_ty(&UncheckedTyValue::Bool)
    }
}

pub struct HirModule<'s> {
    pub str_store: &'s mut StrStore,
    pub ast_store: &'s mut AstStore,
    pub hir_store: &'s mut HirStore,
}

pub struct MirModule<'s> {
    pub str_store: &'s mut StrStore,
    pub hir_store: &'s mut HirStore,
    pub mir_store: &'s mut MirStore,
}
