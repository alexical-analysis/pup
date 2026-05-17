use crate::compiler::module::MirModule;

pub struct Grapher<'m, 'ctx> {
    module: &'m mut MirModule<'ctx>,
}

impl<'m, 'ctx> Grapher<'m, 'ctx> {
    pub fn new(module: &'m mut MirModule<'ctx>) -> Self {
        Self { module }
    }

    pub fn graph(&self) {
        todo!("Grapher::graph()")
    }
}
