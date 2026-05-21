use std::collections::HashMap;

use crate::hir::hir::{Decl, DeclValue, Expr, ExprValue};
use crate::hir::noder::Noder;
use crate::hir::noder::sym_table::SymTable;
use crate::types::checked_ty::{CheckedTy, CheckedTyValue, FuncTy, NamedTy};
use crate::types::unchecked_ty::{UncheckedTy, UncheckedTyValue};

pub struct Checker {
    ident_map: HashMap<Expr, Decl>,
    return_ty: CheckedTy,
}

impl Checker {
    pub fn new(sym_table: SymTable) -> Self {
        Self {
            ident_map: sym_table.get_ident_map(),
            return_ty: CheckedTy::from(0),
        }
    }

    pub fn check_decl(&mut self, noder: &mut Noder, decl: Decl) {
        let decl_value = noder.get_hir_decl_value(decl).clone();

        match decl_value {
            DeclValue::Invalid(_) => {}
            DeclValue::Ty(_) => {}
            DeclValue::Func(v) => {
                let func_ty = noder.get_checked_ty_value(v.ty);
                let func_ty = match func_ty {
                    CheckedTyValue::Func(ty) => ty,
                    _ => panic!("invalid type for function decl"),
                };
                self.return_ty = func_ty.return_ty;

                // this clone is needed to prevent the noder borrow from continuing
                for expr in v.body.exprs.clone() {
                    self.check_expr(noder, expr);
                }
            }
        }
    }

    fn check_expr(&self, noder: &mut Noder, expr: Expr) -> CheckedTy {
        let expr_value = noder.get_hir_expr_value(expr);

        match expr_value.clone() {
            ExprValue::Invalid(_) => noder.map_ty_value(expr, CheckedTyValue::Panic),
            ExprValue::Identifier(v) => {
                let &decl = self
                    .ident_map
                    .get(&expr)
                    .expect("failed to find identifier in symbol table");
                let _decl_value = noder.get_hir_decl_value(decl);
                todo!("use the decl to figure out the type of the identifier")
            }
            ExprValue::Call(v) => {
                let &decl = self
                    .ident_map
                    .get(&v.func)
                    .expect("failed to find identifier in symbol table");
                let decl_value = noder.get_hir_decl_value(decl);
                let func_ty = match decl_value {
                    DeclValue::Func(func) => func.ty,
                    // TODO: need to fold this into the tree as invalid node
                    _ => todo!("calling non-function type"),
                };

                let ty_value = noder.get_checked_ty_value(func_ty).clone();
                let ty_value = match ty_value {
                    CheckedTyValue::Func(func) => func,
                    // TODO: need to fold this into the tree as invalid node
                    _ => todo!("invalid checked type for function"),
                };

                if ty_value.params.len() != v.args.len() {
                    // TODO: need to fold this into the tree as invalid node
                    todo!("invalid number of args for function call")
                }

                let mut arg_ty = vec![];
                for &arg in &v.args {
                    let ty = self.check_expr(noder, arg);
                    arg_ty.push(ty);
                }

                for (param_ty, arg_ty) in ty_value.params.iter().zip(arg_ty.iter()) {
                    if param_ty != arg_ty {
                        // TODO: need to fold this into the tree as invalid node
                        todo!("param type and arg type do not match")
                    }
                }

                noder.map_ty(expr, ty_value.return_ty);
            }
            ExprValue::Block(_) => noder.map_ty_value(expr, CheckedTyValue::Unit),
            ExprValue::Return(expr) => {
                if let Some(expr) = expr.value {
                    self.check_expr(noder, expr);
                }
            }
            ExprValue::If(_) => noder.map_ty_value(expr, CheckedTyValue::Unit),
            ExprValue::Loop(_) => noder.map_ty_value(expr, CheckedTyValue::Unit),
            ExprValue::Range(v) => {
                let start = self.check_expr(noder, v.start);
                let end = self.check_expr(noder, v.end);
                if start != end {
                    noder.map_ty_value(v.start, CheckedTyValue::Panic);
                    noder.map_ty_value(v.end, CheckedTyValue::Panic);
                    noder.map_ty_value(expr, CheckedTyValue::Panic);
                } else {
                    noder.map_ty_value(expr, CheckedTyValue::Range)
                }
            }
            ExprValue::Break => noder.map_ty_value(expr, CheckedTyValue::Unit),
            ExprValue::Binary(v) => {
                let left = self.check_expr(noder, v.left);
                let right = self.check_expr(noder, v.right);
                if left != right {
                    noder.map_ty_value(v.left, CheckedTyValue::Panic);
                    noder.map_ty_value(v.right, CheckedTyValue::Panic);
                    noder.map_ty_value(expr, CheckedTyValue::Panic);
                } else {
                    noder.map_ty(expr, left);
                }
            }
            ExprValue::IntLiteral(_) => noder.map_ty_value(expr, CheckedTyValue::I64),
            ExprValue::FloatLiteral(_) => noder.map_ty_value(expr, CheckedTyValue::F64),
            ExprValue::BoolLiteral(_) => noder.map_ty_value(expr, CheckedTyValue::Bool),
        };

        noder.get_expr_ty(expr)
    }
}

pub fn check_single_type(noder: &mut Noder, ty: UncheckedTy) -> CheckedTy {
    let ty_value = noder.get_unchecked_ty_value(ty).clone();
    let checked_ty = match ty_value {
        UncheckedTyValue::Panic => CheckedTyValue::Panic,
        UncheckedTyValue::Unit => CheckedTyValue::Unit,
        // if there's no value to check the int literal against then just default to an i64
        UncheckedTyValue::ILiteral => CheckedTyValue::I64,
        UncheckedTyValue::I64 => CheckedTyValue::I64,
        // if there's no value to check the float literal against then just default to an f64
        UncheckedTyValue::FLiteral => CheckedTyValue::F64,
        UncheckedTyValue::F64 => CheckedTyValue::F64,
        UncheckedTyValue::Bool => CheckedTyValue::Bool,
        UncheckedTyValue::Named(ty) => {
            let module = match ty.module {
                Some(name) => noder.find_dep(name),
                None => noder.module,
            };

            CheckedTyValue::Named(NamedTy {
                module,
                name: ty.name,
            })
        }
        UncheckedTyValue::Func(func) => {
            let mut params = vec![];
            for p in &func.params {
                let checked = check_single_type(noder, *p);
                params.push(checked)
            }

            let return_ty = check_single_type(noder, func.return_ty);

            CheckedTyValue::Func(FuncTy { params, return_ty })
        }
    };

    noder.get_checked_ty(checked_ty)
}
