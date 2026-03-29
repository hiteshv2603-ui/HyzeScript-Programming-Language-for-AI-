// HyzeScript/compiler/src/ir.rs

use crate::ast;

#[derive(Debug)]
pub struct IrProgram {
    pub nodes: Vec<IrNode>,
}

#[derive(Debug)]
pub enum IrNode {
    Let {
        name: String,
        ty: IrType,
        expr: IrExpr,
    },
    Call {
        callee: IrCallee,
        args: Vec<IrExpr>,
    },
    Print(IrExpr),
    IpuBlock(Vec<IrNode>),
    Loss {
        name: String,
        pred: IrExpr,
        target: IrExpr,
    },
    Backward(IrExpr),
}

#[derive(Debug)]
pub enum IrType {
    Tensor { dims: Vec<usize> },
}

#[derive(Debug)]
pub enum IrCallee {
    Tensor,
    Linear,
    ReLU,
    Sequential,
    MatMul,
}

#[derive(Debug)]
pub enum IrExpr {
    Number(f64),
    Var(String),
    Tensor(Vec<f64>),
    Call {
        callee: IrCallee,
        args: Vec<IrExpr>,
    },
    Unary {
        op: UnaryOp,
        expr: Box<IrExpr>,
    },
    Binary {
        left: Box<IrExpr>,
        right: Box<IrExpr>,
        op: BinaryOp,
    },
    Mse {
        pred: Box<IrExpr>,
        target: Box<IrExpr>,
    },
}

#[derive(Debug, Clone)]
pub enum UnaryOp {
    Neg,
}

#[derive(Debug, Clone)]
pub enum BinaryOp {
    Add,
    Mul,
}
