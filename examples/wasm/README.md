# WASM example

To build this example you need [wasm-pack](https://rustwasm.github.io/docs/book/game-of-life/setup.html).

Then simply run

```bash
wasm-pack build --target web
```

After that the wasm payload should be built. All we have to do now is serve the server:

### Python installed

If you have Python installed run the following:

```bash
python -m SimpleHTTPServer
```

Now you can visit the wasm in your browser at [127.0.0.1:8000/www](http://127.0.0.1:8000/www).

### No Python

Otherwise install `basic-http-server` and run it using that:

```bash
cargo install basic-http-server
basic-http-server
```

Now you can visit the wasm in your browser at [127.0.0.1:4000/www](http://127.0.0.1:4000/www).
