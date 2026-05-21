use crate::ast::ast;
use crate::ast::lexer::Pos;
use crate::hir::hir;
use crate::hir::noder::Noder;
use crate::hir::noder::checker::check_single_type;
use crate::hir::noder::expr::node_expr;
use crate::hir::noder::sym_table::SymTable;

fn node_invalid_decl(noder: &mut Noder, start: Pos, msg: &'static str) -> hir::Decl {
    noder.get_decl(start, hir::DeclValue::Invalid(msg))
}

fn node_type_decl(noder: &mut Noder, start: Pos, decl: ast::TyDecl) -> hir::Decl {
    let ty = check_single_type(noder, decl.ty);

    noder.get_decl(
        start,
        hir::DeclValue::Ty(hir::TyDecl {
            name: decl.name,
            ty,
        }),
    )
}

fn node_fn_decl(
    noder: &mut Noder,
    sym_table: &mut SymTable,
    start: Pos,
    decl: ast::FuncDecl,
) -> hir::Decl {
    let mut exprs = vec![];
    for &e in &decl.body.exprs {
        let expr = node_expr(noder, sym_table, e);
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

pub fn node_decl(
    noder: &mut Noder,
    sym_table: &mut SymTable,
    decl: ast::Decl,
) -> Option<hir::Decl> {
    let start = noder.get_decl_start(decl);
    let decl_value = noder.get_ast_decl_value(decl).clone();

    let hir_decl = match decl_value {
        ast::DeclValue::Invalid(v) => node_invalid_decl(noder, start, v),
        ast::DeclValue::Mod(_) => return None,
        ast::DeclValue::Use(_) => return None,
        ast::DeclValue::Ty(v) => node_type_decl(noder, start, v),
        ast::DeclValue::Func(v) => node_fn_decl(noder, sym_table, start, v),
    };

    sym_table.map_decl(decl, hir_decl);

    Some(hir_decl)
}
