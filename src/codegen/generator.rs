use crate::compiler::{
    context::Context,
    module::{Module, ModuleValue},
};

pub struct Generator<'ctx> {
    module: Module,
    module_value: &'ctx ModuleValue,
}

impl<'ctx> Generator<'ctx> {
    pub fn new(ctx: &'ctx mut Context, module: Module) -> Self {
        Self {
            module,
            module_value: ctx
                .module_store
                .get(module)
                .expect("failed to get module value"),
        }
    }

    pub fn codegen(&mut self) {
        todo!("Generator::codegen()")
    }
}
