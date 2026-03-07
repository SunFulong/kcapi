# pkloong-kcapi Workspace

## 简介 / Overview

`pkloong-kcapi` 是一个 Rust workspace，提供 Linux Kernel Crypto API（`libkcapi`）的底层绑定与高层封装。\
`pkloong-kcapi` is a Rust workspace that provides both low-level bindings and a high-level wrapper for Linux Kernel Crypto API (`libkcapi`).

## 组成 / Crates

- `pkloong-kcapi-sys`：底层 FFI 绑定与 `libkcapi` 构建集成。\
  `pkloong-kcapi-sys`: low-level FFI bindings and `libkcapi` build integration.
- `pkloong-kcapi`：更安全、易用的高层 API（消息杂凑值/鉴别码）。\
  `pkloong-kcapi`: safer high-level API (Digest/HMAC).

## 特性开关 / Feature Flags

- `md`：启用消息杂凑值与鉴别码模块 [`md`]。\
  `md`: enables digest and HMAC module [`md`].
- `all`：聚合特性，当前等价于启用 `md`。\
  `all`: aggregate feature, currently equivalent to enabling `md`.

## 快速开始 / Quick Start

### 通过 crates.io 使用 / Use via crates.io

添加依赖：\
Add dependency:

```bash
cargo add pkloong-kcapi
```

或手动编辑 `Cargo.toml`：\
Or edit `Cargo.toml` manually:

```toml
[dependencies]
pkloong-kcapi = "0.1.0"
```

### 本地构建 / Local Build

在 workspace 根目录执行：\
Run from workspace root:

```bash
cargo build
cargo test
```

## 文档入口 / Docs

### 相关文档 / Related Docs

- 高层 crate：`kcapi/README.md`\
  High-level crate: `kcapi/README.md`
- 底层 crate：`kcapi-sys/README.md`\
  Low-level crate: `kcapi-sys/README.md`

## 许可证 / License

双许可证：`BSD-2-Clause OR GPL-2.0-only`，可二选一。若选择 GPLv2 且对外分发，通常需要以 GPLv2 兼容方式分发并提供完整对应源码（含修改），同时保留版权与许可证声明且不得附加限制。\
Dual license: `BSD-2-Clause OR GPL-2.0-only`, choose either. If GPLv2 is chosen for external distribution, you generally need GPLv2-compatible redistribution with complete corresponding source (including modifications), preserved copyright/license notices, and no extra restrictions.

本仓库对内置 `libkcapi` 明确选择 **BSD 许可路径**；按本仓库发布与使用，不适用其 GPL 路径约束。\
This repository explicitly selects the **BSD licensing path** for vendored `libkcapi`; when distributed/used via this repository, the GPL path does not apply.

### 许可证文件 / License Files

- GPLv2：`LICENSE-GPL-2.0`
- BSD 2-Clause：`LICENSE-BSD-2-Clause`
- 上游 `libkcapi` 代码按其原始许可证执行。\
  Upstream `libkcapi` sources remain under their original licenses.

## 作者 / Author

孙福龙（Fulong Sun），中国北京。\
Fulong Sun (孙福龙), Beijing, China.
