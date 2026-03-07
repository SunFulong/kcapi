//! `md` 模块封装 Linux Kernel Crypto API 的消息杂凑值（Digest）与鉴别码（HMAC）便捷接口，当前支持 `SM3`、`SHA1`、`SHA2`、`SHA3`，以及`SM3`、`SHA1`、`SHA2` 算法对应的 HMAC。\
//! The `md` module provides convenient wrappers for digest and HMAC operations backed by Linux Kernel Crypto API, currently covering `SM3`, `SHA1`, `SHA2`, `SHA3`, plus HMAC variants for the `SM3`, `SHA1`, `SHA2` algorithms.
//!
//! 本模块所有函数都返回 `Option<[u8; N]>`：成功时返回 `Some(digest)`，失败时返回 `None`。失败通常意味着底层 `libkcapi` 调用未返回预期输出长度。\
//! All functions return `Option<[u8; N]>`: `Some(digest)` on success and `None` on failure. Failure usually means the underlying `libkcapi` call did not produce the expected output size.
//!
//! 杂凑值长度常量（如 `SM3_DIGEST_SIZE`）可用于预分配、协议字段校验或与其他实现做一致性检查。\
//! Digest-size constants (for example `SM3_DIGEST_SIZE`) are useful for pre-allocation, protocol field validation, and cross-implementation consistency checks.
//!
//! # 使用说明 / Usage Notes
//!
//! 调用方应将 `None` 视为可恢复错误并自行处理，例如重试、记录日志或回退到其他实现。\
//! Callers should treat `None` as a recoverable error and handle it explicitly, such as retrying, logging, or falling back to another implementation.
//!
//! # 示例 / Example
//!
//! ```rust
//! use pkloong_kcapi::md::{hmac_sm3, sm3};
//!
//! let message = b"hello world";
//! let key = b"secret";
//! println!("sm3: {:?}", sm3(message));
//! println!("hmac-sm3: {:?}", hmac_sm3(key, message));
//! ```

use pkloong_kcapi_sys::{
    kcapi_md_hmac_sha1, kcapi_md_hmac_sha224, kcapi_md_hmac_sha256, kcapi_md_hmac_sha384,
    kcapi_md_hmac_sha512, kcapi_md_hmac_sm3, kcapi_md_sha1, kcapi_md_sha3_224, kcapi_md_sha3_256,
    kcapi_md_sha3_384, kcapi_md_sha3_512, kcapi_md_sha224, kcapi_md_sha256, kcapi_md_sha384,
    kcapi_md_sha512, kcapi_md_sm3,
};

/// SM3 杂凑值长度（字节）。\
/// SM3 digest length in bytes.
pub const SM3_DIGEST_SIZE: usize = 32;
/// SHA1 杂凑值长度（字节）。\
/// SHA1 digest length in bytes.
pub const SHA1_DIGEST_SIZE: usize = 20;
/// SHA224 杂凑值长度（字节）。\
/// SHA224 digest length in bytes.
pub const SHA224_DIGEST_SIZE: usize = 28;
/// SHA256 杂凑值长度（字节）。\
/// SHA256 digest length in bytes.
pub const SHA256_DIGEST_SIZE: usize = 32;
/// SHA384 杂凑值长度（字节）。\
/// SHA384 digest length in bytes.
pub const SHA384_DIGEST_SIZE: usize = 48;
/// SHA512 杂凑值长度（字节）。\
/// SHA512 digest length in bytes.
pub const SHA512_DIGEST_SIZE: usize = 64;
/// SHA3-224 杂凑值长度（字节）。\
/// SHA3-224 digest length in bytes.
pub const SHA3_224_DIGEST_SIZE: usize = 28;
/// SHA3-256 杂凑值长度（字节）。\
/// SHA3-256 digest length in bytes.
pub const SHA3_256_DIGEST_SIZE: usize = 32;
/// SHA3-384 杂凑值长度（字节）。\
/// SHA3-384 digest length in bytes.
pub const SHA3_384_DIGEST_SIZE: usize = 48;
/// SHA3-512 杂凑值长度（字节）。\
/// SHA3-512 digest length in bytes.
pub const SHA3_512_DIGEST_SIZE: usize = 64;

