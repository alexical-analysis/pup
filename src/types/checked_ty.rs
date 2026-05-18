use crate::compiler::str_store::MStr;
use crate::index_vec::Indexer;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CheckedTy(pub u32);

impl Indexer for CheckedTy {
    fn index(&self) -> usize {
        self.0 as usize
    }
}

impl From<usize> for CheckedTy {
    fn from(value: usize) -> Self {
        CheckedTy(value as u32)
    }
}

#[derive(PartialEq, Eq, Hash)]
pub enum CheckedTyValue {
    Unit,
    I32,
    I64,
    F32,
    F64,
    Bool,
    Named(CheckedTy),
    Function(FunctionType),
}

#[derive(PartialEq, Eq, Hash)]
pub struct FunctionType {
    params: Vec<FunctionParam>,
    return_type: CheckedTy,
}

#[derive(PartialEq, Eq, Hash)]
pub struct FunctionParam {
    name: MStr,
    ty: CheckedTy,
}
