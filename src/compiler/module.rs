use std::collections::HashSet;
use std::path::PathBuf;

use crate::ast::ast;
use crate::ast::lexer::{Pos, Token};
use crate::compiler::ast_store::AstStore;
use crate::compiler::context::Context;
use crate::compiler::gen_store::GenStore;
use crate::compiler::hir_store::HirStore;
use crate::compiler::mir_store::MirStore;
use crate::compiler::str_store::{MStr, StrStore};
use crate::hir::hir;
use crate::types::unchecked_ty::{UncheckedTy, UncheckedTyValue};

pub struct ModuleDag<'ctx> {
    root: Module,
    modules: &'ctx [Module],
}

impl<'ctx> ModuleDag<'ctx> {
    fn new(ctx: &'ctx mut Context, modules: &'ctx [Module]) -> Self {
        let mut root = None;
        // in pup the root module is always in the main.pup file
        let root_import_path = ctx.get_mstr("main");

        for module in modules {
            let import_path = ctx
                .get_import_path(*module)
                .expect("failed to get module import path");
            if import_path == root_import_path {
                root = Some(*module);
                break;
            }
        }

        match root {
            Some(root) => Self { root, modules },
            None => panic!("failed to find root module"),
        }
    }
}

struct ModuleDagIter {
    stack: Vec<Module>,
}

impl<'a> Iterator for ModuleDagIter {
    type Item = Module;

    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop()
    }
}

pub struct ModuleValue {
    pub name: MStr,
    pub import_path: MStr,
    pub ast_store: AstStore,
    pub hir_store: HirStore,
    pub mir_store: MirStore,
    pub gen_store: GenStore,
}

impl ModuleValue {
    pub fn new(import_path: MStr, name: MStr) -> Self {
        Self {
            name,
            import_path,
            ast_store: AstStore::new(),
            hir_store: HirStore::new(),
            mir_store: MirStore::new(),
            gen_store: GenStore::new(),
        }
    }
}

#[derive(Clone, Copy)]
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
    pub fn get_decl(&mut self, token: Token, decl_value: ast::DeclValue) -> ast::Decl {
        self.ast_store.get_decl(token, decl_value)
    }

    pub fn get_expr(&mut self, token: Token, expr_value: ast::ExprValue) -> ast::Expr {
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

impl<'s> HirModule<'s> {
    pub fn get_decl(&mut self, start: Pos, decl: hir::DeclValue) -> hir::Decl {
        self.hir_store.get_decl(start, decl)
    }

    pub fn get_expr(&mut self, start: Pos, expr: hir::ExprValue) -> hir::Expr {
        self.hir_store.get_expr(start, expr)
    }
}

pub struct MirModule<'s> {
    pub str_store: &'s mut StrStore,
    pub hir_store: &'s mut HirStore,
    pub mir_store: &'s mut MirStore,
}