/// 计算输入消息的 SM3 杂凑值。\
/// Computes the SM3 digest for the input message.
///
/// # 返回 / Returns
///
/// 成功返回 `Some(digest)`，失败返回 `None`。\
/// Returns `Some(digest)` on success and `None` on failure.
///
/// # 示例 / Example
///
/// ```
/// use pkloong_kcapi::md::sm3;
///
/// let digest = sm3(b"hello world").expect("SM3 calculation failed");
/// assert_eq!(digest.as_slice(), [
///     0x44, 0xF0, 0x06, 0x1E, 0x69, 0xFA, 0x6F, 0xDF, 0xC2, 0x90, 0xC4, 0x94, 0x65, 0x4A, 0x05,
///     0xDC, 0x0C, 0x05, 0x3D, 0xA7, 0xE5, 0xC5, 0x2B, 0x84, 0xEF, 0x93, 0xA9, 0xD6, 0x7D, 0x3F,
///     0xFF, 0x88,
/// ].as_slice());
/// ```
pub fn sm3(message: &[u8]) -> Option<[u8; SM3_DIGEST_SIZE]> {
    let mut digest = [0u8; SM3_DIGEST_SIZE];
    let result = unsafe {
        kcapi_md_sm3(
            message.as_ptr(),
            message.len(),
            digest.as_mut_ptr(),
            digest.len(),
        )
    };

    if result == SM3_DIGEST_SIZE as isize {
        Some(digest)
    } else {
        None
    }
}

/// 使用给定密钥计算输入消息的 HMAC-SM3。\
/// Computes HMAC-SM3 for the input message with the provided key.
///
/// # 返回 / Returns
///
/// 成功返回 `Some(digest)`，失败返回 `None`。\
/// Returns `Some(digest)` on success and `None` on failure.
///
/// # 示例 / Example
///
/// ```
/// use pkloong_kcapi::md::hmac_sm3;
///
/// let key = b"secret";
/// let message = b"hello world";
/// let digest = hmac_sm3(key, message).expect("HMAC-SM3 calculation failed");
/// assert_eq!(digest.as_slice(), [
///     0x7A, 0xF5, 0xF1, 0x79, 0x5D, 0x73, 0xF5, 0x09, 0xDC, 0x01, 0xD6, 0x03, 0x00, 0x3A, 0xC8,
///     0xF2, 0xD9, 0x26, 0x9B, 0x96, 0xD1, 0xFB, 0xE3, 0x69, 0x8B, 0xD8, 0xC8, 0xF3, 0xE6, 0x25,
///     0xD7, 0x99,
/// ].as_slice());
/// ```
pub fn hmac_sm3(key: &[u8], message: &[u8]) -> Option<[u8; SM3_DIGEST_SIZE]> {
    let mut digest = [0u8; SM3_DIGEST_SIZE];
    let result = unsafe {
        kcapi_md_hmac_sm3(
            key.as_ptr(),
            key.len() as u32,
            message.as_ptr(),
            message.len(),
            digest.as_mut_ptr(),
            digest.len(),
        )
    };

    if result == SM3_DIGEST_SIZE as isize {
        Some(digest)
    } else {
        None
    }
}

/// 计算输入消息的 SHA1 杂凑值。\
/// Computes the SHA1 digest for the input message.
///
/// # 返回 / Returns
///
/// 成功返回 `Some(digest)`，失败返回 `None`。\
/// Returns `Some(digest)` on success and `None` on failure.
///
/// # 示例 / Example
///
/// ```
/// use pkloong_kcapi::md::sha1;
///
/// let digest = sha1(b"hello world").expect("SHA1 calculation failed");
/// assert_eq!(digest.as_slice(), [
///     0x2A, 0xAE, 0x6C, 0x35, 0xC9, 0x4F, 0xCF, 0xB4, 0x15, 0xDB, 0xE9, 0x5F, 0x40, 0x8B, 0x9C,
///     0xE9, 0x1E, 0xE8, 0x46, 0xED,
/// ].as_slice());
/// ```
pub fn sha1(message: &[u8]) -> Option<[u8; SHA1_DIGEST_SIZE]> {
    let mut digest = [0u8; SHA1_DIGEST_SIZE];
    let result = unsafe {
        kcapi_md_sha1(
            message.as_ptr(),
            message.len(),
            digest.as_mut_ptr(),
            digest.len(),
        )
    };

    if result == SHA1_DIGEST_SIZE as isize {
        Some(digest)
    } else {
        None
    }
}

