use crate::ast::ast;
use crate::hir::hir;
use crate::hir::noder::Noder;

pub fn parse_decl(noder: &mut Noder, decl: ast::Decl) -> hir::Decl {
    let decl_value = noder.get_ast_decl_value(decl);
    todo!()
}
