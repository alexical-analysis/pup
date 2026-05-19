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

#[derive(Clone)]
pub enum DeclValue {
    Invalid(&'static str),
    Mod(ModDecl),
    Use(UseDecl),
    Ty(TyDecl),
    Func(FuncDecl),
}

#[derive(Clone)]
pub struct ModDecl {
    pub name: MStr,
}

#[derive(Clone)]
pub struct UseDecl {
    pub deps: Vec<MStr>,
}

#[derive(Clone)]
pub struct TyDecl {
    pub name: MStr,
    pub ty: UncheckedTy,
}

#[derive(Clone)]
pub struct FuncDecl {
    pub name: MStr,
    pub params: Vec<MStr>,
    pub body: BlockExpr,
    pub ty: UncheckedTy,
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

#[derive(Clone)]
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
