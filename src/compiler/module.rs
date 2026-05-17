use std::path::PathBuf;

use crate::ast::ast::{self};
use crate::ast::lexer::{Pos, Token};
use crate::compiler::ast_store::AstStore;
use crate::compiler::context::Context;
use crate::compiler::gen_store::GenStore;
use crate::compiler::hir_store::HirStore;
use crate::compiler::mir_store::MirStore;
use crate::compiler::str_store::{MStr, StrStore};
use crate::hir::hir::{self};
use crate::index_vec::Indexer;
use crate::types::unchecked_ty::{UncheckedTy, UncheckedTyValue};

#[derive(Debug, Clone)]
pub struct ImportList {
    path: Vec<Module>,
}

impl ImportList {
    fn new(root: Module) -> Self {
        Self { path: vec![root] }
    }

    fn push(&mut self, module: Module) {
        self.path.push(module)
    }

    fn pop(&mut self) {
        self.path.pop();
    }

    fn is_cycle(&self) -> bool {
        let last = match self.path.last() {
            Some(last) => last,
            None => return false,
        };

        for module in &self.path[..self.path.len() - 1] {
            if *module == *last {
                return true;
            }
        }

        return false;
    }
}

pub struct ModuleDag {
    root: Module,
}

impl ModuleDag {
    pub fn new(ctx: &mut Context, modules: &[Module]) -> Self {
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
            Some(root) => Self { root },
            None => panic!("failed to find root module"),
        }
    }

    // returns any ImportLists that are cyclic
    pub fn validate(&self, ctx: &mut Context) -> Vec<ImportList> {
        let mut import_list = ImportList {
            path: vec![self.root],
        };

        self.find_import_cycles(ctx, &mut import_list)
    }

    fn find_import_cycles(
        &self,
        ctx: &mut Context,
        import_path: &mut ImportList,
    ) -> Vec<ImportList> {
        // check if we're in an import cycle, if we are we can stop recursing and return this as a
        // cyclical path in the module dag
        if import_path.is_cycle() {
            return vec![import_path.clone()];
        }

        let last = *import_path.path.last().expect("import path is empty");

        let mut cycles = vec![];
        let deps = ctx.get_module_deps(last).to_vec();
        for dep in deps {
            import_path.push(dep);
            let mut new_cycles = self.find_import_cycles(ctx, import_path);
            cycles.append(&mut new_cycles);
            import_path.pop();
        }

        cycles
    }

    pub fn iter(&self, ctx: &mut Context) -> ModuleDagIter {
        let mut stack = vec![self.root];
        self.build_stack(ctx, &mut stack);

        ModuleDagIter { stack }
    }

    fn build_stack(&self, ctx: &mut Context, stack: &mut Vec<Module>) {
        let last = match stack.last() {
            Some(last) => last,
            None => return,
        };

        let deps = ctx.get_module_deps(*last).to_vec();
        for dep in deps {
            if stack.contains(&dep) {
                continue;
            }

            stack.push(dep);
            self.build_stack(ctx, stack);
        }
    }
}

pub struct ModuleDagIter {
    stack: Vec<Module>,
}

impl<'a> Iterator for ModuleDagIter {
    type Item = Module;

    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop()
    }
}

pub struct ModuleValue {
    pub name: Option<MStr>,
    pub import_path: MStr,
    pub deps: Vec<Module>,
    pub object_path: Option<PathBuf>,
    pub ast_store: AstStore,
    pub hir_store: HirStore,
    pub mir_store: MirStore,
    pub gen_store: GenStore,
}

impl ModuleValue {
    pub fn new(import_path: MStr) -> Self {
        Self {
            name: None,
            import_path,
            deps: vec![],
            object_path: None,
            ast_store: AstStore::new(),
            hir_store: HirStore::new(),
            mir_store: MirStore::new(),
            gen_store: GenStore::new(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Module(u32);

impl Indexer for Module {
    fn index(&self) -> usize {
        self.0 as usize
    }
}

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

    pub fn get_decl_value(&self, decl: ast::Decl) -> &ast::DeclValue {
        self.ast_store
            .decls
            .get(decl)
            .expect("failed to find decl value")
    }

    pub fn update_decl_value(&mut self, decl: ast::Decl, value: ast::DeclValue) {
        let decl_value = self
            .ast_store
            .decls
            .get_mut(decl)
            .expect("failed to find decl value");
        *decl_value = value
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

    pub fn get_name(&self) -> MStr {
        let mod_decl = match self.ast_store.ast.first() {
            Some(decl) => decl,
            None => todo!("missing module name"),
        };

        let decl_value = self
            .ast_store
            .decls
            .get(*mod_decl)
            .expect("failed to find decl");

        match decl_value {
            ast::DeclValue::Mod(decl) => decl.name,
            _ => todo!("first statment must be a module decl name"),
        }
    }

    pub fn get_use_decl(&self) -> Option<&ast::UseDecl> {
        // use decl can only appear as the second decl in a file
        let use_decl = match self.ast_store.ast.get(1) {
            Some(decl) => {
                let decl_value = self.get_decl_value(*decl);
                match decl_value {
                    ast::DeclValue::Use(use_decl) => use_decl,
                    _ => return None,
                }
            }
            None => return None,
        };

        Some(use_decl)
    }
}

pub struct HirModule<'s> {
    pub str_store: &'s mut StrStore,
    pub ast_store: &'s AstStore,
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
    pub hir_store: &'s HirStore,
    pub mir_store: &'s mut MirStore,
}

pub struct GenModule<'s> {
    pub str_store: &'s mut StrStore,
    pub hir_store: &'s HirStore,
}

impl<'s> GenModule<'s> {}
