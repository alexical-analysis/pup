use crate::compiler::str_store::MStr;
use crate::types::unchecked_ty::UncheckedTy;

pub struct Decl(u32);

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
    pub name: MStr,
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

pub struct Expr(u32);

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
    BinaryExpr,
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

pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Equal,
    LessThan,
}
