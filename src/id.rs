/// Stable 64 bit FNV-1a hash.
///
/// This is used instead of the standard library hasher because its output is
/// guaranteed to stay the same across Rust versions.
pub fn fnv1a(text: &str) -> u64 {
    let mut hash = 0xcbf2_9ce4_8422_2325_u64;
    for byte in text.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    hash
}

/// Build a stable identifier with a readable prefix.
pub fn stable_id(prefix: &str, text: &str) -> String {
    format!("{prefix}-{:012x}", fnv1a(text) & 0x0000_ffff_ffff_ffff)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_is_stable() {
        assert_eq!(fnv1a("hola"), fnv1a("hola"));
        assert_ne!(fnv1a("hola"), fnv1a("adios"));
    }

    #[test]
    fn stable_id_has_a_readable_prefix() {
        let id = stable_id("vocab", "hola");
        assert!(id.starts_with("vocab-"));
        assert_eq!(id.len(), "vocab-".len() + 12);
        assert_eq!(id, stable_id("vocab", "hola"));
    }
}
