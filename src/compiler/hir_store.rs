use std::collections::HashMap;

use crate::ast::lexer::Pos;
use crate::hir::hir::{Decl, DeclValue, Expr, ExprValue};

pub struct HirStore {
    decls: Vec<DeclValue>,
    decl_start: HashMap<Decl, Pos>,

    exprs: Vec<ExprValue>,
    expr_start: HashMap<Expr, Pos>,

    // the acctual ast produced from parsing
    hir: Vec<Decl>,
}

impl HirStore {
    pub fn new() -> Self {
        Self {
            decls: vec![],
            decl_start: HashMap::new(),
            exprs: vec![],
            expr_start: HashMap::new(),
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
