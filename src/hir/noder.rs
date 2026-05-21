mod checker;
mod decl;
mod expr;
mod sym_table;

use crate::ast::ast;
use crate::ast::lexer::Pos;
use crate::compiler::ast_store::AstStore;
use crate::compiler::context::Context;
use crate::compiler::hir_store::HirStore;
use crate::compiler::module::{Module, ModuleValue};
use crate::compiler::str_store::MStr;
use crate::compiler::ty_store::{CheckedTyStore, UncheckedTyStore};
use crate::hir::hir;
use crate::hir::noder::checker::Checker;
use crate::hir::noder::sym_table::SymTable;
use crate::types::checked_ty::{CheckedTy, CheckedTyValue};
use crate::types::unchecked_ty::{UncheckedTy, UncheckedTyValue};

use decl::node_decl;

pub struct Noder<'ctx> {
    module: Module,
    module_value: &'ctx ModuleValue,
    ast_store: &'ctx mut AstStore,
    hir_store: &'ctx mut HirStore,
    unchecked_ty_store: &'ctx mut UncheckedTyStore,
    checked_ty_store: &'ctx mut CheckedTyStore,
}

impl<'ctx> Noder<'ctx> {
    pub fn new(ctx: &'ctx mut Context, module: Module) -> Self {
        Self {
            module,
            module_value: ctx.module_store.get(module).expect("failed to find module"),
            ast_store: &mut ctx.ast_store,
            hir_store: &mut ctx.hir_store,
            unchecked_ty_store: &mut ctx.unchecked_ty_store,
            checked_ty_store: &mut ctx.checked_ty_store,
        }
    }

    pub fn get_decl(&mut self, start: Pos, decl: hir::DeclValue) -> hir::Decl {
        self.hir_store.get_decl(start, decl)
    }

    pub fn get_hir_decl_value(&self, decl: hir::Decl) -> &hir::DeclValue {
        self.hir_store.get_decl_value(decl)
    }

    pub fn get_ast_decl_value(&self, decl: ast::Decl) -> &ast::DeclValue {
        self.ast_store.get_decl_value(decl)
    }

    pub fn get_decl_start(&self, decl: ast::Decl) -> Pos {
        self.ast_store.get_decl_start(decl)
    }

    pub fn get_expr(&mut self, start: Pos, expr: hir::ExprValue) -> hir::Expr {
        self.hir_store.get_expr(start, expr)
    }

    pub fn get_hir_expr_value(&self, expr: hir::Expr) -> &hir::ExprValue {
        self.hir_store.get_expr_value(expr)
    }

    pub fn get_ast_expr_value(&self, expr: ast::Expr) -> &ast::ExprValue {
        self.ast_store.get_expr_value(expr)
    }

    pub fn get_expr_start(&self, expr: ast::Expr) -> Pos {
        self.ast_store.get_expr_start(expr)
    }

    pub fn get_expr_ty(&self, expr: hir::Expr) -> CheckedTy {
        self.hir_store.get_expr_ty(expr)
    }

    pub fn get_checked_ty(&mut self, ty: CheckedTyValue) -> CheckedTy {
        self.checked_ty_store.get_ty(ty)
    }

    pub fn get_checked_ty_value(&mut self, ty: CheckedTy) -> &CheckedTyValue {
        self.checked_ty_store.get_ty_value(ty)
    }

    pub fn get_unchecked_ty_value(&self, ty: UncheckedTy) -> &UncheckedTyValue {
        self.unchecked_ty_store.get_ty_value(ty)
    }

    pub fn map_ty(&mut self, expr: hir::Expr, ty: CheckedTy) {
        self.hir_store.map_ty(expr, ty);
    }

    pub fn map_ty_value(&mut self, expr: hir::Expr, ty: CheckedTyValue) {
        let ty = self.get_checked_ty(ty);
        self.hir_store.map_ty(expr, ty);
    }

    pub fn find_dep(&mut self, alias: MStr) -> Module {
        self.module_value.get_dep(alias)
    }

    pub fn node(&mut self) {
        let mut decls = vec![];

        // build the symbol table from the ast for use when building the hir and type checking
        let mut sym_table = SymTable::new();
        for &decl in &self.module_value.ast {
            sym_table.add_decl(self.ast_store, decl);
        }

        // now construct the hir using the ast and the symbol table
        for decl in &self.module_value.ast {
            let decl = match node_decl(self, &mut sym_table, *decl) {
                Some(decl) => decl,
                None => continue,
            };

            decls.push(decl);
        }

        // once the hir is filled out we can type check the tree
        let mut checker = Checker::new(sym_table);
        for &decl in &self.module_value.hir {
            checker.check_decl(self, decl)
        }
    }
}
