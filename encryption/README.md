# Encryption



![](./encryption.png)



This is a WebAssembly module I built for the company I work for. The idea is to propose it to improve the 

performance of our chat module which is using javascript libs to handle encryption of messages. With the 

power of wasm we could abstract away that feature and provide a better approach in terms of speed for such 

task at the same time.



## What is WebAssembly ?

> WebAssembly (abbreviated *Wasm*) is a binary instruction format for a stack-based virtual machine. Wasm is designed as a portable compilation target for programming languages, enabling deployment on the web for client and server applications.



## Why WebAssembly ?



A web assembly module for the aforementioned task would bring us four important assets:



- Portabilty

- Reusabily

- Flexibility

- Encapsulation



Below you can find my annotations.



Somehow I ran into an issue when trying to build the project with the AES-GCM crate, 

fortunatelly it got solved after running the following commands:





`rustup update nightly`

`rustup update stable`

`cargo build`



Before compiling your rust lib to webassembly, make sure you have installed the wasm32-unknown-unknown target





`rustup target add wasm32-unknown-unknown`



Now include in your Cargo.tml file the following dependency



`getrandom = { version = "0.2", features = ["js"] }`



Then proceed to build into webassembly



`cargo build --target wasm32-unknown-unknown --release`



Now install wasm-bindgen-cli



`cargo install wasm-bindgen-cli`   



Finally:



`wasm-bindgen target/wasm32-unknown-unknown/release/encryption.wasm --target web --out-dir ./www --omit-default-module-path --no-typescript`



Another way to compile your web assembly module is to use wasm-pack



`cargo install wasm-pack`  

`wasm-pack build --target web` 



To see it working you can start a http server:



python -m SimpleHTTPServer