/// 使用给定密钥计算输入消息的 HMAC-SHA1。\
/// Computes HMAC-SHA1 for the input message with the provided key.
///
/// # 返回 / Returns
///
/// 成功返回 `Some(digest)`，失败返回 `None`。\
/// Returns `Some(digest)` on success and `None` on failure.
///
/// # 示例 / Example
///
/// ```
/// use pkloong_kcapi::md::hmac_sha1;
///
/// let digest = hmac_sha1(b"secret", b"hello world")
///     .expect("HMAC-SHA1 calculation failed");
/// assert_eq!(digest.as_slice(), [
///     0x03, 0x37, 0x6E, 0xE7, 0xAD, 0x7B, 0xBF, 0xCE, 0xEE, 0x98, 0x66, 0x04, 0x39, 0xA4, 0xD8,
///     0xB1, 0x25, 0x12, 0x2A, 0x5A,
/// ].as_slice());
/// ```
pub fn hmac_sha1(key: &[u8], message: &[u8]) -> Option<[u8; SHA1_DIGEST_SIZE]> {
    let mut digest = [0u8; SHA1_DIGEST_SIZE];
    let result = unsafe {
        kcapi_md_hmac_sha1(
            key.as_ptr(),
            key.len() as u32,
            message.as_ptr(),
            message.len(),
            digest.as_mut_ptr(),
            digest.len(),
        )
    };

    if result == SHA1_DIGEST_SIZE as isize {
        Some(digest)
    } else {
        None
    }
}

/// 计算输入消息的 SHA224 杂凑值。\
/// Computes the SHA224 digest for the input message.
///
/// # 返回 / Returns
///
/// 成功返回 `Some(digest)`，失败返回 `None`。\
/// Returns `Some(digest)` on success and `None` on failure.
///
/// # 示例 / Example
///
/// ```
/// use pkloong_kcapi::md::sha224;
///
/// let digest = sha224(b"hello world").expect("SHA224 calculation failed");
/// assert_eq!(digest.as_slice(), [
///     0x2F, 0x05, 0x47, 0x7F, 0xC2, 0x4B, 0xB4, 0xFA, 0xEF, 0xD8, 0x65, 0x17, 0x15, 0x6D, 0xAF,
///     0xDE, 0xCE, 0xC4, 0x5B, 0x8A, 0xD3, 0xCF, 0x25, 0x22, 0xA5, 0x63, 0x58, 0x2B,
/// ].as_slice());
/// ```
pub fn sha224(message: &[u8]) -> Option<[u8; SHA224_DIGEST_SIZE]> {
    let mut digest = [0u8; SHA224_DIGEST_SIZE];
    let result = unsafe {
        kcapi_md_sha224(
            message.as_ptr(),
            message.len(),
            digest.as_mut_ptr(),
            digest.len(),
        )
    };

    if result == SHA224_DIGEST_SIZE as isize {
        Some(digest)
    } else {
        None
    }
}

/// 使用给定密钥计算输入消息的 HMAC-SHA224。\
/// Computes HMAC-SHA224 for the input message with the provided key.
///
/// # 返回 / Returns
///
/// 成功返回 `Some(digest)`，失败返回 `None`。\
/// Returns `Some(digest)` on success and `None` on failure.
///
/// # 示例 / Example
///
/// ```
/// use pkloong_kcapi::md::hmac_sha224;
///
/// let digest = hmac_sha224(b"secret", b"hello world").expect("HMAC-SHA224 calculation failed");
/// assert_eq!(digest.as_slice(), [
///     0xDD, 0xD3, 0x26, 0xE7, 0x7D, 0xF7, 0xF2, 0x64, 0x5C, 0xD4, 0xC8, 0x97, 0x86, 0x47, 0x77,
///     0x52, 0xFD, 0x7C, 0x7D, 0xAC, 0xB5, 0xFA, 0x02, 0x00, 0xBD, 0xF6, 0x91, 0x0F,
/// ].as_slice());
/// ```
pub fn hmac_sha224(key: &[u8], message: &[u8]) -> Option<[u8; SHA224_DIGEST_SIZE]> {
    let mut digest = [0u8; SHA224_DIGEST_SIZE];
    let result = unsafe {
        kcapi_md_hmac_sha224(
            key.as_ptr(),
            key.len() as u32,
            message.as_ptr(),
            message.len(),
            digest.as_mut_ptr(),
            digest.len(),
        )
    };

    if result == SHA224_DIGEST_SIZE as isize {
        Some(digest)
    } else {
        None
    }
}

