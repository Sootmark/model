use core::fmt;

use crate::{Record, RecordId, RecordTime};

/// Identity of an event: its record and the timestamp's position in it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EventId {
    /// The record the event belongs to.
    pub record: RecordId,
    /// Index of the timestamp in [`Record::times`].
    pub ordinal: u16,
}

impl fmt::Display for EventId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.record, self.ordinal)
    }
}

/// One timestamp of one record: the unit of the timeline.
#[derive(Debug, Clone, PartialEq)]
pub struct Event<'r> {
    /// The event's identity.
    pub id: EventId,
    /// The timestamp.
    pub time: &'r RecordTime,
    /// The record it belongs to.
    pub record: &'r Record,
}

impl Record {
    /// The record's events, one per timestamp, in record order.
    ///
    /// # Panics
    /// If a record has more than 65,535 timestamps, which no artifact has.
    pub fn events(&self) -> impl Iterator<Item = Event<'_>> {
        self.times
            .iter()
            .enumerate()
            .map(move |(ordinal, time)| Event {
                id: EventId {
                    record: self.id(),
                    ordinal: u16::try_from(ordinal)
                        .expect("fewer than 65,536 timestamps per record"),
                },
                time,
                record: self,
            })
    }
}

#[cfg(test)]
mod tests {
    use crate::{EvidenceId, Locator, Namespace, ParserInfo, Record, RecordTime, TimeKind, Ts};

    #[test]
    fn one_event_per_timestamp() {
        let mut record = Record::new(
            EvidenceId::of_content(b"x"),
            Namespace::new("windows.prefetch"),
            Locator::ByteOffset(0),
            ParserInfo {
                name: "prefetch",
                version: "0.1.0",
            },
        );
        for run in 0..3 {
            let ts = Ts::from_unix_seconds(1_000 + run);
            record.times.push(RecordTime::new(
                TimeKind::Executed,
                format!("last_run[{run}]"),
                ts,
            ));
        }
        let events: Vec<_> = record.events().collect();
        assert_eq!(events.len(), 3);
        assert_eq!(events[2].id.ordinal, 2);
        assert_eq!(events[2].id.record, record.id());
        assert_eq!(events[2].time.field, "last_run[2]");
    }
}
