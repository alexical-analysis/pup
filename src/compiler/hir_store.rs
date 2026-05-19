use std::collections::HashMap;

use crate::ast::lexer::Pos;
use crate::hir::hir::{Decl, DeclValue, Expr, ExprValue};
use crate::index_vec::IndexVec;
use crate::types::checked_ty::CheckedTy;

pub struct HirStore {
    decls: IndexVec<Decl, DeclValue>,
    decl_start: HashMap<Decl, Pos>,

    exprs: IndexVec<Expr, ExprValue>,
    expr_start: HashMap<Expr, Pos>,
    ty_map: HashMap<Expr, CheckedTy>,

    hir: Vec<Decl>,
}

impl HirStore {
    pub fn new() -> Self {
        Self {
            decls: IndexVec::new(),
            decl_start: HashMap::new(),
            exprs: IndexVec::new(),
            expr_start: HashMap::new(),
            ty_map: HashMap::new(),
            hir: vec![],
        }
    }

    pub fn get_decl(&mut self, start: Pos, decl: DeclValue) -> Decl {
        let idx = self.decls.len();
        self.decls.push(decl);

        let decl = Decl::from(idx);
        self.decl_start.insert(decl, start);

        decl
    }

    pub fn get_expr(&mut self, start: Pos, expr: ExprValue) -> Expr {
        let idx = self.exprs.len();
        self.exprs.push(expr);

        let expr = Expr::from(idx);
        self.expr_start.insert(expr, start);

        expr
    }
}
