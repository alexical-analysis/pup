use crate::ast::ast;
use crate::ast::lexer::Pos;
use crate::hir::hir::{self, TypeDecl};
use crate::hir::noder::Noder;
use crate::hir::noder::checker::check_single_type;
use crate::hir::noder::expr::node_expr;

fn node_invalid_decl(noder: &mut Noder, start: Pos, msg: &'static str) -> hir::Decl {
    noder.get_decl(start, hir::DeclValue::Invalid(msg))
}

fn node_type_decl(noder: &mut Noder, start: Pos, decl: &ast::TyDecl) -> hir::Decl {
    let ty = check_single_type(noder, decl.ty);

    noder.get_decl(
        start,
        hir::DeclValue::Type(TypeDecl {
            name: decl.name,
            ty,
        }),
    )
}

fn node_fn_decl(noder: &mut Noder, start: Pos, decl: &ast::FuncDecl) -> hir::Decl {
    let mut exprs = vec![];
    for &e in &decl.body.exprs {
        let expr = node_expr(noder, e);
        exprs.push(expr);
    }

    let ty = check_single_type(noder, decl.ty);

    noder.get_decl(
        start,
        hir::DeclValue::Func(hir::FuncDecl {
            name: decl.name,
            params: decl.params.clone(),
            body: hir::BlockExpr { exprs },
            ty,
        }),
    )
}

pub fn parse_decl(noder: &mut Noder, decl: ast::Decl) -> Option<hir::Decl> {
    let start = noder.get_decl_start(decl);
    let decl_value = noder.module.ast_store.get_decl_value(decl);

    match decl_value {
        ast::DeclValue::Invalid(v) => Some(node_invalid_decl(noder, start, v)),
        ast::DeclValue::Mod(_) => None,
        ast::DeclValue::Use(_) => None,
        ast::DeclValue::Ty(v) => Some(node_type_decl(noder, start, v)),
        ast::DeclValue::Func(v) => Some(node_fn_decl(noder, start, v)),
    }
}
