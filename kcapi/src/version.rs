//! `version` 模块提供包版本与底层 `libkcapi` 版本的统一查询接口，返回结构化结果，便于日志输出、兼容性检查与调试定位。\
//! The `version` module provides a unified API to query both package version and underlying `libkcapi` version, returning structured data suitable for logging, compatibility checks, and debugging.
//!
//! `get_version()` 会组合两个来源：包版本来自 `CARGO_PKG_VERSION`，库版本来自 `pkloong_kcapi_sys::kcapi_version()`；解析后统一映射为 `VersionInfo`。\
//! `get_version()` combines two sources: package version from `CARGO_PKG_VERSION`, and library version from `pkloong_kcapi_sys::kcapi_version()`, then normalizes both into `VersionInfo`.
//!
//! 为避免重复解析，本模块使用 `OnceLock` 缓存版本信息；同一进程内后续调用只读缓存值。\
//! To avoid repeated parsing, this module caches version information with `OnceLock`; subsequent calls in the same process read from cache.
//!
//! # 示例 / Example
//!
//! ```rust
//! use pkloong_kcapi::version::get_version;
//!
//! let version = get_version();
//! println!("package: {}", version.package);
//! println!("library: {}", version.library);
//! ```

use std::fmt::Display;
use std::sync::OnceLock;

use pkloong_kcapi_sys::kcapi_version;

static PACKAGE_VERSION_INFO: OnceLock<VersionInfo> = OnceLock::new();
static LIBRARY_VERSION_INFO: OnceLock<VersionInfo> = OnceLock::new();

/// 结构化版本信息：包含主版本号、次版本号、修订号和可选标识。\
/// Structured version information including major, minor, patch, and an optional identifier.
#[derive(Clone, Debug)]
pub struct VersionInfo {
    /// 主版本号。\
    /// Major version number.
    pub major: u16,
    /// 次版本号。\
    /// Minor version number.
    pub minor: u16,
    /// 修订号。\
    /// Patch version number.
    pub patch: u16,
    /// 可选标识，可用于预发布标签或扩展版本信息。\
    /// Optional identifier used for pre-release tags or extended version metadata.
    pub ident: Option<String>,
}

impl Display for VersionInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(ident_str) = self.ident.as_ref() {
            write!(
                f,
                "{}.{}.{}-{}",
                self.major, self.minor, self.patch, ident_str
            )
        } else {
            write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
        }
    }
}

/// 组合后的版本信息：同时包含包版本与底层库版本。\
/// Combined version data containing both package version and library version.
pub struct KcapiVersion {
    /// 包版本信息。\
    /// Package version information.
    pub package: VersionInfo,
    /// 底层库版本信息。\
    /// Underlying library version information.
    pub library: VersionInfo,
}

/// 获取包版本与底层库版本，并返回统一结构。\
/// Retrieves both package and library versions and returns them in a unified structure.
pub fn get_version() -> KcapiVersion {
    let package_version_info = PACKAGE_VERSION_INFO.get_or_init(|| {
        const PACKAGE_VERSION: &str = env!("CARGO_PKG_VERSION");
        let mut tokens = PACKAGE_VERSION
            .splitn(4usize, ['.', '+', '-', '_'])
            .map(str::trim_ascii);
        VersionInfo {
            major: tokens
                .next()
                .and_then(|str| str.parse::<u16>().ok())
                .unwrap_or_default(),
            minor: tokens
                .next()
                .and_then(|str| str.parse::<u16>().ok())
                .unwrap_or_default(),
            patch: tokens
                .next()
                .and_then(|str| str.parse::<u16>().ok())
                .unwrap_or_default(),
            ident: tokens
                .next()
                .filter(|str| !str.is_empty())
                .map(str::to_owned),
        }
    });

    let library_version_info = LIBRARY_VERSION_INFO.get_or_init(|| {
        let version = unsafe { kcapi_version() } as u64;
        VersionInfo {
            major: ((version / 1000000) % 100) as u16,
            minor: ((version / 10000) % 100) as u16,
            patch: ((version / 100) % 100) as u16,
            ident: if version % 100 == 0 {
                None
            } else {
                Some((version % 100).to_string())
            },
        }
    });

    KcapiVersion {
        package: package_version_info.clone(),
        library: library_version_info.clone(),
    }
}
