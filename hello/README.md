# Hello Rust

``` plain
Visual Studio Code

Extensions:

rust-lang.rust
https://github.com/rust-lang/vscode-rust

rust-analyzer
https://github.com/rust-analyzer/rust-analyzer

Native Debug
https://github.com/WebFreak001/code-debug
```

[Cargo](https://doc.rust-lang.org/stable/rust-by-example/cargo/deps.html)
[Hello World](https://doc.rust-lang.org/stable/rust-by-example/hello.html)

``` plain
$ cargo new hello
Created binary (application) `hello` package

$ tree
.
└── hello
    ├── Cargo.toml
    └── src
        └── main.rs

$ rustc ./hello/src/main.rs

$ ./main
Hello, world!
```
