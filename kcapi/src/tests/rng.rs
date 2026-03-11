#[test]
fn test_get_bytes_zero_length() {
    let bytes = crate::rng::get_bytes(0).expect("RNG zero-length request failed");
    assert!(bytes.is_empty());
}

#[test]
fn test_get_bytes_length_16() {
    let bytes = crate::rng::get_bytes(16).expect("RNG 16-byte request failed");
    assert_eq!(bytes.len(), 16);
}

#[test]
fn test_get_bytes_multiple_calls() {
    for len in [1_usize, 8, 32, 256] {
        let bytes = crate::rng::get_bytes(len).expect("RNG request failed in loop");
        assert_eq!(bytes.len(), len);
    }
}
