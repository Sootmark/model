//! The contract between the workbench and a parser.
//!
//! Each parser is an independent library with its own API. An adapter wraps
//! one parser and maps its output into [`Record`]s. Every adapter must keep
//! these promises (the `conformance` crate checks them):
//!
//! 1. **Deterministic**: the same input always yields the same records, in
//!    the same order, with the same timestamps.
//! 2. **Never silent**: every entry that can't be turned into a record is
//!    reported to [`Sink::skipped`] with its location and a reason.
//! 3. **Never panics**: truncated, corrupt or hostile input yields records,
//!    skipped entries or a [`ParseError`], never a panic.
//! 4. **Honest provenance**: every record carries this adapter's
//!    [`ParserInfo`] and one of its declared namespaces.
//! 5. **Locators are physical**: record locators describe where the entry
//!    lives, not the order it was read in.

use core::fmt;

use crate::{EvidenceId, Locator, Namespace, ParserInfo, Record};

/// What an adapter is asked to parse.
#[derive(Debug, Clone, Copy)]
pub struct Input<'a> {
    /// Identity of the evidence file (SHA-256 of its content).
    pub evidence: EvidenceId,
    /// File name or path inside the collection, for detection and messages.
    pub name: &'a str,
    /// The file content.
    pub data: &'a [u8],
    /// When the file was last modified, as the collection or image recorded
    /// it; `None` when it didn't. Formats whose times lack a year (classic
    /// syslog) date their entries from it.
    pub modified: Option<common::time::Ts>,
}

/// How confident an adapter is that it can parse a file.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Confidence {
    /// Not this adapter's format.
    No,
    /// Plausible (e.g. the name matches but the header wasn't checked).
    Maybe,
    /// The header or magic bytes match.
    Certain,
}

/// An entry that couldn't become a record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Skipped {
    /// Where the entry lives.
    pub locator: Locator,
    /// Why it was skipped.
    pub reason: String,
}

/// Receives an adapter's output as it is produced.
pub trait Sink {
    /// Accept a record.
    fn record(&mut self, record: Record);
    /// Note an entry that couldn't become a record.
    fn skipped(&mut self, skipped: Skipped);
}

/// A failure that stops parsing the whole file (e.g. not the right format).
/// Records already sent to the sink stay valid.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError {
    /// Byte offset of the problem, when known.
    pub offset: Option<u64>,
    /// What went wrong.
    pub message: String,
}

impl ParseError {
    /// An error at a known offset.
    #[must_use]
    pub fn at(offset: u64, message: impl Into<String>) -> Self {
        Self {
            offset: Some(offset),
            message: message.into(),
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.offset {
            Some(offset) => write!(f, "{} (at offset {offset})", self.message),
            None => f.write_str(&self.message),
        }
    }
}

impl std::error::Error for ParseError {}

/// Maps one parser's output into Sootmark records. See the [module docs](self)
/// for the promises every implementation must keep.
pub trait Adapter {
    /// The wrapped parser's name and version, stamped on every record.
    fn parser(&self) -> ParserInfo;

    /// Every namespace this adapter can emit.
    fn namespaces(&self) -> &'static [Namespace];

    /// Whether this adapter can parse a file, from its name and first bytes.
    fn probe(&self, name: &str, head: &[u8]) -> Confidence;

    /// Parse `input`, sending records and skipped entries to `sink`.
    ///
    /// # Errors
    /// A [`ParseError`] when the file as a whole can't be parsed.
    fn parse(&self, input: &Input<'_>, sink: &mut dyn Sink) -> Result<(), ParseError>;
}

/// A [`Sink`] that keeps everything in memory. For tests and small files.
#[derive(Debug, Default)]
pub struct Collected {
    /// Records, in the order they were produced.
    pub records: Vec<Record>,
    /// Skipped entries, in the order they were reported.
    pub skipped: Vec<Skipped>,
}

impl Sink for Collected {
    fn record(&mut self, record: Record) {
        self.records.push(record);
    }

    fn skipped(&mut self, skipped: Skipped) {
        self.skipped.push(skipped);
    }
}
