use std::fmt::{Debug, Error, Formatter};
use walrus::ir::*;
use walrus::InstrSeqBuilder;

pub enum Expr {
    Number(i32),
    Op(Box<Expr>, Opcode, Box<Expr>),
    Error,
}

#[derive(Copy, Clone)]
pub enum Opcode {
    Mul,
    Div,
    Add,
    Sub,
}

impl Debug for Expr {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Expr::*;
        match *self {
            Number(n) => write!(fmt, "{:?}", n),
            Op(ref l, op, ref r) => write!(fmt, "({:?} {:?} {:?})", l, op, r),
            Error => write!(fmt, "error"),
        }
    }
}

impl Debug for Opcode {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Opcode::*;
        match *self {
            Mul => write!(fmt, "*"),
            Div => write!(fmt, "/"),
            Add => write!(fmt, "+"),
            Sub => write!(fmt, "-"),
        }
    }
}

impl Expr {
    pub fn emit(&self, exprBuilder: &mut InstrSeqBuilder){
        use self::Expr::*;
        match *self {
            Number(n) => {
                exprBuilder.i32_const(n);
            },
            Op(ref l, op, ref r) => {
                l.emit(exprBuilder);
                r.emit(exprBuilder);
                op.emit(exprBuilder);
            }
            _ => {}
        }
    }
}

impl Opcode {
    pub fn emit(&self, exprBuilder: &mut InstrSeqBuilder){
        use self::Opcode::*;
        match *self {
            Mul => exprBuilder.binop(BinaryOp::I32Mul),
            Div => exprBuilder.binop(BinaryOp::I32DivS),
            Add => exprBuilder.binop(BinaryOp::I32Add),
            Sub => exprBuilder.binop(BinaryOp::I32Sub),
        };
    }
}