/// 计算输入消息的 SHA256 杂凑值。\
/// Computes the SHA256 digest for the input message.
///
/// # 返回 / Returns
///
/// 成功返回 `Some(digest)`，失败返回 `None`。\
/// Returns `Some(digest)` on success and `None` on failure.
///
/// # 示例 / Example
///
/// ```
/// use pkloong_kcapi::md::sha256;
///
/// let digest = sha256(b"hello world").expect("SHA256 calculation failed");
/// assert_eq!(digest.as_slice(), [
///     0xB9, 0x4D, 0x27, 0xB9, 0x93, 0x4D, 0x3E, 0x08, 0xA5, 0x2E, 0x52, 0xD7, 0xDA, 0x7D, 0xAB,
///     0xFA, 0xC4, 0x84, 0xEF, 0xE3, 0x7A, 0x53, 0x80, 0xEE, 0x90, 0x88, 0xF7, 0xAC, 0xE2, 0xEF,
///     0xCD, 0xE9,
/// ].as_slice());
/// ```
pub fn sha256(message: &[u8]) -> Option<[u8; SHA256_DIGEST_SIZE]> {
    let mut digest = [0u8; SHA256_DIGEST_SIZE];
    let result = unsafe {
        kcapi_md_sha256(
            message.as_ptr(),
            message.len(),
            digest.as_mut_ptr(),
            digest.len(),
        )
    };

    if result == SHA256_DIGEST_SIZE as isize {
        Some(digest)
    } else {
        None
    }
}

/// 使用给定密钥计算输入消息的 HMAC-SHA256。\
/// Computes HMAC-SHA256 for the input message with the provided key.
///
/// # 返回 / Returns
///
/// 成功返回 `Some(digest)`，失败返回 `None`。\
/// Returns `Some(digest)` on success and `None` on failure.
///
/// # 示例 / Example
///
/// ```
/// use pkloong_kcapi::md::hmac_sha256;
///
/// let digest = hmac_sha256(b"secret", b"hello world").expect("HMAC-SHA256 calculation failed");
/// assert_eq!(digest.as_slice(), [
///     0x73, 0x4C, 0xC6, 0x2F, 0x32, 0x84, 0x15, 0x68, 0xF4, 0x57, 0x15, 0xAE, 0xB9, 0xF4, 0xD7,
///     0x89, 0x13, 0x24, 0xE6, 0xD9, 0x48, 0xE4, 0xC6, 0xC6, 0x0C, 0x06, 0x21, 0xCD, 0xAC, 0x48,
///     0x62, 0x3A,
/// ].as_slice());
/// ```
pub fn hmac_sha256(key: &[u8], message: &[u8]) -> Option<[u8; SHA256_DIGEST_SIZE]> {
    let mut digest = [0u8; SHA256_DIGEST_SIZE];
    let result = unsafe {
        kcapi_md_hmac_sha256(
            key.as_ptr(),
            key.len() as u32,
            message.as_ptr(),
            message.len(),
            digest.as_mut_ptr(),
            digest.len(),
        )
    };

    if result == SHA256_DIGEST_SIZE as isize {
        Some(digest)
    } else {
        None
    }
}

/// 计算输入消息的 SHA384 杂凑值。\
/// Computes the SHA384 digest for the input message.
///
/// # 返回 / Returns
///
/// 成功返回 `Some(digest)`，失败返回 `None`。\
/// Returns `Some(digest)` on success and `None` on failure.
///
/// # 示例 / Example
///
/// ```
/// use pkloong_kcapi::md::sha384;
///
/// let digest = sha384(b"hello world").expect("SHA384 calculation failed");
/// assert_eq!(digest.as_slice(), [
///     0xFD, 0xBD, 0x8E, 0x75, 0xA6, 0x7F, 0x29, 0xF7, 0x01, 0xA4, 0xE0, 0x40, 0x38, 0x5E, 0x2E,
///     0x23, 0x98, 0x63, 0x03, 0xEA, 0x10, 0x23, 0x92, 0x11, 0xAF, 0x90, 0x7F, 0xCB, 0xB8, 0x35,
///     0x78, 0xB3, 0xE4, 0x17, 0xCB, 0x71, 0xCE, 0x64, 0x6E, 0xFD, 0x08, 0x19, 0xDD, 0x8C, 0x08,
///     0x8D, 0xE1, 0xBD,
/// ].as_slice());
/// ```
pub fn sha384(message: &[u8]) -> Option<[u8; SHA384_DIGEST_SIZE]> {
    let mut digest = [0u8; SHA384_DIGEST_SIZE];
    let result = unsafe {
        kcapi_md_sha384(
            message.as_ptr(),
            message.len(),
            digest.as_mut_ptr(),
            digest.len(),
        )
    };

    if result == SHA384_DIGEST_SIZE as isize {
        Some(digest)
    } else {
        None
    }
}

