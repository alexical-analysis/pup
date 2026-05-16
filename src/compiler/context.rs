use crate::compiler::module::{AstModule, Module, ModuleValue};
use crate::compiler::str_store::{MStr, StrStore};
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
            ty_store: TyStore::new(),
            modules: vec![],
        }
    }

    pub fn get_mstr(&mut self, s: &str) -> MStr {
        self.str_store.get_mstr(s)
    }

    pub fn create_module(&mut self, import_path: MStr) -> Module {
        let idx = self.modules.len();
        let module_value = ModuleValue::new(import_path);
        self.modules.push(module_value);

        Module::from(idx)
    }

    pub fn get_module_deps(&self, module: Module) -> &[Module] {
        &self
            .modules
            .get(module.0 as usize)
            .expect("failed to get module")
            .deps
    }

    pub fn get_import_path(&self, module: Module) -> Option<MStr> {
        match self.modules.get(module.0 as usize) {
            Some(module) => Some(module.import_path),
            None => None,
        }
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
