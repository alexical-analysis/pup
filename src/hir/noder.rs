mod decl;
mod expr;
mod types;

use crate::ast::ast;
use crate::ast::lexer::Pos;
use crate::compiler::module::{HirModule, Module};
use crate::compiler::str_store::MStr;
use crate::hir::hir;
use crate::types::checked_ty::{CheckedTy, CheckedTyValue};

use decl::parse_decl;

pub struct Noder<'m, 'ctx> {
    module: &'m mut HirModule<'ctx>,
}

impl<'m, 'ctx> Noder<'m, 'ctx> {
    pub fn new(module: &'m mut HirModule<'ctx>) -> Self {
        Self { module }
    }

    pub fn get_decl_start(&self, decl: ast::Decl) -> Pos {
        self.module.ast_store.get_decl_start(decl)
    }

    pub fn get_decl(&mut self, start: Pos, decl: hir::DeclValue) -> hir::Decl {
        self.module.get_decl(start, decl)
    }

    pub fn get_expr_start(&self, expr: ast::Expr) -> Pos {
        self.module.ast_store.get_expr_start(expr)
    }

    pub fn get_expr(&mut self, start: Pos, expr: hir::ExprValue) -> hir::Expr {
        self.module.get_expr(start, expr)
    }

    pub fn get_ty(&mut self, ty: CheckedTyValue) -> CheckedTy {
        self.module.ty_store.get_ty(ty)
    }

    pub fn find_module(&mut self, alias: MStr) -> Module {
        *self
            .module
            .deps
            .get(&alias)
            .expect("failed to find dependent module")
    }

    pub fn node(&mut self) {
        let mut decls = vec![];

        for decl in &self.module.ast_store.ast {
            let decl = match parse_decl(self, *decl) {
                Some(decl) => decl,
                None => continue,
            };

            decls.push(decl);
        }
    }
}
