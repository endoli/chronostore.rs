// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use super::{Chronology, Direction, Entry};

/// A stream of values over time for a single variable.
pub struct VectorChronology<V: Copy> {
    timestamps: Vec<u64>,
    values: Vec<V>,
}

impl<V: Copy> VectorChronology<V> {
    /// Create a new `VectorChronology`.
    pub fn new() -> Self {
        VectorChronology {
            timestamps: vec![],
            values: vec![],
        }
    }
}

impl<V: Copy> Chronology<V> for VectorChronology<V> {
    fn find_nearest_value(&self, timestamp: u64, _direction: Direction) -> Option<Entry<V>> {
        self.timestamps
            .iter()
            .position(|&t| t > timestamp)
            .map(|idx| Entry::new(self.timestamps[idx], self.values[idx]))
    }

    fn insert_values(&mut self, values: &[Entry<V>]) {
        for v in values {
            self.timestamps.push(v.timestamp);
            self.values.push(v.value);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::{Chronology, Direction, Entry};
    use super::*;

    #[test]
    fn basics() {
        let mut v = VectorChronology::<f32>::new();
        v.insert_values(&[Entry::new(5, 2.0),
                          Entry::new(10, 3.0),
                          Entry::new(15, 4.0),
                          Entry::new(20, 5.0)]);
        assert_eq!(v.find_nearest_value(2, Direction::Forward),
                   Some(Entry::new(5, 2.0)));
    }
}
