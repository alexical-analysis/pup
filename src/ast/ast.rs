use crate::compiler::str_store::MStr;
use crate::index_vec::Indexer;
use crate::types::unchecked_ty::UncheckedTy;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Decl(u32);

impl Indexer for Decl {
    fn index(&self) -> usize {
        self.0 as usize
    }
}

impl Decl {
    pub fn index(&self) -> usize {
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
    Mod(ModDecl),
    Use(UseDecl),
    Type(TypeDecl),
    Function(FunctionDecl),
}

pub struct ModDecl {
    pub name: MStr,
}

pub struct UseDecl {
    pub deps: Vec<MStr>,
}

pub struct TypeDecl {
    pub name: MStr,
    pub ty: UncheckedTy,
}

pub struct FunctionDecl {
    pub name: MStr,
    pub params: Vec<ParamExpr>,
    pub body: BlockExpr,
    pub return_ty: UncheckedTy,
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
    pub module: Option<MStr>,
    pub name: MStr,
}

pub struct CallExpr {
    pub func: Expr,
    pub args: Vec<Expr>,
}

pub struct ParamExpr {
    pub name: MStr,
    pub ty: UncheckedTy,
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
