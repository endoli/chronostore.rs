// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use super::Entry;

/// Pluggable means of maintaining summary information about the
/// data stored in a `Chronology`.
pub trait Summary<V> {
    /// Update the summary with a batch of entries.
    ///
    /// Some summary implementations may be able to operate
    /// more efficiently in batch form rather than updating
    /// over and over for each individual value.
    fn batch_update(&mut self, entry: &[Entry<V>]);

    /// Update the summary with a single new entry.
    fn update(&mut self, entry: &Entry<V>);
}
