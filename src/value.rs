use std::collections::BTreeMap;

use crate::Ts;

/// A record's native fields, ordered by name so output is deterministic.
pub type Fields = BTreeMap<String, Value>;

/// A typed field value.
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    /// No value.
    Null,
    /// A boolean.
    Bool(bool),
    /// A signed integer.
    Int(i64),
    /// An unsigned integer.
    UInt(u64),
    /// A floating-point number.
    Float(f64),
    /// Text.
    Text(String),
    /// Raw bytes.
    Bytes(Vec<u8>),
    /// A timestamp.
    Time(Ts),
    /// An ordered list.
    List(Vec<Value>),
    /// Named sub-fields.
    Map(Fields),
}

impl From<&str> for Value {
    fn from(s: &str) -> Self {
        Self::Text(s.to_owned())
    }
}

impl From<String> for Value {
    fn from(s: String) -> Self {
        Self::Text(s)
    }
}

impl From<u64> for Value {
    fn from(n: u64) -> Self {
        Self::UInt(n)
    }
}

impl From<i64> for Value {
    fn from(n: i64) -> Self {
        Self::Int(n)
    }
}

impl From<bool> for Value {
    fn from(b: bool) -> Self {
        Self::Bool(b)
    }
}

impl From<Ts> for Value {
    fn from(ts: Ts) -> Self {
        Self::Time(ts)
    }
}
