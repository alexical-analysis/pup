use crate::compiler::context::Context;
use crate::compiler::module::{Module, ModuleValue};

pub struct Grapher<'ctx> {
    module: Module,
    module_value: &'ctx ModuleValue,
}

impl<'ctx> Grapher<'ctx> {
    pub fn new(ctx: &'ctx mut Context, module: Module) -> Self {
        Self {
            module,
            module_value: ctx.module_store.get(module).expect("failed to find module"),
        }
    }

    pub fn graph(&mut self) {
        todo!("Grapher::graph()")
    }
}
