# pkloong-kcapi

## 简介 / Overview

`pkloong-kcapi` 是 Linux Kernel Crypto API 的 Rust 高层封装，当前提供消息杂凑值与鉴别码接口。\
`pkloong-kcapi` is a high-level Rust wrapper for Linux Kernel Crypto API, currently providing Digest/HMAC APIs.

## 功能 / Features

- 杂凑值：`sm3`、`sha1`、`sha224`、`sha256`、`sha384`、`sha512`\
  Digest: `sm3`, `sha1`, `sha224`, `sha256`, `sha384`, `sha512`
- 鉴别码：以上算法对应 HMAC 接口\
  HMAC: matching HMAC APIs for the above algorithms

## 特性开关 / Feature Flags

- `md`：启用消息杂凑值与鉴别码模块 [`md`]。\
  `md`: enables digest and HMAC module [`md`].
- `all`：聚合特性，当前等价于启用 `md`。\
  `all`: aggregate feature, currently equivalent to enabling `md`.

## 快速开始 / Quick Start

### 安装（crates.io）/ Installation (crates.io)

推荐使用：\
Recommended:

```bash
cargo add pkloong-kcapi
```

或在 `Cargo.toml` 中添加：\
Or add in `Cargo.toml`:

```toml
[dependencies]
pkloong-kcapi = "0.1.0"
```

### 构建与测试 / Build and Test

在 workspace 根目录执行：\
Run from workspace root:

```bash
cargo build -p pkloong-kcapi
cargo test -p pkloong-kcapi
```

## 最小示例 / Minimal Example

### 代码示例 / Code Example

示例：返回 `None` 表示底层调用失败。\
Example: `None` indicates lower-level failure.

```rust
use pkloong_kcapi::md::{hmac_sm3, sm3};

fn main() {
    let msg = b"hello world";
    let key = b"secret";
    println!("sm3: {:?}", sm3(msg));
    println!("hmac-sm3: {:?}", hmac_sm3(key, msg));
}
```

## 发布顺序 / Publish Order

### 发布提示 / Publishing Note

发布到 crates.io 时，请先发布 `pkloong-kcapi-sys`，待索引同步后再发布本 crate。\
For crates.io publishing, release `pkloong-kcapi-sys` first, then publish this crate after index propagation.

## 许可证 / License

双许可证：`BSD-2-Clause OR GPL-2.0-only`，可二选一。若选择 GPLv2 且对外分发，通常需要继续以 GPLv2 兼容条款分发并提供完整对应源码（含修改），保留版权与许可证声明且不得附加限制。\
Dual license: `BSD-2-Clause OR GPL-2.0-only`, choose either. If GPLv2 is chosen for external distribution, you generally need GPLv2-compatible redistribution with complete corresponding source (including modifications), preserved notices, and no extra downstream restrictions.

本 crate 对底层 `libkcapi` 明确选择 **BSD 许可路径**；按本 crate 发布与使用，不适用其 GPL 路径约束。\
This crate explicitly selects the **BSD licensing path** for underlying `libkcapi`; when distributed/used via this crate, the GPL path does not apply.

### 许可证文件 / License Files

- GPLv2：`LICENSE-GPL-2.0`
- BSD 2-Clause: `LICENSE-BSD-2-Clause`

## 作者 / Author

孙福龙（Fulong Sun），中国北京。\
Fulong Sun (孙福龙), Beijing, China.
