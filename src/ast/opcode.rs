use super::*;
use std::collections::HashMap;
use std::fmt::{Debug, Error, Formatter};
use walrus::ir::*;
use walrus::InstrSeqBuilder;

#[derive(Copy, Clone)]
pub enum Opcode {
    Mul,
    Div,
    Add,
    Sub,
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

impl Compile for Opcode {
    fn compile(
        &self,
        builder: &mut InstrSeqBuilder,
        local_ids: &HashMap<String, LocalId>,
        function_ids: &HashMap<String, FunctionId>,
    ) {
        use self::Opcode::*;
        match *self {
            Mul => builder.binop(BinaryOp::F64Mul),
            Div => builder.binop(BinaryOp::F64Div),
            Add => builder.binop(BinaryOp::F64Add),
            Sub => builder.binop(BinaryOp::F64Sub),
        };
    }
}
