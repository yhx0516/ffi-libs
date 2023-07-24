# toml-rs
通过 rust ffi 方式编写的库集合

---
### rtoml - toml 的封装库
build cmd
- win64
```
cross build --release -p rtoml
```

- android
```
cross build --target aarch64-unknown-linux-gnu --release -p rtoml
```

- linux
```
cross build --target x86_64-unknown-linux-gnu --release -p rtoml
```

- macos
```
cargo build --target x86_64-apple-darwin --release -p rtoml

cargo build --target aarch64-apple-darwin --release -p rtoml
```

- ios
```
cargo build --target aarch64-apple-ios --release -p rtoml
```

---
### rpkg - .pkg 解析与文件搜索
build cmd
- win64
```
cross build --release -p rpkg
```

- android
```
cross build --target aarch64-unknown-linux-gnu --release -p rpkg
```

- linux
```
cross build --target x86_64-unknown-linux-gnu --release -p rpkg
```

- macos
```
cargo build --target x86_64-apple-darwin --release -p rpkg

cargo build --target aarch64-apple-darwin --release -p rpkg
```

- ios
```
cargo build --target aarch64-apple-ios --release -p rpkg
```

run examples
```
cargo run -p rpkg --example pkg_addon_scan
cargo run -p rpkg --example pkg_norm_scan
```

---
### rhandlebars - handlebars 的封装库
build cmd
- win64
```
cross build --release -p rhandlebars
```

- android
```
cross build --target aarch64-unknown-linux-gnu --release -p rhandlebars
```

- linux
```
cross build --target x86_64-unknown-linux-gnu --release -p rhandlebars
```

- macos
```
cargo build --target x86_64-apple-darwin --release -p rhandlebars

cargo build --target aarch64-apple-darwin --release -p rhandlebars
```

- ios
```
cargo build --target aarch64-apple-ios --release -p rhandlebars
```