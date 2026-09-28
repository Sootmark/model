//! The Sootmark record and event model.
//!
//! Parsers are independent libraries with their own APIs. The workbench owns
//! this model and one small [`adapter::Adapter`] per parser that maps the
//! parser's native output into [`Record`]s. Everything downstream (storage,
//! the timeline, hunts, findings) works on records and the [`Event`]s
//! projected from them.
//!
//! - A [`Record`] is one artifact entry with full fidelity: where it came
//!   from ([`EvidenceId`] + [`Locator`]), which parser read it, its native
//!   fields, the shared [`Facets`] hunts rely on, and its timestamps.
//! - An [`Event`] is one timestamp of a record: the unit of the timeline.
//! - [`RecordId`]s are derived from physical location, never from parse
//!   order, so marks and findings survive re-parsing with newer parsers.

pub mod adapter;
mod event;
mod facets;
mod id;
mod locator;
mod namespace;
mod record;
mod time;
mod value;

pub use event::{Event, EventId};
pub use facets::Facets;
pub use id::{EvidenceId, RecordId};
pub use locator::Locator;
pub use namespace::Namespace;
pub use record::{Flags, ParserInfo, Record};
pub use time::{Precision, RecordTime, Semantic, TimeKind, Ts};
pub use value::{Fields, Value};
