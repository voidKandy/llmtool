## WASM
This crate in the `llmtool` workspace contains everything needed for running models. Currently there is only a single wasm binary: `bin/model.rs`, when built, the code in this file defines a `Model` class, which can generate embeddings once instantiated. Eventually this should be moved to a file called `bin/embedding_model.rs` and another `bin/completion_model.rs` should be created.
**To Build For Testing**:
```bash
sh build-lib.sh
```
**To Build And Move to Client**:
```bash
sh build-and-move.sh
```
### The example
`lib-example.html` allows you to test the wasm binary in the browser. It is meant only for testing as the models eventually need to be moved to be accessible via the `client` crate in this workspace.
To start a basic http server to test this out, run:
```bash
python -m http.server
```
Then open `http://localhost:8000/lib-example.html` in your browser.

#### Overview
While we haven't yet figured out how we are going to be deploying these wasm models through our dioxus app, I think creating web components that interract with webworkers would be our best bet. Currently `bertWorker.js` defines a web worker that handles any stuff regarding the model actually running inference.

[this was a very helpful resource for this](https://github.com/huggingface/candle/blob/main/candle-wasm-examples/bert)
