# pkloong-kcapi-sys

## 简介 / Overview

`pkloong-kcapi-sys` 是 `libkcapi` 的 Rust 低级绑定 crate，提供 FFI 接口与自动构建集成。\
`pkloong-kcapi-sys` is a low-level Rust binding crate for `libkcapi`, providing FFI APIs and automated build integration.

## 功能 / Features

- 构建阶段自动编译 vendored `libkcapi`\
  Builds vendored `libkcapi` during build time
- 使用 `bindgen` 生成 Rust 绑定\
  Generates Rust bindings via `bindgen`
- 通过 feature 控制能力集（`md`、`sym`、`aead`、`rng`、`kdf`、`asym`、`kpp` 等）\
  Exposes feature-gated capability sets (`md`, `sym`, `aead`, `rng`, `kdf`, `asym`, `kpp`, etc.)

## 构建要求 / Build Requirements

需要 Linux、C 工具链、`autotools`、`clang/libclang` 与内核头文件环境。\
Requires Linux, a C toolchain, `autotools`, `clang/libclang`, and kernel headers.

## 快速开始 / Quick Start

### 安装（crates.io）/ Installation (crates.io)

推荐使用：\
Recommended:

```bash
cargo add pkloong-kcapi-sys
```

或在 `Cargo.toml` 中添加：\
Or add in `Cargo.toml`:

```toml
[dependencies]
pkloong-kcapi-sys = "0.1.0"
```

### 构建命令 / Build Commands

在 workspace 根目录执行：\
Run from workspace root:

```bash
cargo build -p pkloong-kcapi-sys
```

按需启用功能：\
Enable features on demand:

```bash
cargo build -p pkloong-kcapi-sys --no-default-features --features "md,sym,rng"
```

## 使用示例 / Usage Example

### FFI 示例 / FFI Example

示例：SM3 FFI 调用，返回 32 代表成功。\
Example: SM3 FFI call; return value 32 means success.

```rust
use pkloong_kcapi_sys::kcapi_md_sm3;

fn main() {
    let msg = b"hello world";
    let mut digest = [0u8; 32];

    let ret = unsafe { kcapi_md_sm3(msg.as_ptr(), msg.len(), digest.as_mut_ptr(), digest.len()) };

    if ret == 32 {
        println!("sm3 ok: {:02x?}", digest);
    } else {
        eprintln!("sm3 failed: {}", ret);
    }
}
```

## 发布顺序 / Publish Order

### 发布提示 / Publishing Note

若与上层 crate 一同发布，请先发布本 crate，再发布 `pkloong-kcapi`。\
If publishing with the higher-level crate, publish this crate first, then `pkloong-kcapi`.

## 许可证 / License

双许可证：`BSD-2-Clause OR GPL-2.0-only`，可二选一。若选择 GPLv2 且对外分发，通常需要以 GPLv2 兼容方式分发并提供完整对应源码（含修改），保留版权与许可证声明且不得附加限制。\
Dual license: `BSD-2-Clause OR GPL-2.0-only`, choose either. If GPLv2 is chosen for external distribution, you generally need GPLv2-compatible redistribution with complete corresponding source (including modifications), preserved notices, and no extra restrictions.

本 crate 对内置 `libkcapi` 明确选择 **BSD 许可路径**；按本 crate 发布与使用，不适用其 GPL 路径约束。\
This crate explicitly selects the **BSD licensing path** for vendored `libkcapi`; when distributed/used via this crate, the GPL path does not apply.

### 许可证文件 / License Files

- GPLv2：`LICENSE-GPL-2.0`
- BSD 2-Clause：`LICENSE-BSD-2-Clause`
- 上游代码：`libkcapi/` 按其原始许可证执行（`COPYING*`）\
  Upstream sources: `libkcapi/` remains under its original licenses (`COPYING*`)

## 作者 / Author

孙福龙（Fulong Sun），中国北京。\
Fulong Sun (孙福龙), Beijing, China.
