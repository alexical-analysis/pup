use crate::hir::noder::Noder;
use crate::types::checked_ty::{CheckedTy, CheckedTyValue, FuncTy, NamedTy};
use crate::types::unchecked_ty::{UncheckedTy, UncheckedTyValue};

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
