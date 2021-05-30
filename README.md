# Rust lang exercises

## Personal tips and drills in my journey as a beginner rustacean

To install rust lang run:

```console
$ curl https://sh.rustup.rs -sSf | sh -s -- --no-modify-path 
$ sudo vim $HOME/.bash_profile
```

Then add `source ~/.cargo/env` to your .bash_profile

To check rust version run: 

```console
$ rustc --version
```

To read the rust documentation locally in the browser run `rustup doc`

***Cargo*** is the dependency management tool for rust lang, to check the cargo version simply run:

```console
$ cargo --version
```

To create a new cargo project run:

```console
$ cargo new sample_project
```

To create a new cargo project without git init:

```console
$ cargo new --vcs none sample_project
```

### Building & executing programs

In order to build the program run the following command:

```console
$ cargo build
```

This command will generate the ***Cargo.lock*** file which keeps track of the dependencies in the program and also will generate an executable file which is either for release or debug depending on the presence of the ***--release*** flag.

If you want to build & run in one step you can use the following command:

```console
$ cargo run
```

To check if your program compiles simply run check which is faster than a build as it does not generate an executable file:

```console
$ cargo check
```

### Installing crates locally

In order to install a crate into your project you have to type the crate name and the desired version in you Cargo.toml file:

```console
[dependencies]
rsa = "0.4.0"
```

### Fix warnings with rustfix

Rustfix comes by default in every rust installation so, in order to fix your code simply run:

```console
cargo fix
```

### Improve your code by using Clippy

To install clipy run 

```console
rustup component add clippy
````

then you can run it by entering cargo clippy in the console.