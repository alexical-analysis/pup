#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct CheckedTy(pub u32);

#[derive(PartialEq, Eq, Hash)]
pub enum CheckedTyValue {
    Unit,
    I32,
    I64,
    F32,
    F64,
    Bool,
    Named(CheckedTy),
}
