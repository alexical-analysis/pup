use std::collections::HashMap;

use bumpalo::Bump;

use crate::ast::ast::{Decl, DeclValue, Expr, ExprValue};
use crate::ast::lexer::{Pos, Token};
use crate::index_vec::IndexVec;
use crate::types::unchecked_ty::{UncheckedTy, UncheckedTyValue};

pub struct AstStore {
    pub decls: IndexVec<Decl, DeclValue>,
    decl_start: HashMap<Decl, Pos>,

    exprs: IndexVec<Expr, ExprValue>,
    expr_start: HashMap<Expr, Pos>,

    // manage unchecked types
    type_arena: Bump,
    type_map: HashMap<&'static UncheckedTyValue, UncheckedTy>,
    types: IndexVec<UncheckedTy, &'static UncheckedTyValue>,

    // the acctual ast produced from parsing
    pub ast: Vec<Decl>,
}

impl AstStore {
    pub fn new() -> Self {
        Self {
            decls: IndexVec::new(),
            decl_start: HashMap::new(),
            exprs: IndexVec::new(),
            expr_start: HashMap::new(),
            type_arena: Bump::new(),
            type_map: HashMap::new(),
            types: IndexVec::new(),
            ast: vec![],
        }
    }

    pub fn get_decl(&mut self, start: Token, decl: DeclValue) -> Decl {
        let idx = self.decls.len();
        self.decls.push(decl);

        let decl = Decl::from(idx);
        self.decl_start.insert(decl, start.pos);

        decl
    }

    pub fn get_expr(&mut self, start: Token, expr: ExprValue) -> Expr {
        let idx = self.exprs.len();
        self.exprs.push(expr);

        let expr = Expr::from(idx);
        self.expr_start.insert(expr, start.pos);

        expr
    }

    pub fn get_ty(&mut self, ty: &UncheckedTyValue) -> UncheckedTy {
        if let Some(&id) = self.type_map.get(ty) {
            return id;
        }

        // SAFETY: Bump allocations live in heap-allocated chunks that never
        // move. We never expose these references beyond the lifetime of `self`.
        let interned: &'static UncheckedTyValue =
            unsafe { std::mem::transmute(self.type_arena.alloc(ty)) };
        let idx = self.types.len();
        let ty = UncheckedTy::from(idx);

        self.types.push(interned);
        self.type_map.insert(interned, ty);

        ty
    }
}
