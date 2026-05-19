use crate::hir::hir::{Decl, DeclValue, Expr, ExprValue};
use crate::hir::noder::Noder;
use crate::types::checked_ty::{CheckedTy, CheckedTyValue, FuncTy, NamedTy};
use crate::types::unchecked_ty::{UncheckedTy, UncheckedTyValue};

pub struct Checker {
    return_ty: CheckedTy,
}

impl Checker {
    pub fn new() -> Self {
        Self {
            // TODO: this is probably not the right way to do this...
            return_ty: CheckedTy::from(0),
        }
    }

    pub fn check_decl(&self, noder: &mut Noder, decl: Decl) {
        let decl_value = noder
            .module
            .hir_store
            .decls
            .get(decl)
            .expect("failed to find hir decl value");

        match decl_value {
            DeclValue::Invalid(_) => {}
            DeclValue::Type(_) => {}
            DeclValue::Func(v) => {
                // this clone is needed to prevent the noder borrow from continuing
                for expr in v.body.exprs.clone() {
                    self.check_expr(noder, expr);
                }
            }
        }
    }

    fn check_expr(&self, noder: &mut Noder, expr: Expr) {
        let expr_value = noder
            .module
            .hir_store
            .exprs
            .get(expr)
            .expect("failed to get hir expr value");

        match expr_value {
            ExprValue::Invalid(_) => noder.map_ty(expr, CheckedTyValue::Panic),
            ExprValue::Identifier(_) => todo!("check type for identifier expression"),
            ExprValue::Call(_) => todo!("check type for call expressions"),
            ExprValue::Block(_) => noder.map_ty(expr, CheckedTyValue::Unit),
            ExprValue::Return(_) => todo!("check type for return expression"),
            ExprValue::If(_) => noder.map_ty(expr, CheckedTyValue::Unit),
            ExprValue::Loop(_) => noder.map_ty(expr, CheckedTyValue::Unit),
            ExprValue::Range(_) => todo!("check type for range expression"),
            ExprValue::Break => noder.map_ty(expr, CheckedTyValue::Unit),
            ExprValue::Binary(_) => todo!("check type for binary expression"),
            ExprValue::IntLiteral(_) => noder.map_ty(expr, CheckedTyValue::I64),
            ExprValue::FloatLiteral(_) => noder.map_ty(expr, CheckedTyValue::F64),
            ExprValue::BoolLiteral(_) => noder.map_ty(expr, CheckedTyValue::Bool),
        }
    }
}

pub fn check_single_type(noder: &mut Noder, ty: UncheckedTy) -> CheckedTy {
    let ty_value = noder.module.ast_store.get_ty_value(ty);
    let checked_ty = match &ty_value {
        &UncheckedTyValue::Panic => CheckedTyValue::Panic,
        &UncheckedTyValue::Unit => CheckedTyValue::Unit,
        // if there's no value to check the int literal against then just default to an i64
        &UncheckedTyValue::ILiteral => CheckedTyValue::I64,
        &UncheckedTyValue::I64 => CheckedTyValue::I64,
        // if there's no value to check the float literal against then just default to an f64
        &UncheckedTyValue::FLiteral => CheckedTyValue::F64,
        &UncheckedTyValue::F64 => CheckedTyValue::F64,
        &UncheckedTyValue::Bool => CheckedTyValue::Bool,
        &UncheckedTyValue::Named(ty) => {
            let module = match ty.module {
                Some(name) => noder.find_module(name),
                None => noder.module.module,
            };

            CheckedTyValue::Named(NamedTy {
                module,
                name: ty.name,
            })
        }
        &UncheckedTyValue::Func(func) => {
            let mut params = vec![];
            for p in &func.params {
                let checked = check_single_type(noder, *p);
                params.push(checked)
            }

            let return_ty = check_single_type(noder, func.return_ty);

            CheckedTyValue::Func(FuncTy { params, return_ty })
        }
    };

    noder.get_ty(checked_ty)
}
