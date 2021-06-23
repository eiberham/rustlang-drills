# Encryption



![](./encryption.png)



This is a WebAssembly module I built for the company I work for. The idea is to propose it to improve the 
performance of our chat module which is using javascript libs to handle encryption of messages. With the 
power of wasm we could abstract away that feature and provide a better approach in terms of speed for such 
task at the same time.



## What is WebAssembly ?

> WebAssembly (abbreviated *Wasm*) is a binary instruction format for a stack-based virtual machine. Wasm is designed as a portable compilation target for programming languages, enabling deployment on the web for client and server applications.

## Benefits we'd get out of it :comet:

A web assembly module for the aforementioned task would bring us five important assets:


- Portabilty
- Reusabily
- Flexibility
- Encapsulation
- Speed


Below you can find my annotations.



Somehow I ran into an issue when trying to build the project with the AES-GCM crate, 
fortunatelly it got solved after running the following commands:

```console
foo@bar:~$ rustup update nightly
foo@bar:~$ rustup update stable
foo@bar:~$ cargo build
```


Before compiling your rust lib to webassembly, make sure you have installed the 
wasm32-unknown-unknown target:

```console
foo@bar:~$ rustup target add wasm32-unknown-unknown
```

Now include in your Cargo.tml file the following dependency

```console
getrandom = { version = "0.2", features = ["js"] }
```

Then proceed to build into webassembly

```console
foo@bar:~$ cargo build --target wasm32-unknown-unknown --release
```

Now install wasm-bindgen-cli

```console
foo@bar:~$ cargo install wasm-bindgen-cli   
```

Finally:

```console
foo@bar:~$ wasm-bindgen target/wasm32-unknown-unknown/release/encryption.wasm --target web --out-dir ./www --omit-default-module-path --no-typescript
```

Another way to compile your web assembly module is to use wasm-pack

```console
foo@bar:~$ cargo install wasm-pack
foo@bar:~$ wasm-pack build --target web
```

To see it working you can start a http server:

```console
foo@bar:~$ python -m SimpleHTTPServer
```

## Profiling :alarm_clock:

In order to benchmark this tool the approach carried out was the console.time function in javascript, the
last test got the following output:

**Asymmetric**: 43.134765625 ms
**Symmetric** : 0.260986328125 ms

## Contributing
Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.
