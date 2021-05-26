Somehow I ran into an issue when trying to build the project with the AES-GCM crate, 
fortunatelly it got solved after running the following commands:

```console
rustup update nightly
rustup update stable
cargo build
```

Before compiling your rust lib to webassembly, make sure you have installed the wasm32-unknown-unknown target

```console
rustup target add wasm32-unknown-unknown
```

Now include in your Cargo.tml file the following dependency

```console
getrandom = { version = "0.2", features = ["js"] }
```

Then proceed to build into webassembly

```console
    cargo build --target wasm32-unknown-unknown --release
```

To see it working you can start a http server:

```console
python -m SimpleHTTPServer
```
