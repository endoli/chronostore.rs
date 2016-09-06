// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! # Chronostore
//!
//! Chronostore is a system for storing time series in memory.
//!
//! What is Chronostore NOT?
//!
//! * It does not try to be a distributed system.
//! * It does not have failover.
//! * It doesn't run as a separate process out of the box.
//! * It doesn't even persist data to disk automatically.
//!
//! So, what *is* Chronostore good for?
//!
//! When you need a smaller scale storage of data that is
//! timestamped, Chronostore is useful.
//!
//! Once data has been collected from a primary source
//! such as profiling samplers or counters, program tracing,
//! hardware counters, or other sources of high frequency,
//! high precision data, it is often useful to have it in
//! a form that tools can work with for analyzing and
//! visualizing that data.
//!
//! The initial implementation is quite naive and is just
//! here to get something working. Over time, the implementation
//! will evolve and become significantly more sophisticated.
//!
//! Chronostore must be fast at inserts, fast at queries,
//! and memory efficient.

#![warn(missing_docs)]
#![deny(trivial_numeric_casts,
        unsafe_code, unstable_features,
        unused_import_braces, unused_qualifications)]

mod vector_backed;

pub use self::vector_backed::VectorChronology;

/// Direction to search for a value from a timestamp.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Direction {
    /// Search backward from the timestamp.
    Backward,

    /// Search forward from the timestamp.
    Forward,
}

/// A paired timestamp and value.
#[derive(Debug, PartialEq)]
pub struct Entry<V> {
    /// Timestamp for which the value is valid.
    pub timestamp: u64,

    /// V to be recorded.
    pub value: V,
}

impl<V> Entry<V> {
    /// Create a new `Entry`.
    pub fn new(timestamp: u64, value: V) -> Self {
        Entry {
            timestamp: timestamp,
            value: value,
        }
    }
}

/// A stream of values over time for a single variable.
pub trait Chronology<V: Copy> {
    /// Find the nearest value in time.
    fn find_nearest_value(&self, timestamp: u64, direction: Direction) -> Option<Entry<V>>;

    /// Record a set of values and their timestamps.
    fn insert_values(&mut self, values: &[Entry<V>]);
}
