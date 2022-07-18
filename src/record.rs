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

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_record_set() {

        let unsorted = [
            Record{timestamp: 1, value: 1.0},
            Record{timestamp: 4, value: 4.0},
            Record{timestamp: 2, value: 2.0},
            Record{timestamp: 5, value: 5.0},
            Record{timestamp: 3, value: 3.0},
        ];

        let expected = [
            Record{timestamp: 1, value: 1.0},
            Record{timestamp: 2, value: 2.0},
            Record{timestamp: 3, value: 3.0},
            Record{timestamp: 4, value: 4.0},
            Record{timestamp: 5, value: 5.0},
        ];
        let actual: RecordSet = unsorted.into_iter().collect();

        assert_eq!(expected.as_slice(), actual.deref());
    }
}
