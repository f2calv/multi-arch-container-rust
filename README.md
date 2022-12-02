# Multi-Architecture Container Image for a Rust app

I wish to run a Rust application in a container on the follow infrastructures;

- Azure Kubernetes Service (AKS) (AMD64)
- Raspberry Pi 4 (ARM v7)
- Raspberry Pi 2 (ARM v6)

This is how to do it...

https://www.docker.com/blog/cross-compiling-rust-code-for-multiple-architectures/

```bash
#https://rust-lang.github.io/rustup/cross-compilation.html
#https://doc.rust-lang.org/nightly/rustc/platform-support.html

rustup target list
rustup target list | grep windows

rustup target add aarch64-unknown-linux-gnu
rustup target add aarch64-unknown-none
```