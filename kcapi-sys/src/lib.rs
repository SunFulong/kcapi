//! `pkloong-kcapi-sys` 是 `libkcapi` 的 Rust 低级绑定 crate，提供 FFI 接口与自动构建集成。\
//! `pkloong-kcapi-sys` is a low-level Rust binding crate for `libkcapi`, providing FFI APIs and automated build integration.
//!
//! # 功能 / Features
//!
//! - 构建阶段自动编译 vendored `libkcapi`\
//!   Builds vendored `libkcapi` during build time
//! - 使用 `bindgen` 生成 Rust 绑定\
//!   Generates Rust bindings via `bindgen`
//! - 通过 feature 控制能力集（`md`、`sym`、`aead`、`rng`、`kdf`、`asym`、`kpp` 等）\
//!   Exposes feature-gated capability sets (`md`, `sym`, `aead`, `rng`, `kdf`, `asym`, `kpp`, etc.)
//!
//! # 使用示例 / Usage Example
//!
//! ## FFI 示例 / FFI Example
//!
//! 示例：SM3 FFI 调用，返回 32 代表成功。\
//! Example: SM3 FFI call; return value 32 means success.
//!
//! ```rust
//! use pkloong_kcapi_sys::kcapi_md_sm3;
//!
//! fn main() {
//!     let msg = b"hello world";
//!     let mut digest = [0u8; 32];
//!
//!     let ret = unsafe { kcapi_md_sm3(msg.as_ptr(), msg.len(), digest.as_mut_ptr(), digest.len()) };
//!
//!     if ret == 32 {
//!         println!("sm3 ok: {:02x?}", digest);
//!     } else {
//!         eprintln!("sm3 failed: {}", ret);
//!     }
//! }
//! ```
//!
//! # 许可证 / License
//!
//! 双许可证：`BSD-2-Clause OR GPL-2.0-only`，可二选一。若选择 GPLv2 且对外分发，通常需要以 GPLv2 兼容方式分发并提供完整对应源码（含修改），保留版权与许可证声明且不得附加限制。\
//! Dual license: `BSD-2-Clause OR GPL-2.0-only`, choose either. If GPLv2 is chosen for external distribution, you generally need GPLv2-compatible redistribution with complete corresponding source (including modifications), preserved notices, and no extra restrictions.
//!
//! 本 crate 对内置 `libkcapi` 明确选择 **BSD 许可路径**；按本 crate 发布与使用，不适用其 GPL 路径约束。\
//! This crate explicitly selects the **BSD licensing path** for vendored `libkcapi`; when distributed/used via this crate, the GPL path does not apply.
//!
//! # 作者 / Author
//!
//! 孙福龙（Fulong Sun），中国北京。\
//! Fulong Sun (孙福龙), Beijing, China.

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        let version = unsafe { kcapi_version() };
        assert_eq!(version, 1_05_00_00);
    }
}
