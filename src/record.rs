use crate::{EvidenceId, Facets, Fields, Locator, Namespace, RecordId, RecordTime};

/// The parser (and version) that produced a record, for provenance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ParserInfo {
    /// Parser name, e.g. `evtx`.
    pub name: &'static str,
    /// Parser version, e.g. `0.3.1`.
    pub version: &'static str,
}

/// Conditions that change how much weight a record can carry.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Flags {
    /// Recovered from slack or unallocated space inside the file.
    pub recovered: bool,
    /// Marked deleted by the artifact itself.
    pub deleted: bool,
    /// Parsed despite structural damage; some fields may be missing.
    pub corrupted: bool,
}

/// One artifact entry, with full fidelity and provenance.
///
/// Identity fields (evidence, namespace, locator, parser) are fixed at
/// construction and the [`RecordId`] is derived from them, so an id can never
/// disagree with the location it names.
#[derive(Debug, Clone, PartialEq)]
pub struct Record {
    id: RecordId,
    evidence: EvidenceId,
    namespace: Namespace,
    locator: Locator,
    parser: ParserInfo,
    /// Conditions affecting reliability.
    pub flags: Flags,
    /// Timestamps, in the order the adapter emits them (must be deterministic).
    pub times: Vec<RecordTime>,
    /// Cross-artifact fields.
    pub facets: Facets,
    /// The artifact's native fields.
    pub fields: Fields,
    /// One human-readable line for the timeline grid.
    pub summary: String,
}

impl Record {
    /// A new record with empty content. Its id is derived from `evidence`,
    /// `namespace` and `locator`.
    #[must_use]
    pub fn new(
        evidence: EvidenceId,
        namespace: Namespace,
        locator: Locator,
        parser: ParserInfo,
    ) -> Self {
        Self {
            id: RecordId::derive(&evidence, namespace, &locator),
            evidence,
            namespace,
            locator,
            parser,
            flags: Flags::default(),
            times: Vec::new(),
            facets: Facets::default(),
            fields: Fields::new(),
            summary: String::new(),
        }
    }

    /// The record's stable id.
    #[must_use]
    pub const fn id(&self) -> RecordId {
        self.id
    }

    /// The evidence file the record came from.
    #[must_use]
    pub const fn evidence(&self) -> EvidenceId {
        self.evidence
    }

    /// The kind of artifact.
    #[must_use]
    pub const fn namespace(&self) -> Namespace {
        self.namespace
    }

    /// Where the record lives in the evidence file.
    #[must_use]
    pub const fn locator(&self) -> &Locator {
        &self.locator
    }

    /// The parser that read it.
    #[must_use]
    pub const fn parser(&self) -> ParserInfo {
        self.parser
    }
}