/// 使用给定密钥计算输入消息的 HMAC-SHA384。\
/// Computes HMAC-SHA384 for the input message with the provided key.
///
/// # 返回 / Returns
///
/// 成功返回 `Some(digest)`，失败返回 `None`。\
/// Returns `Some(digest)` on success and `None` on failure.
///
/// # 示例 / Example
///
/// ```
/// use pkloong_kcapi::md::hmac_sha384;
///
/// let digest = hmac_sha384(b"secret", b"hello world")
///     .expect("HMAC-SHA384 calculation failed");
/// assert_eq!(digest.as_slice(), [
///     0x2D, 0xA3, 0xBB, 0x17, 0x7B, 0x92, 0xAA, 0xE9, 0x8C, 0x3A, 0xB2, 0x27, 0x27, 0xD7, 0xF6,
///     0x0C, 0x90, 0x5B, 0xE1, 0xBA, 0xFF, 0x71, 0xFB, 0x4B, 0x00, 0xA6, 0xE4, 0x10, 0x92, 0x3E,
///     0x65, 0x58, 0x37, 0x65, 0x90, 0xC1, 0xFA, 0xF9, 0x22, 0xFF, 0x51, 0xEC, 0x49, 0xBE, 0x77,
///     0x40, 0x9A, 0xC6,
/// ].as_slice());
/// ```
pub fn hmac_sha384(key: &[u8], message: &[u8]) -> Option<[u8; SHA384_DIGEST_SIZE]> {
    let mut digest = [0u8; SHA384_DIGEST_SIZE];
    let result = unsafe {
        kcapi_md_hmac_sha384(
            key.as_ptr(),
            key.len() as u32,
            message.as_ptr(),
            message.len(),
            digest.as_mut_ptr(),
            digest.len(),
        )
    };

    if result == SHA384_DIGEST_SIZE as isize {
        Some(digest)
    } else {
        None
    }
}

/// 计算输入消息的 SHA512 杂凑值。\
/// Computes the SHA512 digest for the input message.
///
/// # 返回 / Returns
///
/// 成功返回 `Some(digest)`，失败返回 `None`。\
/// Returns `Some(digest)` on success and `None` on failure.
///
/// # 示例 / Example
///
/// ```
/// use pkloong_kcapi::md::sha512;
///
/// let digest = sha512(b"hello world").expect("SHA512 calculation failed");
/// assert_eq!(digest.as_slice(), [
///     0x30, 0x9E, 0xCC, 0x48, 0x9C, 0x12, 0xD6, 0xEB, 0x4C, 0xC4, 0x0F, 0x50, 0xC9, 0x02, 0xF2,
///     0xB4, 0xD0, 0xED, 0x77, 0xEE, 0x51, 0x1A, 0x7C, 0x7A, 0x9B, 0xCD, 0x3C, 0xA8, 0x6D, 0x4C,
///     0xD8, 0x6F, 0x98, 0x9D, 0xD3, 0x5B, 0xC5, 0xFF, 0x49, 0x96, 0x70, 0xDA, 0x34, 0x25, 0x5B,
///     0x45, 0xB0, 0xCF, 0xD8, 0x30, 0xE8, 0x1F, 0x60, 0x5D, 0xCF, 0x7D, 0xC5, 0x54, 0x2E, 0x93,
///     0xAE, 0x9C, 0xD7, 0x6F,
/// ].as_slice());
/// ```
pub fn sha512(message: &[u8]) -> Option<[u8; SHA512_DIGEST_SIZE]> {
    let mut digest = [0u8; SHA512_DIGEST_SIZE];
    let result = unsafe {
        kcapi_md_sha512(
            message.as_ptr(),
            message.len(),
            digest.as_mut_ptr(),
            digest.len(),
        )
    };

    if result == SHA512_DIGEST_SIZE as isize {
        Some(digest)
    } else {
        None
    }
}

