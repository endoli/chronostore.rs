// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! # Chronostore
//!
//! Chronostore is a system for storing time series in memory.
//! Chronostore is intended for use wihh datasets where a
//! single time series has 100 million or fewer points.
//!
//! Chronostore intends to be fast at inserts, fast at queries,
//! and memory efficient.
//!
//! Once data has been collected from a primary source
//! such as profiling samplers or counters, program tracing,
//! hardware counters, or other sources of high frequency,
//! high precision data, it is often useful to have it in
//! a form that tools can work with for analyzing and
//! visualizing that data.
//!
//! What is Chronostore NOT?
//!
//! * It does not try to be a distributed system.
//! * It does not have failover.
//! * It doesn't run as a separate process out of the box.
//! * It doesn't even persist data to disk automatically.
//!
//! ## Implementation Status
//!
//! The initial implementation is quite naive and is just
//! here to get something working. Over time, the implementation
//! will evolve and become significantly more sophisticated.

#![warn(missing_docs)]
#![deny(trivial_numeric_casts,
        unsafe_code, unstable_features,
        unused_import_braces, unused_qualifications)]

mod chronology;
mod entry;
mod null_summary;
mod summary;

pub use self::chronology::Chronology;
pub use self::entry::Entry;
pub use self::null_summary::NullSummary;
pub use self::summary::Summary;

/// Direction to search for a value from a timestamp.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Direction {
    /// Search backward from the timestamp.
    Backward,

    /// Search forward from the timestamp.
    Forward,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        let mut v = Chronology::<f32, NullSummary<f32>>::new();
        v.insert_values(&[Entry::new(5, 2.0),
                          Entry::new(10, 3.0),
                          Entry::new(15, 4.0),
                          Entry::new(20, 5.0)]);
        assert_eq!(v.find_nearest_value(2, Direction::Forward),
                   Some(Entry::new(5, 2.0)));
        assert_eq!(v.find_nearest_value(12, Direction::Forward),
                   Some(Entry::new(15, 4.0)));
        assert_eq!(v.find_nearest_value(22, Direction::Forward), None);
    }
}
