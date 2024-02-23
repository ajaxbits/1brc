use std::collections::BTreeSet;

#[derive(Default, Clone)]
pub struct StationEntry {
    pub running_sum: i64, // Make sure to account for the four decimal places
    pub count: i64,
    pub entries: BTreeSet<i64>,
}

impl StationEntry {
    pub fn new(running_sum: i64, count: i64, entries: BTreeSet<i64>) -> Self {
        Self {
            running_sum,
            count,
            entries,
        }
    }
}