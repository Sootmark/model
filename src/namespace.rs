use core::fmt;

/// The kind of artifact a record describes, e.g. `windows.evtx`.
///
/// Lowercase dot-separated segments of `[a-z0-9_]`. Namespaces are compile-time
/// constants owned by adapters.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Namespace(&'static str);

impl Namespace {
    /// Create a namespace.
    ///
    /// # Panics
    /// If `name` isn't lowercase dot-separated segments of `[a-z0-9_]`. Called
    /// in a `const` context, a bad name fails the build.
    #[must_use]
    pub const fn new(name: &'static str) -> Self {
        assert!(is_valid(name.as_bytes()), "invalid namespace");
        Self(name)
    }

    /// The namespace string.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        self.0
    }
}

impl fmt::Display for Namespace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0)
    }
}

const fn is_valid(bytes: &[u8]) -> bool {
    if bytes.is_empty() || bytes[0] == b'.' || bytes[bytes.len() - 1] == b'.' {
        return false;
    }
    let mut i = 0;
    while i < bytes.len() {
        let b = bytes[i];
        let allowed = b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'_' || b == b'.';
        let double_dot = b == b'.' && i > 0 && bytes[i - 1] == b'.';
        if !allowed || double_dot {
            return false;
        }
        i += 1;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_well_formed_names() {
        assert_eq!(Namespace::new("windows.evtx").as_str(), "windows.evtx");
        assert_eq!(Namespace::new("m365.ual").to_string(), "m365.ual");
    }

    #[test]
    fn rejects_malformed_names() {
        for bad in [
            "",
            "Windows.evtx",
            "windows..evtx",
            ".evtx",
            "evtx.",
            "win evtx",
        ] {
            assert!(!is_valid(bad.as_bytes()), "{bad:?} should be rejected");
        }
    }
}
