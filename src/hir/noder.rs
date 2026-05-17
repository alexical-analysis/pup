use crate::ast::lexer::Pos;
use crate::compiler::module::HirModule;
use crate::hir::hir::{Decl, DeclValue, Expr, ExprValue};

pub struct Noder<'m, 'ctx> {
    module: &'m mut HirModule<'ctx>,
}

impl<'m, 'ctx> Noder<'m, 'ctx> {
    pub fn new(module: &'m mut HirModule<'ctx>) -> Self {
        Self { module }
    }

    pub fn get_decl(&mut self, start: Pos, decl: DeclValue) -> Decl {
        self.module.get_decl(start, decl)
    }

    pub fn get_expr(&mut self, start: Pos, expr: ExprValue) -> Expr {
        self.module.get_expr(start, expr)
    }

    pub fn node(&self) {
        todo!("Noder::node()")
    }
}
