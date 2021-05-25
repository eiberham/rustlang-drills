Somehow I ran into an issue when trying to build the project with the AES-GCM crate, 
fortunatelly it got solved after running the following commands:

```console
rustup update nightly
rustup update stable
cargo build
```