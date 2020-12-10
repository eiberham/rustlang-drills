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
cargo new --vcs none sample_project
```

