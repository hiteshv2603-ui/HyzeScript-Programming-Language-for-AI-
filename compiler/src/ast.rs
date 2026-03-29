// HyzeScript/compiler/src/ast.rs

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug)]
pub enum Statement {
    LetDecl {
        name: String,
        ty: Option<Type>,
        expr: Expr,
    },
    ExprStmt(Expr),
    IpuBlock(Vec<Statement>),
    PrintStmt(Expr),
}

#[derive(Debug)]
pub enum Type {
    Tensor { dims: Vec<usize> },
    Infer,
}

#[derive(Debug)]
pub enum Expr {
    Ident(String),
    Tensor(Vec<f64>),
    Call {
        callee: String,
        args: Vec<Expr>,
    },
    Unary {
        op: UnaryOp,
        expr: Box<Expr>,
    },
    Binary {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
    },
    ReLU(Box<Expr>),
    MatMul {
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Backward(Box<Expr>),
    LossMse {
        pred: Box<Expr>,
        target: Box<Expr>,
    },
}

#[derive(Debug)]
pub enum UnaryOp {
    Neg,
}

#[derive(Debug)]
pub enum BinaryOp {
    Add,
    Mul,
}
