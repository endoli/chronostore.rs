// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use super::{Direction, Entry};

/// A stream of values over time for a single variable.
pub struct Chronology<V: Copy> {
    timestamps: Vec<u64>,
    values: Vec<V>,
}

impl<V: Copy> Chronology<V> {
    /// Create a new `Chronology`.
    pub fn new() -> Self {
        Chronology {
            timestamps: vec![],
            values: vec![],
        }
    }

    /// Find the nearest value in time.
    pub fn find_nearest_value(&self, timestamp: u64, _direction: Direction) -> Option<Entry<V>> {
        self.timestamps
            .iter()
            .position(|&t| t > timestamp)
            .map(|idx| Entry::new(self.timestamps[idx], self.values[idx]))
    }

    /// Record a set of values and their timestamps.
    pub fn insert_values(&mut self, values: &[Entry<V>]) {
        for v in values {
            self.timestamps.push(v.timestamp);
            self.values.push(v.value);
        }
    }
}
