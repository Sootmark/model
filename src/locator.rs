/// Where a record physically lives inside its evidence file.
///
/// Locators are the basis of [`RecordId`](crate::RecordId)s, so they must
/// describe *position*, never parse order: a newer parser that recovers extra
/// records adds new locators without shifting existing ones.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Locator {
    /// A byte offset in the evidence file.
    ByteOffset(u64),
    /// An NTFS MFT entry and its sequence number.
    MftEntry {
        /// MFT entry number.
        entry: u64,
        /// Sequence number (distinguishes reuses of the same entry).
        sequence: u16,
    },
    /// A registry key, or one of its values.
    Registry {
        /// Full key path inside the hive.
        key: String,
        /// Value name, or `None` for the key itself.
        value: Option<String>,
    },
    /// A row of a table in a database file (ESE, `SQLite`).
    TableRow {
        /// Table name.
        table: String,
        /// Stable row identifier (rowid, record id).
        row: u64,
    },
    /// A line of a text log, counted from 1.
    Line(u64),
}

impl Locator {
    /// Append an unambiguous binary encoding: a variant tag, then fields,
    /// with strings length-prefixed so no two locators encode the same way.
    pub(crate) fn encode(&self, out: &mut Vec<u8>) {
        match self {
            Self::ByteOffset(offset) => {
                out.push(1);
                out.extend_from_slice(&offset.to_le_bytes());
            }
            Self::MftEntry { entry, sequence } => {
                out.push(2);
                out.extend_from_slice(&entry.to_le_bytes());
                out.extend_from_slice(&sequence.to_le_bytes());
            }
            Self::Registry { key, value } => {
                out.push(3);
                encode_str(key, out);
                match value {
                    Some(name) => {
                        out.push(1);
                        encode_str(name, out);
                    }
                    None => out.push(0),
                }
            }
            Self::TableRow { table, row } => {
                out.push(4);
                encode_str(table, out);
                out.extend_from_slice(&row.to_le_bytes());
            }
            Self::Line(line) => {
                out.push(5);
                out.extend_from_slice(&line.to_le_bytes());
            }
        }
    }
}

pub(crate) fn encode_str(s: &str, out: &mut Vec<u8>) {
    out.extend_from_slice(&(s.len() as u64).to_le_bytes());
    out.extend_from_slice(s.as_bytes());
}

#[cfg(test)]
mod tests {
    use super::*;

    fn encoded(locator: &Locator) -> Vec<u8> {
        let mut out = Vec::new();
        locator.encode(&mut out);
        out
    }

    #[test]
    fn variants_never_collide() {
        assert_ne!(encoded(&Locator::ByteOffset(5)), encoded(&Locator::Line(5)));
    }

    #[test]
    fn strings_are_length_prefixed() {
        let a = Locator::Registry {
            key: "ab".into(),
            value: Some("c".into()),
        };
        let b = Locator::Registry {
            key: "a".into(),
            value: Some("bc".into()),
        };
        assert_ne!(encoded(&a), encoded(&b));
    }

    #[test]
    fn key_and_its_default_value_differ() {
        let key = Locator::Registry {
            key: "k".into(),
            value: None,
        };
        let unnamed_value = Locator::Registry {
            key: "k".into(),
            value: Some(String::new()),
        };
        assert_ne!(encoded(&key), encoded(&unnamed_value));
    }
}
