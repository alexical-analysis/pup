use crate::compiler::module::Module;
use crate::compiler::str_store::MStr;
use crate::index_vec::Indexer;
use crate::types::checked_ty::CheckedTy;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Decl(u32);

impl Indexer for Decl {
    fn index(&self) -> usize {
        self.0 as usize
    }
}

impl From<usize> for Decl {
    fn from(value: usize) -> Self {
        Decl(value as u32)
    }
}

pub enum DeclValue {
    Invalid(&'static str),
    Type(TypeDecl),
    Func(FuncDecl),
}

pub struct TypeDecl {
    pub name: MStr,
    pub ty: CheckedTy,
}

pub struct FuncDecl {
    pub name: MStr,
    pub params: Vec<MStr>,
    pub body: BlockExpr,
    pub ty: CheckedTy,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Expr(u32);

impl Indexer for Expr {
    fn index(&self) -> usize {
        self.0 as usize
    }
}

impl From<usize> for Expr {
    fn from(value: usize) -> Self {
        Expr(value as u32)
    }
}

pub enum ExprValue {
    Invalid(&'static str),
    Identifier(IdentifierExpr),
    Call(CallExpr),
    Param(ParamExpr),
    Block(BlockExpr),
    Return(ReturnExpr),
    If(IfExpr),
    Loop(LoopExpr),
    Range(RangeExpr),
    Break,
    Binary(BinaryExpr),
    IntLiteral(i64),
    FloatLiteral(f64),
    BoolLiteral(bool),
}

pub struct IdentifierExpr {
    pub module: Module,
    pub name: MStr,
}

pub struct CallExpr {
    pub func: Expr,
    pub args: Vec<Expr>,
}

pub struct ParamExpr {
    pub name: MStr,
    pub ty: CheckedTy,
}

pub struct BlockExpr {
    pub exprs: Vec<Expr>,
}

pub struct ReturnExpr {
    pub value: Option<Expr>,
}

pub struct IfExpr {
    pub check: Expr,
    pub success: BlockExpr,
}

pub struct LoopExpr {
    pub body: BlockExpr,
}

pub struct RangeExpr {
    pub start: Expr,
    pub end: Expr,
}

pub struct BinaryExpr {
    pub left: Expr,
    pub operator: Operator,
    pub right: Expr,
}

#[derive(Clone, Copy)]
pub enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide,
    Equal,
    LessThan,
}
