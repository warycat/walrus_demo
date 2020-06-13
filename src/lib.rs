mod ast;

use std::collections::HashMap;
use walrus::FunctionId;
use walrus::*;
use wasm_bindgen::prelude::*;

#[macro_use]
extern crate lalrpop_util;
lalrpop_mod!(pub kaleidoscope); // syntesized by LALRPOP

use ast::Compile;

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

struct Compiler {
    module: Module,
    src: String,
}

impl Compiler {
    fn new(src: String) -> Self {
        let config = ModuleConfig::new();
        let module = Module::with_config(config);
        Compiler { module, src }
    }

    fn compile(&mut self) -> Vec<u8> {
        let functions = kaleidoscope::ProgramParser::new().parse(&self.src).unwrap();
        let mut function_ids: HashMap<String, FunctionId> = HashMap::new();
        for function in functions {
            let s = format!("{:?}", function);
            let mut params: Vec<ValType> = vec![];
            let mut args: Vec<LocalId> = vec![];
            let mut local_ids: HashMap<String, LocalId> = HashMap::new();
            for param in &function.prototype.params {
                params.push(ValType::F64);
                let id = self.module.locals.add(ValType::F64);
                local_ids.insert(param.to_string(), id);
                args.push(id);
            }
            let mut function_builder =
                FunctionBuilder::new(&mut self.module.types, &params, &[ValType::F64]);
            let mut builder: InstrSeqBuilder = function_builder.func_body();
            function.compile(&mut builder, &local_ids, &function_ids);
            let function_id = function_builder.finish(args, &mut self.module.funcs);
            function_ids.insert(function.prototype.name.to_string(), function_id);
            self.module
                .exports
                .add(&function.prototype.name, function_id);
            log(&s);
        }
        self.module.emit_wasm()
    }
}

#[wasm_bindgen]
pub fn code2wasm(src: String) -> Vec<u8> {
    let mut compiler = Compiler::new(src);
    compiler.compile()
}
