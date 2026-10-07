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
use std::io::Read;

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

/// What an adapter is asked to parse as a stream: an [`Input`] without the
/// content, which comes from a reader.
#[derive(Debug, Clone, Copy)]
pub struct StreamInput<'a> {
    /// Identity of the evidence file (SHA-256 of its content).
    pub evidence: EvidenceId,
    /// File name or path inside the collection, for detection and messages.
    pub name: &'a str,
    /// The content's size in bytes.
    pub size: u64,
    /// When the file was last modified: as [`Input::modified`].
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

    /// Parse a file read from `content` rather than held in memory: for
    /// files too large to hold (a USN journal of many gigabytes). The same
    /// promises hold, and the records must be those [`Adapter::parse`]
    /// gives. `None` when this adapter needs the whole file (the default).
    fn parse_stream(
        &self,
        _input: &StreamInput<'_>,
        _content: &mut dyn Read,
        _sink: &mut dyn Sink,
    ) -> Option<Result<(), ParseError>> {
        None
    }

    /// Parse `input` with the SQLite write-ahead log found beside it (its
    /// `-wal` file). Adapters of SQLite formats override this to read the
    /// pair as SQLite would, and the log's older page versions too, where
    /// deleted rows often survive; the same promises hold, and with an
    /// empty log the records must be those [`Adapter::parse`] gives. The
    /// default ignores the log: callers hand it only to adapters of SQLite
    /// formats.
    ///
    /// # Errors
    /// As [`Adapter::parse`].
    fn parse_with_log(
        &self,
        input: &Input<'_>,
        log: &[u8],
        sink: &mut dyn Sink,
    ) -> Result<(), ParseError> {
        let _ = log;
        self.parse(input, sink)
    }

    /// The other files, beside the one named `name` in its folder, this
    /// adapter reads with it: the WMI repository's `INDEX.BTR` and
    /// `MAPPING` files for its `OBJECTS.DATA`. Callers look them up
    /// without case and hand those found to
    /// [`Adapter::parse_with_companions`]. None by default.
    fn companions(&self, name: &str) -> Vec<String> {
        let _ = name;
        Vec::new()
    }

    /// Parse `input` with the companion files found beside it (those
    /// [`Adapter::companions`] names; any may be missing). The same
    /// promises hold. The default ignores them.
    ///
    /// # Errors
    /// As [`Adapter::parse`].
    fn parse_with_companions(
        &self,
        input: &Input<'_>,
        companions: &[Companion<'_>],
        sink: &mut dyn Sink,
    ) -> Result<(), ParseError> {
        let _ = companions;
        self.parse(input, sink)
    }
}

/// A file read with another ([`Adapter::companions`]).
#[derive(Debug, Clone, Copy)]
pub struct Companion<'a> {
    /// Its name, as [`Adapter::companions`] gave it.
    pub name: &'a str,
    /// Its content.
    pub data: &'a [u8],
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

#[cfg(test)]
mod tests {
    use super::*;

    /// An adapter that records how many bytes it was given.
    struct Counting;

    impl Adapter for Counting {
        fn parser(&self) -> ParserInfo {
            ParserInfo {
                name: "counting",
                version: "1",
            }
        }

        fn namespaces(&self) -> &'static [Namespace] {
            &[]
        }

        fn probe(&self, _name: &str, _head: &[u8]) -> Confidence {
            Confidence::No
        }

        fn parse(&self, input: &Input<'_>, sink: &mut dyn Sink) -> Result<(), ParseError> {
            sink.skipped(Skipped {
                locator: Locator::ByteOffset(input.data.len() as u64),
                reason: "counted".to_owned(),
            });
            Ok(())
        }
    }

    #[test]
    fn companions_default_to_none_and_are_ignored() {
        assert!(Counting.companions("OBJECTS.DATA").is_empty());
        let input = Input {
            evidence: EvidenceId::of_content(b"abc"),
            name: "x",
            data: b"abc",
            modified: None,
        };
        let companion = Companion {
            name: "INDEX.BTR",
            data: b"defgh",
        };
        let mut sink = Collected::default();
        Counting
            .parse_with_companions(&input, &[companion], &mut sink)
            .unwrap();
        assert_eq!(sink.skipped[0].locator, Locator::ByteOffset(3));
    }
}