/// 使用给定密钥计算输入消息的 HMAC-SHA512。\
/// Computes HMAC-SHA512 for the input message with the provided key.
///
/// # 返回 / Returns
///
/// 成功返回 `Some(digest)`，失败返回 `None`。\
/// Returns `Some(digest)` on success and `None` on failure.
///
/// # 示例 / Example
///
/// ```
/// use pkloong_kcapi::md::hmac_sha512;
///
/// let digest = hmac_sha512(b"secret", b"hello world").expect("HMAC-SHA512 calculation failed");
/// assert_eq!(digest.as_slice(), [
///     0x6D, 0x32, 0x23, 0x9B, 0x01, 0xDD, 0x17, 0x50, 0x55, 0x72, 0x11, 0x62, 0x93, 0x13, 0xD9,
///     0x5E, 0x4F, 0x4F, 0xCB, 0x8E, 0xE5, 0x17, 0xE4, 0x43, 0x99, 0x0A, 0xC1, 0xAF, 0xC7, 0x56,
///     0x2B, 0xFD, 0x74, 0xFF, 0xA6, 0x11, 0x83, 0x87, 0xEF, 0xD9, 0xE1, 0x68, 0xFF, 0x86, 0xD1,
///     0xDA, 0x5C, 0xEF, 0x4A, 0x55, 0xED, 0xC6, 0x3C, 0xC4, 0xBA, 0x28, 0x9C, 0x4C, 0x3A, 0x8B,
///     0x4F, 0x7B, 0xDF, 0xC2,
/// ].as_slice());
/// ```
pub fn hmac_sha512(key: &[u8], message: &[u8]) -> Option<[u8; SHA512_DIGEST_SIZE]> {
    let mut digest = [0u8; SHA512_DIGEST_SIZE];
    let result = unsafe {
        kcapi_md_hmac_sha512(
            key.as_ptr(),
            key.len() as u32,
            message.as_ptr(),
            message.len(),
            digest.as_mut_ptr(),
            digest.len(),
        )
    };

    if result == SHA512_DIGEST_SIZE as isize {
        Some(digest)
    } else {
        None
    }
}

/// 计算输入消息的 SHA3-224 杂凑值。\
/// Computes the SHA3-224 digest for the input message.
///
/// # 返回 / Returns
///
/// 成功返回 `Some(digest)`，失败返回 `None`。\
/// Returns `Some(digest)` on success and `None` on failure.
///
/// # 示例 / Example
///
/// ```
/// use pkloong_kcapi::md::sha3_224;
///
/// let digest = sha3_224(b"hello world").expect("SHA3-224 calculation failed");
/// assert_eq!(digest.as_slice(), [
///     0xDF, 0xB7, 0xF1, 0x8C, 0x77, 0xE9, 0x28, 0xBB, 0x56, 0xFA, 0xEB, 0x2D, 0xA2, 0x72, 0x91,
///     0xBD, 0x79, 0x0B, 0xC1, 0x04, 0x5C, 0xDE, 0x45, 0xF3, 0x21, 0x0B, 0xB6, 0xC5,
/// ].as_slice());
/// ```
pub fn sha3_224(message: &[u8]) -> Option<[u8; SHA3_224_DIGEST_SIZE]> {
    let mut digest = [0u8; SHA3_224_DIGEST_SIZE];
    let result = unsafe {
        kcapi_md_sha3_224(
            message.as_ptr(),
            message.len(),
            digest.as_mut_ptr(),
            digest.len(),
        )
    };

    if result == SHA3_224_DIGEST_SIZE as isize {
        Some(digest)
    } else {
        None
    }
}

/// 计算输入消息的 SHA3-256 杂凑值。\
/// Computes the SHA3-256 digest for the input message.
///
/// # 返回 / Returns
///
/// 成功返回 `Some(digest)`，失败返回 `None`。\
/// Returns `Some(digest)` on success and `None` on failure.
///
/// # 示例 / Example
///
/// ```
/// use pkloong_kcapi::md::sha3_256;
///
/// let digest = sha3_256(b"hello world").expect("SHA3-256 calculation failed");
/// assert_eq!(digest.as_slice(), [
///     0x64, 0x4B, 0xCC, 0x7E, 0x56, 0x43, 0x73, 0x04, 0x09, 0x99, 0xAA, 0xC8, 0x9E, 0x76, 0x22,
///     0xF3, 0xCA, 0x71, 0xFB, 0xA1, 0xD9, 0x72, 0xFD, 0x94, 0xA3, 0x1C, 0x3B, 0xFB, 0xF2, 0x4E,
///     0x39, 0x38,
/// ].as_slice());
/// ```
pub fn sha3_256(message: &[u8]) -> Option<[u8; SHA3_256_DIGEST_SIZE]> {
    let mut digest = [0u8; SHA3_256_DIGEST_SIZE];
    let result = unsafe {
        kcapi_md_sha3_256(
            message.as_ptr(),
            message.len(),
            digest.as_mut_ptr(),
            digest.len(),
        )
    };

    if result == SHA3_256_DIGEST_SIZE as isize {
        Some(digest)
    } else {
        None
    }
}

