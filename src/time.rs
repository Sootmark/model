pub use common::time::{Precision, Semantic, Ts};

/// What a timestamp means for the artifact. The controlled vocabulary the
/// timeline, hunts and corroboration rules reason about.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TimeKind {
    /// When the entry was written or logged (EVTX `TimeCreated`).
    Logged,
    /// A creation time (file, key, account).
    Created,
    /// A content modification time.
    Modified,
    /// A last-access time.
    Accessed,
    /// A metadata change time (NTFS "entry modified", the C in MACB).
    MetadataChanged,
    /// An execution time (Prefetch run times).
    Executed,
    /// The first time something was observed.
    FirstSeen,
    /// The last time something was observed.
    LastSeen,
    /// A deletion time.
    Deleted,
    /// Anything else; the field name says what.
    Other,
}

/// One timestamp of a record, with its meaning and native field name.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordTime {
    /// What the timestamp means.
    pub kind: TimeKind,
    /// The artifact's own name for the field, e.g. `TimeCreated`,
    /// `last_run[3]`, `si_modified`: shown as the timeline's "meaning" column.
    pub field: String,
    /// The converted value, including its precision and semantic.
    pub ts: Ts,
}

impl RecordTime {
    /// A record time.
    #[must_use]
    pub fn new(kind: TimeKind, field: impl Into<String>, ts: Ts) -> Self {
        Self {
            kind,
            field: field.into(),
            ts,
        }
    }
}
