use std::ops::Deref;

use serde_derive::Serialize;

#[derive(Serialize, Debug, PartialEq, Copy, Clone)]
pub struct Record {
    pub timestamp: u64,
    pub value: f64,
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct RecordSet {
    sorted_records: Vec<Record>,
}

impl FromIterator<Record> for RecordSet {
    fn from_iter<I: IntoIterator<Item = Record>>(iter: I) -> Self {
        let mut records: Vec<Record> = iter.into_iter().collect();
        records.sort_by_key(|r| r.timestamp);
        RecordSet{sorted_records: records}
    }
}

impl Deref for RecordSet {
    type Target = Vec<Record>;
    fn deref(&self) -> &Self::Target {
        &self.sorted_records
    }
}
