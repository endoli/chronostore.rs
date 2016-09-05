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

/// Direction to search for a value from a timestamp.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Direction {
    /// Search backwards from the timestamp.
    Backwards,

    /// Search forwards from the timestamp.
    Forwards,
}

/// A paired timestamp and value.
pub struct Entry<Value> {
    /// Timestamp for which the value is valid.
    pub timestamp: u64,

    /// Value to be recorded.
    pub value: Value,
}

/// A stream of values over time for a single variable.
pub trait Chronology<Value> {
    /// Find the nearest value in time.
    fn find_nearest_value(&self, timestamp: u64, direction: Direction) -> &Entry<Value>;

    /// Record a set of values and their timestamps.
    fn insert_values(&mut self, values: &[Entry<Value>]);
}
