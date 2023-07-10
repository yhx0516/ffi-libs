# toml-rs
通过 rust ffi 方式编写的库集合

### rtoml - toml 的封装库
build cmd:
```
cargo build -p rtoml --release
```

### rpkg - .pkg 解析与文件搜索
build cmd:
```
cargo build -p rpg --release
```
run examples
```
cargo run -p rpkg --example pkg_addon_scan
cargo run -p rpkg --example pkg_norm_scan
```