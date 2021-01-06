# Ratchet

[clap](https://github.com/clap-rs/clap)

[log4rs](https://github.com/estk/log4rs)

[rust-prometheus](https://github.com/tikv/rust-prometheus)  
[Crate prometheus](https://docs.rs/prometheus/0.11.0/prometheus/index.html)  

[lazy_static](https://docs.rs/lazy_static/1.4.0/lazy_static/)

[git-version](https://github.com/fusion-engineering/rust-git-version)

[reqwest](https://github.com/seanmonstar/reqwest)

[Crate serde](https://serde.rs/)
[Crate serde_yaml](https://github.com/dtolnay/serde-yaml)
[yaml-rust](https://github.com/chyh1990/yaml-rust)

## Build

``` plain
# build

$ cd github.com/wangfeiping/ratchet/

$ make

$ ./target/release/ratchet --version
```

## Running

``` plain
# running

$ cd ./target/release/

$ ./ratchet --log-level debug

$ curl http://127.0.0.1:8080/

$ curl http://127.0.0.1:8080/metrics
```

## New package

``` plain
# new

$ cargo new --lib common/ratchet_version
$ cargo new --lib common/prometheus
$ cargo new --lib common/logger
$ cargo new --lib watcher
```
