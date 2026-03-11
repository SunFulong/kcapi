//! `rng` 模块封装 Linux Kernel Crypto API 的随机数接口，用于获取内核生成的随机字节序列。\
//! The `rng` module wraps Linux Kernel Crypto API random-number interfaces to obtain kernel-generated random bytes.
//!
//! 当前提供的 [`get_bytes`] 会一次性申请并填充指定长度的缓冲区，成功返回 `Some(Vec<u8>)`，失败返回 `None`。\
//! The currently provided [`get_bytes`] allocates and fills a buffer of the requested length at once, returning `Some(Vec<u8>)` on success and `None` on failure.
//!
//! # 使用说明 / Usage Notes
//!
//! - `len` 可以为 `0`，此时成功时返回空向量。\
//!   `len` may be `0`, in which case an empty vector is returned on success.
//! - 返回 `None` 通常表示底层 `libkcapi` 未返回期望字节数。\
//!   `None` usually means the underlying `libkcapi` call did not return the expected number of bytes.
//!
//! # 示例 / Example
//!
//! ```rust
//! use pkloong_kcapi::rng::get_bytes;
//!
//! let out = get_bytes(32).expect("RNG failed");
//! assert_eq!(out.len(), 32);
//! ```

use pkloong_kcapi_sys::kcapi_rng_get_bytes;

/// 从 Linux Kernel Crypto API 获取指定长度的随机字节。\
/// Gets random bytes of the requested length from Linux Kernel Crypto API.
///
/// # 参数 / Parameters
///
/// - `len`：需要获取的随机字节长度。\
///   `len`: number of random bytes to retrieve.
///
/// # 返回 / Returns
///
/// 成功返回 `Some(Vec<u8>)`（长度为 `len`），失败返回 `None`。\
/// Returns `Some(Vec<u8>)` (with length `len`) on success, and `None` on failure.
///
/// # 示例 / Example
///
/// ```rust
/// use pkloong_kcapi::rng::get_bytes;
///
/// let bytes = get_bytes(16).expect("RNG failed");
/// assert_eq!(bytes.len(), 16);
/// ```
pub fn get_bytes(len: usize) -> Option<Vec<u8>> {
    let mut buf = vec![0u8; len];
    let ret = unsafe { kcapi_rng_get_bytes(buf.as_mut_ptr(), len) };

    if ret == len as isize { Some(buf) } else { None }
}
