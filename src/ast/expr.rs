use super::*;

pub enum Expr {
    Number(f64),
    Variable(String),
    Op(Box<Expr>, Opcode, Box<Expr>),
    Call(String, Vec<Box<Expr>>),
    Error,
}

impl Debug for Expr {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Expr::*;
        match *self {
            Number(n) => write!(fmt, "{:?}", n),
            Op(ref l, op, ref r) => write!(fmt, "({:?} {:?} {:?})", l, op, r),
            Variable(ref name) => write!(fmt, "{:?}", name),
            Call(ref name, ref exprs) => write!(fmt, "{:?} {:?}", name, exprs),
            Error => write!(fmt, "error"),
        }
    }
}

impl Compile for Expr {
    fn compile(
        &self,
        builder: &mut InstrSeqBuilder,
        local_ids: &HashMap<String, LocalId>,
        function_ids: &HashMap<String, FunctionId>,
    ) {
        use self::Expr::*;
        match *self {
            Number(n) => {
                builder.f64_const(n);
            }
            Op(ref l, op, ref r) => {
                l.compile(builder, local_ids, function_ids);
                r.compile(builder, local_ids, function_ids);
                op.compile(builder, local_ids, function_ids);
            }
            Variable(ref name) => {
                let id = local_ids[name];
                builder.local_get(id);
            }
            Call(ref name, ref exprs) => {
                for expr in exprs {
                    expr.compile(builder, local_ids, function_ids);
                    // builder.local_set();
                }
                let id = function_ids[name];
                builder.call(id);
            }
            _ => {}
        }
    }
}