/// 计算输入消息的 SHA3-384 杂凑值。\
/// Computes the SHA3-384 digest for the input message.
///
/// # 返回 / Returns
///
/// 成功返回 `Some(digest)`，失败返回 `None`。\
/// Returns `Some(digest)` on success and `None` on failure.
///
/// # 示例 / Example
///
/// ```
/// use pkloong_kcapi::md::sha3_384;
///
/// let digest = sha3_384(b"hello world").expect("SHA3-384 calculation failed");
/// assert_eq!(digest.as_slice(), [
///     0x83, 0xBF, 0xF2, 0x8D, 0xDE, 0x1B, 0x1B, 0xF5, 0x81, 0x00, 0x71, 0xC6, 0x64, 0x3C, 0x08,
///     0xE5, 0xB0, 0x5B, 0xDB, 0x83, 0x6E, 0xFF, 0xD7, 0x0B, 0x40, 0x3E, 0xA8, 0xEA, 0x0A, 0x63,
///     0x4D, 0xC4, 0x99, 0x7E, 0xB1, 0x05, 0x3A, 0xA3, 0x59, 0x3F, 0x59, 0x0F, 0x9C, 0x63, 0x63,
///     0x0D, 0xD9, 0x0B,
/// ].as_slice());
/// ```
pub fn sha3_384(message: &[u8]) -> Option<[u8; SHA3_384_DIGEST_SIZE]> {
    let mut digest = [0u8; SHA3_384_DIGEST_SIZE];
    let result = unsafe {
        kcapi_md_sha3_384(
            message.as_ptr(),
            message.len(),
            digest.as_mut_ptr(),
            digest.len(),
        )
    };

    if result == SHA3_384_DIGEST_SIZE as isize {
        Some(digest)
    } else {
        None
    }
}

/// 计算输入消息的 SHA3-512 杂凑值。\
/// Computes the SHA3-512 digest for the input message.
///
/// # 返回 / Returns
///
/// 成功返回 `Some(digest)`，失败返回 `None`。\
/// Returns `Some(digest)` on success and `None` on failure.
///
/// # 示例 / Example
///
/// ```
/// use pkloong_kcapi::md::sha3_512;
///
/// let digest = sha3_512(b"hello world").expect("SHA3-512 calculation failed");
/// assert_eq!(digest.as_slice(), [
///     0x84, 0x00, 0x06, 0x65, 0x3E, 0x9A, 0xC9, 0xE9, 0x51, 0x17, 0xA1, 0x5C, 0x91, 0x5C, 0xAA,
///     0xB8, 0x16, 0x62, 0x91, 0x8E, 0x92, 0x5D, 0xE9, 0xE0, 0x04, 0xF7, 0x74, 0xFF, 0x82, 0xD7,
///     0x07, 0x9A, 0x40, 0xD4, 0xD2, 0x7B, 0x1B, 0x37, 0x26, 0x57, 0xC6, 0x1D, 0x46, 0xD4, 0x70,
///     0x30, 0x4C, 0x88, 0xC7, 0x88, 0xB3, 0xA4, 0x52, 0x7A, 0xD0, 0x74, 0xD1, 0xDC, 0xCB, 0xEE,
///     0x5D, 0xBA, 0xA9, 0x9A,
/// ].as_slice());
/// ```
pub fn sha3_512(message: &[u8]) -> Option<[u8; SHA3_512_DIGEST_SIZE]> {
    let mut digest = [0u8; SHA3_512_DIGEST_SIZE];
    let result = unsafe {
        kcapi_md_sha3_512(
            message.as_ptr(),
            message.len(),
            digest.as_mut_ptr(),
            digest.len(),
        )
    };

    if result == SHA3_512_DIGEST_SIZE as isize {
        Some(digest)
    } else {
        None
    }
}
