use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::compiler::ast_store::AstStore;
use crate::compiler::builder::Builder;
use crate::compiler::module::{AstModule, GenModule, HirModule, MirModule, Module, ModuleValue};
use crate::compiler::str_store::{MStr, StrStore};
use crate::compiler::ty_store::TyStore;
use crate::index_vec::IndexVec;

pub struct Context {
    str_store: StrStore,
    ty_store: TyStore,
    modules: IndexVec<Module, ModuleValue>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            str_store: StrStore::new(),
            ty_store: TyStore::new(),
            modules: IndexVec::new(),
        }
    }

    pub fn create_builder<'ctx>(&'ctx mut self) -> Builder<'ctx> {
        Builder::new(self)
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

    pub fn set_module_name(&mut self, module: Module, name: MStr) {
        let module = self.modules.get_mut(module).expect("failed to find module");
        if module.name.is_some() {
            panic!("module already has a name")
        }

        module.name = Some(name)
    }

    pub fn set_module_deps(&mut self, module: Module, import_paths: &[MStr]) {
        let mut deps = HashMap::new();
        for import_path in import_paths {
            let dep = self
                .find_module(*import_path)
                .expect("failed to find module import");
            deps.insert(*import_path, dep);
        }

        let module = self.modules.get_mut(module).expect("failed to find module");
        if !module.deps.is_empty() {
            panic!("module deps already defined")
        }

        module.deps = deps
    }

    pub fn find_module(&mut self, import_path: MStr) -> Option<Module> {
        for (idx, module) in self.modules.iter().enumerate() {
            if module.import_path == import_path {
                let module = Module::from(idx);
                return Some(module);
            }
        }
        return None;
    }

    pub fn get_module_deps(&self, module: Module) -> HashMap<MStr, Module> {
        self.modules
            .get(module)
            .expect("failed to get module")
            .deps
            .clone()
    }

    pub fn get_import_path(&self, module: Module) -> Option<MStr> {
        match self.modules.get(module) {
            Some(module) => Some(module.import_path),
            None => None,
        }
    }

    pub fn get_ast_module<'s>(&'s mut self, module: Module) -> AstModule<'s> {
        let module_value = self.modules.get_mut(module).expect("failed to find module");

        AstModule {
            str_store: &mut self.str_store,
            ast_store: &mut module_value.ast_store,
        }
    }

    pub fn get_hir_module<'s>(&'s mut self, module: Module) -> HirModule<'s> {
        let module_value = self.modules.get_mut(module).expect("failed to find module");

        HirModule {
            module: module,
            deps: &module_value.deps,
            ast_store: &module_value.ast_store,
            str_store: &mut self.str_store,
            hir_store: &mut module_value.hir_store,
            ty_store: &mut self.ty_store,
        }
    }

    pub fn get_mir_module<'s>(&'s mut self, module: Module) -> MirModule<'s> {
        let module_value = self.modules.get_mut(module).expect("failed to find module");

        MirModule {
            str_store: &mut self.str_store,
            hir_store: &module_value.hir_store,
            mir_store: &mut module_value.mir_store,
        }
    }

    pub fn get_gen_module<'s>(&'s mut self, module: Module) -> GenModule<'s> {
        let module_vlue = self.modules.get_mut(module).expect("failed to find module");

        GenModule {
            str_store: &mut self.str_store,
            hir_store: &module_vlue.hir_store,
        }
    }

    pub fn get_object_files(&self) -> Vec<&Path> {
        let mut paths = vec![];

        for module in self.modules.iter() {
            match &module.object_path {
                Some(path) => paths.push(path.as_path()),
                None => panic!("missing object path for module"),
            }
        }

        paths
    }
}
