use crate::{hir::hir::FuncDecl, mir::grapher::Grapher};

pub fn graph_func(grapher: &mut Grapher, decl: FuncDecl) {
    let func = grapher.create_func(decl);
}
