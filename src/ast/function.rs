use super::*;

#[derive(Debug)]
pub struct Function {
    pub prototype: Prototype,
    pub expr: Box<Expr>,
}

impl Function {
    pub fn new(prototype: Prototype, expr: Box<Expr>) -> Self {
        Function { prototype, expr }
    }
}

impl Compile for Function {
    fn compile(
        &self,
        builder: &mut InstrSeqBuilder,
        local_ids: &HashMap<String, LocalId>,
        function_ids: &HashMap<String, FunctionId>,
    ) {
        self.prototype.compile(builder, local_ids, function_ids);
        self.expr.as_ref().compile(builder, local_ids, function_ids);
    }
}
