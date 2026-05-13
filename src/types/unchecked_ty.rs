use crate::compiler::str_store::MStr;

#[derive(Clone, Copy)]
pub struct UncheckedTy(u32);

impl From<usize> for UncheckedTy {
    fn from(value: usize) -> Self {
        UncheckedTy(value as u32)
    }
}

#[derive(PartialEq, Eq, Hash)]
pub enum UncheckedTyValue {
    Panic,
    Unit,
    ILiteral,
    I32,
    I64,
    FLiteral,
    F32,
    F64,
    Bool,
    Named(NamedTy),
}

#[derive(PartialEq, Eq, Hash)]
pub struct NamedTy {
    pub module: Option<MStr>,
    pub name: MStr,
}
