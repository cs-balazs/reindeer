import("./wasm/hello_world.js")
  .then((wasm) => {
    wasm.default();
    wasm.run();
  })
  .catch(console.error);
