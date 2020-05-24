var wabt = require("wabt")();
import stdin from './stdin.txt';
var features = {
  exceptions: false,
  mutable_globals: true,
  sat_float_to_int: false,
  sign_extension: false,
  simd: false,
  threads: false,
  multi_value: false,
  tail_call: false,
  bulk_memory: false,
  reference_types: false,
  console: console
};

import("../pkg/index.js").then(compiler => {
  var stdin_el = document.getElementById("stdin");
  stdin_el.textContent = stdin;
  var buffer = compiler.code2wasm(stdin);
  var wasm = new WebAssembly.Module(buffer);
  var module = wabt.readWasm(buffer, {readDebugNames: true});
  module.generateNames();
  module.applyNames();
  var wast = module.toText({
    foldExprs: true,
    inlineExport: false
  });
  document.getElementById("stdout").textContent = wast;
  const wasmInstance = new WebAssembly.Instance(wasm, features);
  const { main } = wasmInstance.exports;
  console.log(main())
}).catch(console.error);
