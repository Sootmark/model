use core::fmt;
use sha2::{Digest, Sha256};

use crate::locator::encode_str;
use crate::{Locator, Namespace};

/// Identity of an evidence file: the SHA-256 of its content.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EvidenceId([u8; 32]);

impl EvidenceId {
    /// Wrap a SHA-256 digest computed during intake.
    #[must_use]
    pub const fn from_sha256(digest: [u8; 32]) -> Self {
        Self(digest)
    }

    /// Hash `content` (convenient for small inputs and tests; intake streams
    /// large files and uses [`EvidenceId::from_sha256`]).
    #[must_use]
    pub fn of_content(content: &[u8]) -> Self {
        Self(Sha256::digest(content).into())
    }

    /// The digest bytes.
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

impl fmt::Display for EvidenceId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write_hex(f, &self.0)
    }
}

/// Stable identity of a record, derived from where it physically lives.
///
/// The same evidence, namespace and locator always give the same id, whatever
/// the parser version or the order records were read in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RecordId([u8; 16]);

/// Separates record-id hashes from any other SHA-256 use.
const RECORD_ID_DOMAIN: &str = "sootmark.record-id.v1";

impl RecordId {
    /// Derive the id of the record at `locator` in `evidence`, for `namespace`.
    #[must_use]
    pub fn derive(evidence: &EvidenceId, namespace: Namespace, locator: &Locator) -> Self {
        let mut input = Vec::with_capacity(96);
        encode_str(RECORD_ID_DOMAIN, &mut input);
        input.extend_from_slice(evidence.as_bytes());
        encode_str(namespace.as_str(), &mut input);
        locator.encode(&mut input);
        let digest = Sha256::digest(&input);
        let mut id = [0u8; 16];
        id.copy_from_slice(&digest[..16]);
        Self(id)
    }

    /// The id bytes.
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; 16] {
        &self.0
    }
}

impl fmt::Display for RecordId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write_hex(f, &self.0)
    }
}

fn write_hex(f: &mut fmt::Formatter<'_>, bytes: &[u8]) -> fmt::Result {
    bytes.iter().try_for_each(|b| write!(f, "{b:02x}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EVTX: Namespace = Namespace::new("windows.evtx");
    const PREFETCH: Namespace = Namespace::new("windows.prefetch");

    fn evidence() -> EvidenceId {
        EvidenceId::of_content(b"evidence")
    }

    #[test]
    fn same_inputs_give_the_same_id() {
        let a = RecordId::derive(&evidence(), EVTX, &Locator::ByteOffset(4096));
        let b = RecordId::derive(&evidence(), EVTX, &Locator::ByteOffset(4096));
        assert_eq!(a, b);
    }

    #[test]
    fn every_input_changes_the_id() {
        let base = RecordId::derive(&evidence(), EVTX, &Locator::ByteOffset(4096));
        let other_evidence = EvidenceId::of_content(b"other");
        assert_ne!(
            base,
            RecordId::derive(&other_evidence, EVTX, &Locator::ByteOffset(4096))
        );
        assert_ne!(
            base,
            RecordId::derive(&evidence(), PREFETCH, &Locator::ByteOffset(4096))
        );
        assert_ne!(
            base,
            RecordId::derive(&evidence(), EVTX, &Locator::ByteOffset(4097))
        );
    }

    #[test]
    fn displays_as_lowercase_hex() {
        let id = RecordId::derive(&evidence(), EVTX, &Locator::Line(1)).to_string();
        assert_eq!(id.len(), 32);
        assert!(id
            .chars()
            .all(|c| c.is_ascii_hexdigit() && !c.is_ascii_uppercase()));
    }
}
