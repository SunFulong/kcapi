#[cfg(feature = "md")]
mod md;
#[cfg(feature = "rng")]
mod rng;

#[test]
fn test_version() {
    let version = crate::version::get_version();
    assert_eq!(version.package.to_string(), env!("CARGO_PKG_VERSION"));
    assert_eq!(version.library.to_string(), "1.5.0");
}
