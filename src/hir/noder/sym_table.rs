use crate::ast::ast;
use crate::compiler::str_store::MStr;
use crate::hir::noder::Noder;
use crate::index_vec::{IndexVec, Indexer};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Scope(u32);

impl Indexer for Scope {
    fn index(&self) -> usize {
        self.0 as usize
    }
}

impl From<usize> for Scope {
    fn from(value: usize) -> Self {
        Self(value as u32)
    }
}

struct ScopeValue {
    parent: Option<Scope>,
    decls: HashMap<MStr, ast::Decl>,
}

impl ScopeValue {
    pub fn new_root() -> Self {
        Self {
            parent: None,
            decls: HashMap::new(),
        }
    }

    pub fn new_child(parent: Scope) -> Self {
        Self {
            parent: Some(parent),
            decls: HashMap::new(),
        }
    }
}

pub struct SymTable {
    current_scope: Scope,
    scopes: IndexVec<Scope, ScopeValue>,
    ident_map: HashMap<ast::Expr, ast::Decl>,
    uknown_idents: Vec<ast::Expr>,
}

impl SymTable {
    pub fn new() -> Self {
        let mut scopes = IndexVec::new();
        scopes.push(ScopeValue::new_root());

        Self {
            current_scope: Scope::from(0),
            scopes,
            ident_map: HashMap::new(),
            uknown_idents: vec![],
        }
    }

    pub fn add_decl(&mut self, noder: &mut Noder, decl: ast::Decl) {
        let decl_value = noder
            .module
            .ast_store
            .decls
            .get(decl)
            .expect("failed to get ast decl value");

        let current_scope = self.get_current_scope_vaue_mut();

        match decl_value {
            ast::DeclValue::Invalid(_) => {}
            ast::DeclValue::Mod(_) => {}
            ast::DeclValue::Use(_) => {}
            ast::DeclValue::Ty(v) => {
                current_scope.decls.insert(v.name, decl);
            }
            ast::DeclValue::Func(v) => {
                current_scope.decls.insert(v.name, decl);

                self.open_scope();

                for &expr in &v.body.exprs {
                    self.map_expr_bindings(noder, expr);
                }

                self.close_scope();
            }
        };
    }

    fn open_scope(&mut self) {
        let next_scope_value = ScopeValue::new_child(self.current_scope);
        let scope = Scope::from(self.scopes.len());
        self.scopes.push(next_scope_value);

        self.current_scope = scope
    }

    fn close_scope(&mut self) {
        let scope_value = self.get_current_scope_vaue();
        match scope_value.parent {
            Some(parent) => self.current_scope = parent,
            None => panic!("already at the root scope"),
        }
    }

    fn get_current_scope_vaue(&self) -> &ScopeValue {
        self.scopes
            .get(self.current_scope)
            .expect("failed to find scope")
    }

    fn get_current_scope_vaue_mut(&mut self) -> &mut ScopeValue {
        self.scopes
            .get_mut(self.current_scope)
            .expect("failed to find scope")
    }

    fn map_expr_bindings(&mut self, noder: &Noder, expr: ast::Expr) {
        let expr_value = noder
            .module
            .ast_store
            .exprs
            .get(expr)
            .expect("failed to get ast expr value");

        match expr_value {
            ast::ExprValue::Invalid(_) => {}
            ast::ExprValue::Identifier(v) => self.map_binding(expr, v),
            ast::ExprValue::Call(v) => {
                self.map_expr_bindings(noder, v.func);
                for &arg in &v.args {
                    self.map_expr_bindings(noder, arg)
                }
            }
            ast::ExprValue::Block(v) => {
                self.open_scope();

                for &expr in &v.exprs {
                    self.map_expr_bindings(noder, expr)
                }

                self.close_scope();
            }
            ast::ExprValue::Return(v) => {
                v.value.map(|v| self.map_expr_bindings(noder, v));
            }
            ast::ExprValue::If(v) => {
                self.map_expr_bindings(noder, v.check);
                for &expr in &v.success.exprs {
                    self.map_expr_bindings(noder, expr);
                }
            }
            ast::ExprValue::Loop(v) => {
                for &expr in &v.body.exprs {
                    self.map_expr_bindings(noder, expr);
                }
            }
            ast::ExprValue::Range(v) => {
                self.map_expr_bindings(noder, v.start);
                self.map_expr_bindings(noder, v.end);
            }
            ast::ExprValue::Break => {}
            ast::ExprValue::Binary(v) => {
                self.map_expr_bindings(noder, v.left);
                self.map_expr_bindings(noder, v.right);
            }
            ast::ExprValue::IntLiteral(_) => {}
            ast::ExprValue::FloatLiteral(_) => {}
            ast::ExprValue::BoolLiteral(_) => {}
        }
    }

    fn map_binding(&mut self, expr: ast::Expr, ident: &ast::IdentifierExpr) {
        if ident.module.is_some() {
            panic!("identifier is not referencing this module")
        }

        match self.search_scope(self.current_scope, expr, ident.name) {
            Some(decl) => {
                self.ident_map.insert(expr, decl);
            }
            None => self.uknown_idents.push(expr),
        };
    }

    fn search_scope(&self, scope: Scope, expr: ast::Expr, ident: MStr) -> Option<ast::Decl> {
        let scope_value = self.scopes.get(scope).expect("failed to find scope");
        match scope_value.decls.get(&ident) {
            Some(&decl) => Some(decl),
            None => match scope_value.parent {
                Some(parent) => self.search_scope(parent, expr, ident),
                None => None,
            },
        }
    }
}
