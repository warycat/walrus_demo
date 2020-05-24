mod ast;

use wasm_bindgen::prelude::*;
use walrus::{FunctionBuilder, InstrSeqBuilder, Module, ModuleConfig, ValType};
#[macro_use]
extern crate lalrpop_util;
lalrpop_mod!(pub calculator1); // syntesized by LALRPOP

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

struct Compiler{
    module: Module,
    src: String,
}

impl Compiler {
    fn new(src: String) -> Self{
        let config = ModuleConfig::new();
        let module = Module::with_config(config);
        Compiler{
            module,
            src,
        }
    }

    fn compile(&mut self) -> Vec<u8>{
        let expr = calculator1::ExprParser::new().parse(&self.src).unwrap();
        let mut function_builder = FunctionBuilder::new(&mut self.module.types, &[], &[ValType::I32]);
        let mut expr_builder: InstrSeqBuilder = function_builder.func_body();
        expr.emit(&mut expr_builder);
        self.module.exports.add("main", function_builder.finish(vec![], &mut self.module.funcs));
        self.module.emit_wasm()
    }
}

#[wasm_bindgen]
pub fn code2wasm(src: String) -> Vec<u8> {
    let mut compiler = Compiler::new(src);
    compiler.compile()
}
