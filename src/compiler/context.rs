use std::path::PathBuf;

use crate::compiler::module::{AstModule, Module, ModuleValue};
use crate::compiler::str_store::StrStore;
use crate::compiler::ty_store::TyStore;

pub struct Context {
    str_store: StrStore,
    ty_store: TyStore,
    modules: Vec<ModuleValue>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            str_store: StrStore::new(),
            ty_store: TyStore {},
            modules: vec![],
        }
    }

    pub fn create_module(&mut self, path: PathBuf) -> Module {
        let idx = self.modules.len();
        let module_value = ModuleValue::new(path);
        self.modules.push(module_value);

        Module::from(idx)
    }

    pub fn get_ast_module<'s>(&'s mut self, module: Module) -> AstModule<'s> {
        let module_value = self
            .modules
            .get_mut(module.0 as usize)
            .expect("failed to find module");

        AstModule {
            str_store: &mut self.str_store,
            ast_store: &mut module_value.ast_store,
        }
    }
}
