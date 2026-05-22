use crate::{
    compiler::{mir_store::MirStore, str_store::MStr},
    hir::hir::FuncDecl,
    index_vec::Indexer,
    types::checked_ty::CheckedTy,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Value(u32);

impl From<usize> for Value {
    fn from(value: usize) -> Self {
        Self(value as u32)
    }
}

impl Indexer for Value {
    fn index(&self) -> usize {
        self.0 as usize
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Block(u32);

impl From<usize> for Block {
    fn from(value: usize) -> Self {
        Self(value as u32)
    }
}

impl Indexer for Block {
    fn index(&self) -> usize {
        self.0 as usize
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Local(u32);

impl From<usize> for Local {
    fn from(value: usize) -> Self {
        Self(value as u32)
    }
}

impl Indexer for Local {
    fn index(&self) -> usize {
        self.0 as usize
    }
}

pub struct LocalValue {
    name: MStr,
    initializer: Value,
}

pub enum Inst {
    Const(ConstValue),
    Call(CallInst),
    Add(AddInst),
    Sub(SubInst),
    Mul(MulInst),
    Div(DivInst),
    Equal(EqualInst),
    LessThan(LessThanInst),
}

pub enum ConstValue {
    Int(i64),
    Float(f64),
    Bool(bool),
}

pub struct CallInst {
    func: MStr,
    args: Vec<Value>,
}

pub struct AddInst {
    left: Value,
    right: Value,
}

pub struct SubInst {
    left: Value,
    right: Value,
}

pub struct MulInst {
    left: Value,
    right: Value,
}

pub struct DivInst {
    left: Value,
    right: Value,
}

pub struct EqualInst {
    left: Value,
    right: Value,
}

pub struct LessThanInst {
    left: Value,
    right: Value,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Terminator(u32);

impl From<usize> for Terminator {
    fn from(value: usize) -> Self {
        Self(value as u32)
    }
}

impl Indexer for Terminator {
    fn index(&self) -> usize {
        self.0 as usize
    }
}

pub enum TerminatorValue {
    Return(Option<Value>),
    Jump(Block),
    Branch(BranchTerminator),
    Panic(MStr),
    // none is a temporary terminator that should be fully removed by the graphing process
    None,
}

pub struct BranchTerminator {
    cond: Value,
    true_block: Block,
    false_block: Block,
}

pub struct BlockValue {
    inst_values: Vec<Value>,
    terminator: Terminator,
}

impl BlockValue {
    pub fn new(mir_store: &mut MirStore) -> Self {
        let terminator = mir_store.create_none_terminator();
        Self {
            inst_values: vec![],
            terminator,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Func(u32);

impl From<usize> for Func {
    fn from(value: usize) -> Self {
        Self(value as u32)
    }
}

impl Indexer for Func {
    fn index(&self) -> usize {
        self.0 as usize
    }
}

pub struct FuncValue {
    name: MStr,
    locals: Vec<Local>,
    blocks: Vec<Block>,
    ty: CheckedTy,
}

impl From<FuncDecl> for FuncValue {
    // TODO: this needs to be a new because we need to create a local for each param
    fn from(decl: FuncDecl) -> Self {
        for param in decl.params {}

        Self {
            name: decl.name,
            locals: vec![],
            blocks: vec![],
            ty: decl.ty,
        }
    }
}

impl FuncValue {
    pub fn push_block(&mut self, block: Block) {
        self.blocks.push(block);
    }
}
