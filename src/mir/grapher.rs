mod expr;
mod func;

use crate::compiler::context::Context;
use crate::compiler::hir_store::HirStore;
use crate::compiler::mir_store::MirStore;
use crate::compiler::module::{Module, ModuleValue};
use crate::compiler::ty_store::CheckedTyStore;
use crate::hir::hir::{DeclValue, FuncDecl};
use crate::mir::grapher::func::graph_func;
use crate::mir::mir::Func;

pub struct Grapher<'ctx> {
    module: Module,
    module_value: &'ctx ModuleValue,
    hir_store: &'ctx mut HirStore,
    mir_store: &'ctx mut MirStore,
    checked_ty_store: &'ctx mut CheckedTyStore,
}

impl<'ctx> Grapher<'ctx> {
    pub fn new(ctx: &'ctx mut Context, module: Module) -> Self {
        Self {
            module,
            module_value: ctx.module_store.get(module).expect("failed to find module"),
            hir_store: &mut ctx.hir_store,
            mir_store: &mut ctx.mir_store,
            checked_ty_store: &mut ctx.checked_ty_store,
        }
    }

    pub fn graph(&mut self) {
        for &decl in &self.module_value.hir {
            let decl_value = self.hir_store.get_decl_value(decl).clone();

            match decl_value {
                DeclValue::Invalid(v) => {
                    todo!(
                        "If one of the modules top level decls are invalid, the 'init' function will panic"
                    )
                }
                DeclValue::Ty(_) => { /* types do not appear in the CFG*/ }
                DeclValue::Func(v) => graph_func(self, v),
            }
        }
    }

    pub fn create_func(&mut self, decl: FuncDecl) -> Func {
        self.mir_store.create_func(decl)
    }
}
