pub struct CheckedTy(u32);

pub enum CheckedTyValue {
    Unit,
    I32,
    I64,
    F32,
    F64,
    Bool,
    Named(CheckedTy),
}
