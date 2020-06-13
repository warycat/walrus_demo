mod expr;
mod function;
mod opcode;
mod prototype;

use std::collections::HashMap;
use std::fmt::{Debug, Error, Formatter};
use walrus::ir::*;
use walrus::FunctionId;
use walrus::InstrSeqBuilder;

pub use expr::*;
pub use function::*;
pub use opcode::*;
pub use prototype::*;

pub trait Compile {
    fn compile(
        &self,
        builder: &mut InstrSeqBuilder,
        local_ids: &HashMap<String, LocalId>,
        function_ids: &HashMap<String, FunctionId>,
    );
}
