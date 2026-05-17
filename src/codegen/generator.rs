use crate::compiler::module::GenModule;

pub struct Generator<'m, 'ctx> {
    module: &'m mut GenModule<'ctx>,
}

impl<'m, 'ctx> Generator<'m, 'ctx> {
    pub fn new(module: &'m mut GenModule<'ctx>) -> Self {
        Self { module }
    }

    pub fn codegen(&self) {
        todo!("Generator::codegen()")
    }
}
